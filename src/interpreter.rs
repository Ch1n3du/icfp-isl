use std::{collections::HashMap, panic::Location};

use crate::{
    ast::{within, BlockId, Color, Move, Orientation, Point},
    token::Position,
};

pub struct Interpreter {
    blocks: HashMap<BlockId, BlockData>,
    counter: u64,
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Interpreter {
    pub fn interpret(&mut self, moves: &[Move]) -> Result<u64, InterpreterError> {
        let mut sum = 0;
        let canvas_size = (self.height * self.width) as u64;

        for move_ in moves.iter().cloned() {
            match self.execute(move_) {
                Ok((base_cost, block_size)) => {
                    let cost = base_cost * canvas_size / block_size;
                    sum += cost;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(sum)
    }
    fn execute(&mut self, m: Move) -> InterpreterResult<(u64, u64)> {
        match m {
            Move::LCut {
                block_id,
                orientation,
                line_no,
                position,
            } => {
                let parent = self.blocks.get(&block_id).unwrap();
                let size = parent.size();
                let bounds = (parent.tl, parent.br);
                let n = line_no;

                let mut zero: BlockData = BlockData {
                    tl: Point::new(0, 0),
                    tr: Point::new(0, 0),
                    bl: Point::new(0, 0),
                    br: Point::new(0, 0),
                };
                let mut one: BlockData = BlockData {
                    tl: Point::new(0, 0),
                    tr: Point::new(0, 0),
                    bl: Point::new(0, 0),
                    br: Point::new(0, 0),
                };

                match orientation {
                    Orientation::Vertical => {
                        let mut bisector = (parent.tl, parent.bl);
                        bisector.0 = bisector.0.move_right(n);
                        bisector.1 = bisector.1.move_right(n);

                        if !(bisector.0.within(bounds) && bisector.1.within(bounds)) {
                            return Err(InterpreterError::OutOfBounds(
                                bisector.0, bounds, position,
                            ));
                        }

                        zero = BlockData {
                            tl: parent.tl,
                            bl: parent.bl,
                            tr: bisector.0,
                            br: bisector.1,
                        };

                        one = BlockData {
                            tl: bisector.0,
                            bl: bisector.1,
                            tr: parent.tr,
                            br: parent.br,
                        };
                    }
                    Orientation::Horizontal => {
                        let mut bisector = (parent.tl, parent.tr);
                        bisector.0 = bisector.0.move_down(n);
                        bisector.1 = bisector.1.move_down(n);

                        zero = BlockData {
                            tl: parent.tl,
                            bl: bisector.0,
                            tr: parent.tr,
                            br: bisector.1,
                        };
                    }
                }

                self.blocks.insert(
                    BlockId {
                        prev: Some(Box::new(block_id.clone())),
                        id: 0,
                    },
                    zero,
                );

                self.blocks.insert(
                    BlockId {
                        prev: Some(Box::new(block_id.clone())),
                        id: 1,
                    },
                    one,
                );
                self.blocks.remove(&block_id);
                Ok((7, size))
            }
            Move::PCut {
                block_id,
                point,
                position,
            } => {
                let parent = self.get_block(&block_id, &position)?;
                let bounds = (parent.tl, parent.br);
                if !within((point.x, point.y), bounds) {
                    return Err(InterpreterError::OutOfBounds(
                        point,
                        bounds,
                        position.to_owned(),
                    ));
                }

                // p1, p2, p3
                // p4, p5, p6
                // p7, p8, p9
                let p1 = parent.tl;
                let p2 = parent.tl.move_right(point.x - parent.tl.x);
                let p3 = parent.tr;
                let p4 = Point::new(parent.tl.y + point.y, parent.tl.x);
                let p5 = point;
                let p6 = Point::new(parent.tr.x, point.y);
                let p7 = parent.bl;
                let p8 = Point::new(point.x, parent.br.y);
                let p9 = parent.br;

                let q3_id = BlockId {
                    prev: Some(Box::new(block_id.clone())),
                    id: 3,
                };
                let q3 = BlockData {
                    tl: p1,
                    tr: p2,
                    bl: p4,
                    br: p5,
                };

                let q2_id = BlockId {
                    prev: Some(Box::new(block_id.clone())),
                    id: 2,
                };
                let q2 = BlockData {
                    tl: p2,
                    tr: p3,
                    bl: p5,
                    br: p6,
                };

                let q1_id = BlockId {
                    prev: Some(Box::new(block_id.clone())),
                    id: 1,
                };
                let q1 = BlockData {
                    tl: p5,
                    tr: p6,
                    bl: p8,
                    br: p9,
                };

                let q0_id = BlockId {
                    prev: Some(Box::new(block_id.clone())),
                    id: 0,
                };
                let q0 = BlockData {
                    tl: p4,
                    tr: p5,
                    bl: p7,
                    br: p8,
                };

                self.blocks.remove(&block_id);
                self.set_block(q0_id, q0);
                self.set_block(q1_id, q1);
                self.set_block(q2_id, q2);
                self.set_block(q3_id, q3);

                Ok((10, parent.size()))
            }
            Move::Color {
                block_id,
                color,
                position,
            } => {
                let block = self.get_block(&block_id, &position)?;
                let block = block.clone();
                let size = block.size();
                self.color_block(block.clone(), color);
                self.blocks.insert(block_id, block);

                Ok((5, size))
            }
            Move::Swap {
                block_id_1,
                block_id_2,
                position,
            } => {
                let block_1 = self.get_block(&block_id_1, &position)?;
                let block_2 = self.get_block(&block_id_2, &position)?;

                if !block_1.same_shape(&block_2) {
                    return Err(InterpreterError::NotTheSameSize(
                        block_id_1, block_id_2, position,
                    ));
                }

                self.blocks.insert(block_id_1, block_2.to_owned());
                self.blocks.insert(block_id_2, block_1.to_owned());
                Ok((3, block_1.size() + block_2.size()))
            }
            Move::Merge {
                block_id_1,
                block_id_2,
                position,
            } => {
                let block_1 = self.get_block(&block_id_1, &position)?;
                let block_2 = self.get_block(&block_id_2, &position)?;

                if !block_1.same_shape(&block_2) {
                    return Err(InterpreterError::NotTheSameSize(
                        block_id_1, block_id_2, position,
                    ));
                } else if !block_1.adjoint(&block_2) {
                    return Err(InterpreterError::NotAdjoint(
                        block_id_1, block_id_2, position,
                    ));
                }

                let new_block_id = BlockId {
                    prev: None,
                    id: self.counter,
                };

                let new_block = block_1.join(&block_2);

                self.blocks.remove(&block_id_1);
                self.blocks.remove(&block_id_2);
                self.blocks.insert(new_block_id, new_block);

                Ok((1, block_1.size() + block_2.size()))
            }
        }
    }

    fn color_block(&mut self, block: BlockData, color: Color) {
        let BlockData { tl, tr, bl, .. } = block;
        for x in tl.x..tr.x {
            for y in tl.y..bl.y {
                self.color_pixel(x, y, color)
            }
        }
    }

    fn color_pixel(&mut self, x: u64, y: u64, color: Color) {
        let x = x as usize;
        let y = y as usize;

        let index = x + (y * self.width);
        self.pixels[index] = color;
    }

    fn get_block(
        &mut self,
        block_id: &BlockId,
        position: &Position,
    ) -> InterpreterResult<BlockData> {
        if let Some(block) = self.blocks.get(&block_id) {
            Ok(block.clone())
        } else {
            Err(InterpreterError::BlockNonExistent(
                block_id.clone(),
                position.to_owned(),
            ))
        }
    }

    fn set_block(&mut self, block_id: BlockId, data: BlockData) {
        self.blocks.insert(block_id, data);
    }
}

#[derive(Debug, Clone)]
pub struct BlockData {
    tl: Point,
    tr: Point,
    bl: Point,
    br: Point,
}

impl BlockData {
    pub fn size(&self) -> u64 {
        (self.tl.y - self.bl.y) * (self.tr.x - self.tl.x)
    }

    pub fn same_shape(&self, rhs: &BlockData) -> bool {
        self.size() == rhs.size()
    }

    pub fn adjoint(&self, rhs: &BlockData) -> bool {
        let own = [self.tl, self.tr, self.br, self.bl];
        let other = [rhs.tl, rhs.tr, rhs.br, rhs.bl];

        let mut common = 0;

        for point in own {
            for point_ in other {
                if point == point_ {
                    common += 1;
                }
            }
        }

        common > 1
    }

    pub fn join(&self, rhs: &BlockData) -> BlockData {
        let tl = if self.tl.x < rhs.tl.x {
            self.tl
        } else {
            rhs.tl
        };

        let tr = if self.tr.x > rhs.tr.x {
            self.tr
        } else {
            rhs.tr
        };

        let bl = if self.bl.x < rhs.bl.x {
            self.bl
        } else {
            rhs.bl
        };

        let br = if self.br.x > rhs.br.x {
            self.br
        } else {
            rhs.br
        };

        BlockData { tl, tr, bl, br }
    }
}

pub enum InterpreterError {
    OutOfBounds(Point, (Point, Point), Position),
    BlockNonExistent(BlockId, Position),
    NotTheSameSize(BlockId, BlockId, Position),
    NotAdjoint(BlockId, BlockId, Position),
}

pub type InterpreterResult<T> = Result<T, InterpreterError>;

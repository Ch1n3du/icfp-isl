use crate::token::Position;

#[derive(Debug, Clone)]
pub enum Move {
    PCut {
        block_id: BlockId,
        point: Point,
        position: Position,
    },
    LCut {
        block_id: BlockId,
        orientation: Orientation,
        line_no: u64,
        position: Position,
    },
    Color {
        block_id: BlockId,
        color: Color,
        position: Position,
    },
    Swap {
        block_id_1: BlockId,
        block_id_2: BlockId,
        position: Position,
    },
    Merge {
        block_id_1: BlockId,
        block_id_2: BlockId,
        position: Position,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}

impl Point {
    pub fn new(x: u64, y: u64) -> Point {
        Point { x, y }
    }
    pub fn move_left(&self, n: u64) -> Point {
        let (x, y) = (self.x - n, self.y);
        Point::new(x, y)
    }

    pub fn move_right(&self, n: u64) -> Point {
        let (x, y) = (self.x + n, self.y);
        Point::new(x, y)
    }

    pub fn move_down(&self, n: u64) -> Point {
        let (x, y) = (self.x, self.y - n);
        Point::new(x, y)
    }

    pub fn move_up(&self, n: u64) -> Point {
        let (x, y) = (self.x, self.y + n);
        Point::new(x, y)
    }

    pub fn within(&self, bounds: (Point, Point)) -> bool {
        let (x, y) = (self.x, self.y);
        let (tl, br) = bounds;

        ((x > tl.x) && (x < br.x)) && ((y < tl.y) && (y > br.y))
    }
}

pub fn within(pos: (u64, u64), bounds: (Point, Point)) -> bool {
    let (x, y) = pos;
    let (tl, br) = bounds;

    ((x > tl.x) && (x < br.x)) && ((y < tl.y) && (y > br.y))
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BlockId {
    pub prev: Option<Box<BlockId>>,
    pub id: u64,
}

#[derive(Debug, Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

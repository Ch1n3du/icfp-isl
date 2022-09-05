use crate::token::Position;

#[derive(Debug)]
pub enum Move {
    PCut {
        block: BlockId,
        point: Point,
        position: Position,
    },
    LCut {
        block: BlockId,
        orientation: Orientation,
        line_no: u64,
        position: Position,
    },
    Color {
        block: BlockId,
        color: Color,
        position: Position,
    },
    Swap {
        block_1: BlockId,
        block_2: BlockId,
        position: Position,
    },
    Merge {
        block_1: BlockId,
        block_2: BlockId,
        position: Position,
    },
}

#[derive(Debug)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

#[derive(Debug)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}

#[derive(Debug)]
pub struct BlockId {
    pub prev: Option<Box<BlockId>>,
    pub id: u64,
}

#[derive(Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

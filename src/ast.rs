pub enum Move {
    PCut {
        block: BlockId,
        point: Point,
    },
    LCut {
        block: BlockId,
        orientation: Orientation,
        line_no: u64,
    },
    Color {
        block: BlockId,
        color: Color,
    },
}

pub struct Color(u8, u8, u8, u8);

pub struct Point {
    x: u64,
    y: u64,
}

pub struct BlockId {
    prev: Option<Box<BlockId>>,
    id: u64,
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

use crate::token::Position;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum Move {
    #[error("cut {block_id} {point}")]
    PCut {
        block_id: BlockId,
        point: Point,
        position: Position,
    },
    #[error("cut {block_id} {orientation} [{line_no}]")]
    LCut {
        block_id: BlockId,
        orientation: Orientation,
        line_no: u64,
        position: Position,
    },
    #[error("color {block_id} {color}")]
    Color {
        block_id: BlockId,
        color: Color,
        position: Position,
    },
    #[error("swap {block_id_1} {block_id_2}")]
    Swap {
        block_id_1: BlockId,
        block_id_2: BlockId,
        position: Position,
    },
    #[error("merge {block_id_1} {block_id_2}")]
    Merge {
        block_id_1: BlockId,
        block_id_2: BlockId,
        position: Position,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Color(r, g, b, a) = self;
        write!(f, "[{}, {}, {}, {}]", r, g, b, a)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[x: {}, y:{}]", self.x, self.y)
    }
}

impl Point {
    pub fn new(x: u64, y: u64) -> Point {
        Point { x, y }
    }

    pub fn set_x(&self, x: u64) -> Point {
        Point { x, y: self.y }
    }

    pub fn set_y(&self, y: u64) -> Point {
        Point { x: self.x, y }
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

impl BlockId {
    pub fn new(id: u64) -> BlockId {
        BlockId { prev: None, id }
    }

    pub fn new_with_prev(prev: &BlockId, id: u64) -> BlockId {
        BlockId {
            prev: Some(Box::new(prev.clone())),
            id,
        }
    }
}

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut base = format!("{}", self.id);
        let mut curr = self.prev.clone();

        while let Some(block_id) = curr {
            base = format!("{}.{}", block_id.id, base);
            curr = block_id.prev;
        }

        write!(f, "[{}]", base)
    }
}

#[derive(Debug, Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Orientation::*;
        match self {
            Vertical => write!(f, "[X]"),
            Horizontal => write!(f, "[Y]"),
        }
    }
}

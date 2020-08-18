pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct BlockShape {
    pub points: Vec<Point>,
}

impl BlockShape {
    ///Takes tuples of (x, y)
    pub fn new(points: Vec<Point>) -> BlockShape {
        BlockShape { points }
    }
}

pub struct BlockInfo {
    shapes: Vec<BlockShape>,
    current_shape: usize,
}

impl BlockInfo {
    pub fn new(shapes: Vec<BlockShape>) -> BlockInfo {
        BlockInfo {
            current_shape: 0,
            shapes,
        }
    }

    pub fn current_shape(&self)  -> &BlockShape {
        &self.shapes[self.current_shape]
    }

    pub fn peek_rotate(&self) -> &BlockShape {
        let next = (self.current_shape + 1) % self.shapes.len();

        &self.shapes[next]
    }

    pub fn rotate(&mut self) -> &BlockShape {
        self.current_shape = (self.current_shape + 1) % self.shapes.len();

        &self.shapes[self.current_shape]
    }
}

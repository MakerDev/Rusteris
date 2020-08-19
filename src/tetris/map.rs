use super::block::*;
//This is abstract version of real map
pub struct Map {
    map: Vec<BlockType>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut map = Map {
            map: Vec::<BlockType>::with_capacity(width * height),
            width,
            height,
        };

        for _ in 0..width * height {
            map.map.push(BlockType::None);
        }

        map.draw_border();

        map
    }

    pub fn get_at(&self, x: usize, y: usize) -> Option<&BlockType> {
        if self.check_boundary(x, y) {
            return Some(&self.map[y * self.width + x]);
        }

        None
    }

    pub fn set_at(&mut self, x: usize, y: usize, new_block: BlockType) {
        if self.check_boundary(x, y) {
            self.map[y * self.width + x] = new_block;
        } else {
            panic!("out of bound");
        }
    }

    pub fn draw_block(&mut self, block: &Block) {
        let (current_x, current_y) = block.get_current_position();

        for point in &block.get_shape().points {
            self.set_at(
                point.x + current_x,
                point.y + current_y,
                BlockType::ArrivedBlock,
            );
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    //x,y are already greater than 0 as they are usize
    fn check_boundary(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    ///Draw border
    fn draw_border(&mut self) {
        //side borders
        for i in 0..self.height - 1 {
            self.set_at(0, i, BlockType::Border);
            self.set_at(self.width - 1, i, BlockType::Border);

            for j in 1..self.width - 1 {
                self.set_at(j, i, BlockType::None);
            }
        }

        //bottom border
        for i in 0..self.width {
            self.set_at(i, self.height - 1, BlockType::Border);
        }
    }
}

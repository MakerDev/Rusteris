use super::blocks::*;
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

        map.init();

        map
    }

    pub fn get_at(&self, x: usize, y: usize) -> Option<&BlockType> {
        if self.check_boundary(x, y) {
            return Some(&self.map[y * self.height + x]);
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

    //x,y are already greater than 0 as they are usize
    fn check_boundary(&self, x: usize, y: usize) -> bool {
         x < self.width && y < self.height 
    }

    ///Draw border
    fn init(&mut self) {
        for i in 0..self.width {
            self.set_at(i, 0, BlockType::Border);
        }

        for i in 1..self.height - 1 {
            self.set_at(0, i, BlockType::Border);
            self.set_at(self.width - 1, i, BlockType::Border);

            for j in 1..self.width - 1 {
                self.set_at(j, i, BlockType::None);
            }
        }

        for i in 0..self.width {
            self.set_at(i, self.height - 1, BlockType::Border);
        }
    }
}

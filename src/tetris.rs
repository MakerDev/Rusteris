use block::*;
use map::*;
use minifb::Key;
use std::time::Instant;

mod block;
mod map;
pub mod renderer;

pub struct Game {
    map: Map,
    width: usize,
    height: usize,
    drop_rate: f64,
    falling_block: Option<Block>,
    last_drop: Instant,
    gameover: bool,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            map: Map::new(width, height),
            width,
            height,
            falling_block: None,
            drop_rate: 1.0,
            last_drop: Instant::now(),
            gameover: false,            
        }
    }

    pub fn update(&mut self, key_input: Option<&Key>) {
        if let Some(key) = key_input {
            //TODO : move block
        }

        //Execute here every 'drop_rate' secs
        if self.last_drop.elapsed().as_secs_f64() >= self.drop_rate {
            match self.falling_block {
                Some(ref mut block) => {
                    if block.can_drop(&self.map) {
                        block.drop();
                    } else {
                        
                    }
                }
                None => {
                    let new_block = Block::new_with_random_shape();
                    
                    if new_block.has_collision(&self.map) {
                        self.gameover = true;
                        return;
                    }

                    self.falling_block = Some(new_block);
                }
            }

            self.last_drop = Instant::now();
        }
    }

    //Draw abstract map to real world
    pub fn render(&self) -> Vec<u32> {
        renderer::render_map(&self)
    }
}

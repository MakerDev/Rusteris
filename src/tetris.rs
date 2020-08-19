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
            drop_rate: 0.2,
            last_drop: Instant::now(),
            gameover: false,
        }
    }

    pub fn gameover(&self) -> bool {
        self.gameover
    }

    pub fn update(&mut self, key_input: Option<&Key>) {
        self.process_key(key_input);

        //TODO : if player rotated or moved something reset timer?
        //Executes after every drop_rate
        if self.last_drop.elapsed().as_secs_f64() >= self.drop_rate {
            match self.falling_block {
                Some(ref mut block) => {
                    if block.can_drop(&self.map) {
                        block.drop();
                    } else {
                        &self.map.draw_block(block);
                        self.falling_block = None;
                    }

                    //TODO : Check gameover
                }
                None => {
                    let block = Block::new_with_random_shape();

                    //FIXME : This is temp gameover
                    // if block.has_collision(&self.map) {
                    //     self.gameover = true;
                    //     return;
                    // }

                    self.falling_block = Some(block);
                }
            }

            self.last_drop = Instant::now();
        }
    }

    //TODO : refactor this rendering mechanism
    //Draw abstract map to real world
    pub fn render(&self) -> Vec<u32> {
        renderer::render_game(self)
    }

    fn process_key(&mut self, key_input: Option<&Key>) {
        if let Some(key) = key_input {
            if key == &Key::Left {
                self.move_if_possible(&Direction::Left);
            } else if key == &Key::Right {
                self.move_if_possible(&Direction::Right);
            } else if key == &Key::Down {
                self.move_if_possible(&Direction::Down);
            } else if key == &Key::Up {
                self.rotate_if_possible();
            } else if key == &Key::Space {
                //TODO : drop to bottom immediately
            }
        }
    }

    fn move_if_possible(&mut self, direction: &Direction) {
        if let Some(ref mut block) = self.falling_block {
            if block.can_move_to(&self.map, direction) {
                block.move_to(direction);
            }
        }
    }

    fn rotate_if_possible(&mut self) {
        if let Some(ref mut block) = self.falling_block {
            if block.can_rotate(&self.map) {
                block.rotate();
            }
        }
    }
}

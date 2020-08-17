use map::*;
use minifb::{Key, Window, WindowOptions};
use blocks::*;

mod map;
mod blocks;
pub mod renderer;

pub struct Game {
    map: Map,
    width: usize,
    height: usize,    
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            map : Map::new(width, height),
            width,
            height,
        }
    }

    pub fn run(&mut self) {

    }

    pub fn update(&mut self, key_input: Option<&Key>) {

    }

    //Draw abstract map to real world
    pub fn render(&self) -> Vec<u32> {
        Vec::new()
    }
}
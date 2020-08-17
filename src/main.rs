use crate::tetris::renderer::BLOCK_RENDER_HEIGHT;
use crate::tetris::renderer::BLOCK_RENDER_WIDTH;

use minifb::{Key, Window, WindowOptions, KeyRepeat};
use tetris::*;

const GAME_WIDTH: usize = 12;
const GAME_HEIGHT: usize = 20;

const WIDTH: usize = 12 * BLOCK_RENDER_WIDTH;
const HEIGHT: usize = 20 * BLOCK_RENDER_HEIGHT;

mod tetris;

fn main() {
    let mut game = Game::new(GAME_WIDTH, GAME_HEIGHT);

    game.run();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });


    // Limit to max ~60 fps update rate
    //window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    window.limit_update_rate(Some(std::time::Duration::from_micros(22400)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 255 | 255 << 8 | 255 << 16; // write something more funny here!
        }

        let key_input = window.get_keys_pressed(KeyRepeat::Yes).unwrap();
        let key_input = key_input.last();

        println!("{:?}", key_input);

        game.update(key_input);
        let buf = game.render();

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
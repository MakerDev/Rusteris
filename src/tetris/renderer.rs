use super::{block::BlockType, Game};
use crate::tetris::Map;

pub const BLOCK_RENDER_WIDTH: usize = 16;
pub const BLOCK_RENDER_HEIGHT: usize = 16;

const _RED: u32 = 255 << 16 | 0;
const GREEN: u32 = 255 << 8 | 0;
const BLUE: u32 = 255;
const BLACK: u32 = 0;
const WHITE: u32 = 255 << 16 | 255 << 8 | 255;

pub fn render_game(game: &Game) -> Vec<u32> {
    let map_size = game.map.get_size();

    //TODO : Cache this. Not always spawning new one
    let mut buffer: Vec<u32> =
        vec![0; game.width * BLOCK_RENDER_HEIGHT * game.height * BLOCK_RENDER_WIDTH];

    render_map(&mut buffer, &game.map);

    if let Some(ref falling_block) = game.falling_block {
        for point in &falling_block.get_shape_at_current_position().points {
            render_block(
                &mut buffer,
                map_size,
                &BlockType::NewBlock,
                (point.x, point.y),
            );
        }
    }

    buffer
}

//TODO : improve renderer
fn render_map(buffer: &mut Vec<u32>, map: &Map) {
    let (width, height) = map.get_size();

    //Draw map without falling_block
    for y in 0..height {
        for x in 0..width {
            let block_type = map.get_at(x, y).unwrap();

            render_block(buffer, (width, height), block_type, (x, y));
        }
    }
}

fn render_block(
    buffer: &mut Vec<u32>,
    map_size: (usize, usize),
    block_type: &BlockType,
    at: (usize, usize),
) {
    let color;

    match block_type {
        BlockType::None => {
            color = WHITE;
        }
        BlockType::Border => {
            color = BLACK;
        }
        BlockType::NewBlock => {
            color = GREEN;
        }
        BlockType::ArrivedBlock => {
            color = BLUE;
        }
    }

    let (x, y) = (at.0 * BLOCK_RENDER_WIDTH, at.1 * BLOCK_RENDER_HEIGHT);
    let (width, _) = (
        map_size.0 * BLOCK_RENDER_WIDTH,
        map_size.1 * BLOCK_RENDER_HEIGHT,
    );

    for i in y..(y + BLOCK_RENDER_HEIGHT) {
        for j in x..(x + BLOCK_RENDER_WIDTH) {
            buffer[i * width + j] = color;
        }
    }
}

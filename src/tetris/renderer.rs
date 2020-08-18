use super::{Game, block::BlockType};

pub const BLOCK_RENDER_WIDTH: usize = 16;
pub const BLOCK_RENDER_HEIGHT: usize = 16;

const _RED: u32 = 255 << 16 | 0;
const GREEN: u32 = 255 << 8 | 0;
const BLUE: u32 = 255;
const BLACK: u32 = 0;
const WHITE: u32 = 255 << 16 | 255 << 8 | 255;

//TODO : improve renderer
pub fn render_map(game: &Game) -> Vec<u32> {
    let map = &game.map;
    let (width, height) = (game.width, game.height);

    //TODO : Cache this. Not always spawning new one
    let mut buffer: Vec<u32> = vec![0; width*BLOCK_RENDER_HEIGHT*height*BLOCK_RENDER_WIDTH];

    for y in 0..height {
        for x in 0..width {
            let block_type = map.get_at(x, y).unwrap();

            render_block(&mut buffer, (width, height), block_type, (x, y));
        }
    }

    buffer
}

fn render_block(
    buffer: &mut Vec<u32>,
    map_size: (usize, usize),
    block_type: &BlockType,
    at: (usize, usize),
) {
    let mut color: u32 = WHITE;

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

    let (x, y) = (at.0 * BLOCK_RENDER_WIDTH, at.1*BLOCK_RENDER_HEIGHT);
    let (width, height) = (map_size.0 * BLOCK_RENDER_WIDTH, map_size.1 * BLOCK_RENDER_HEIGHT);

    for i in y..(y+BLOCK_RENDER_HEIGHT) {
        for j in x..(x+BLOCK_RENDER_WIDTH) {
            buffer[i * width + j] = color;
        }
    }
}

use super::map::Map;
use blockinfo::*;
use rand::random;

mod blockinfo;

#[derive(Clone, Copy, PartialEq)]
pub enum BlockType {
    None,
    Border,
    NewBlock,
    ArrivedBlock, //Represents a block which is already on the ground
}

pub enum Direction {
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn move_amount(&self) -> (i32, i32) {
        match self {
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

pub enum BlockShapeType {
    Straight,
    Type1,
    Type2,
}

impl BlockShapeType {
    pub fn get_random() -> BlockShapeType {
        let num: u8 = random::<u8>() % 3;

        if num == 0 {
            return BlockShapeType::Straight;
        } else if num == 1 {
            return BlockShapeType::Type1;
        }

        BlockShapeType::Type2
    }
}

struct ShapeGenerator {}

impl ShapeGenerator {
    pub fn generate(shape_type: &BlockShapeType) -> Vec<BlockShape> {
        //ㅁㅁㅁㅁ of 90 degree rotated
        match shape_type {
            BlockShapeType::Straight => ShapeGenerator::generate_straight(),
            BlockShapeType::Type1 => ShapeGenerator::generate_type1(),
            BlockShapeType::Type2 => ShapeGenerator::generate_type2(),
        }
    }

    fn generate_straight() -> Vec<BlockShape> {
        let mut shape1 = Vec::new();
        shape1.push(Point { x: 0, y: 0 });
        shape1.push(Point { x: 0, y: 1 });
        shape1.push(Point { x: 0, y: 2 });
        shape1.push(Point { x: 0, y: 3 });

        let mut shape2 = Vec::new();
        shape2.push(Point { x: 0, y: 0 });
        shape2.push(Point { x: 1, y: 0 });
        shape2.push(Point { x: 2, y: 0 });
        shape2.push(Point { x: 3, y: 0 });

        let mut shapes: Vec<BlockShape> = Vec::new();
        shapes.push(BlockShape::new(shape1));
        shapes.push(BlockShape::new(shape2));

        shapes
    }

    fn generate_type1() -> Vec<BlockShape> {
        let mut shapes: Vec<BlockShape> = Vec::new();
        //ㅁ
        //ㅁ
        //ㅁㅁ
        let mut shape = Vec::new();
        shape.push(Point { x: 0, y: 0 });
        shape.push(Point { x: 0, y: 1 });
        shape.push(Point { x: 0, y: 2 });
        shape.push(Point { x: 1, y: 2 });

        shapes.push(BlockShape::new(shape));

        let mut shape = Vec::new();
        shape.push(Point { x: 0, y: 0 });
        shape.push(Point { x: 0, y: 1 });
        shape.push(Point { x: 1, y: 0 });
        shape.push(Point { x: 2, y: 0 });

        shapes.push(BlockShape::new(shape));

        let mut shape = Vec::new();
        shape.push(Point { x: 0, y: 0 });
        shape.push(Point { x: 1, y: 0 });
        shape.push(Point { x: 1, y: 1 });
        shape.push(Point { x: 1, y: 2 });

        shapes.push(BlockShape::new(shape));

        let mut shape = Vec::new();
        shape.push(Point { x: 0, y: 1 });
        shape.push(Point { x: 1, y: 1 });
        shape.push(Point { x: 2, y: 1 });
        shape.push(Point { x: 2, y: 0 });

        shapes.push(BlockShape::new(shape));

        shapes
    }

    fn generate_type2() -> Vec<BlockShape> {
        let mut shapes: Vec<BlockShape> = Vec::new();
        // ㅁ
        //ㅁㅁㅁ
        let mut shape = Vec::new();
        shape.push(Point { x: 1, y: 0 });
        shape.push(Point { x: 0, y: 1 });
        shape.push(Point { x: 1, y: 1 });
        shape.push(Point { x: 2, y: 1 });

        shapes.push(BlockShape::new(shape));

        let mut shape = Vec::new();
        shape.push(Point { x: 0, y: 0 });
        shape.push(Point { x: 0, y: 1 });
        shape.push(Point { x: 0, y: 2 });
        shape.push(Point { x: 1, y: 1 });

        shapes.push(BlockShape::new(shape));

        let mut shape = Vec::new();
        shape.push(Point { x: 0, y: 0 });
        shape.push(Point { x: 1, y: 0 });
        shape.push(Point { x: 2, y: 0 });
        shape.push(Point { x: 1, y: 1 });

        shapes.push(BlockShape::new(shape));

        let mut shape = Vec::new();
        shape.push(Point { x: 1, y: 0 });
        shape.push(Point { x: 1, y: 1 });
        shape.push(Point { x: 1, y: 2 });
        shape.push(Point { x: 0, y: 1 });

        shapes.push(BlockShape::new(shape));

        shapes
    }
}

//Change Block from trait to struct.
//Store block_info and current_position in it. However, make create_shape func to create a new shape.
//Not BlockStraight.. like constructor

pub struct Block {
    current_position: Point,
    block_info: BlockInfo,
}

impl Block {
    pub fn new_with_random_shape() -> Block {
        Block::new(&BlockShapeType::get_random())
    }

    pub fn new(shape_type: &BlockShapeType) -> Block {
        Block {
            current_position: Point { x: 6, y: 0 },
            block_info: BlockInfo::new(ShapeGenerator::generate(shape_type)),
        }
    }

    pub fn has_collision(&self, map: &Map) -> bool {
        for point in &self.get_shape_at_current_position().points {
            if map.get_at(point.x, point.y).unwrap() != &BlockType::None {
                return true;
            }
        }

        false
    }

    pub fn can_move_to(&self, map: &Map, direction: &Direction) -> bool {
        let (x_amount, y_amount) = direction.move_amount();

        for point in &self.get_shape_at_current_position().points {
            let x = (point.x as i32 + x_amount) as usize;
            let y = (point.y as i32 + y_amount) as usize;

            if let Some(block_type) = map.get_at(x, y) {
                if block_type != &BlockType::None {
                    return false;
                }
            }
        }

        true
    }

    pub fn move_to(&mut self, direction: &Direction) {
        let (x_amount, y_amount) = direction.move_amount();

        self.current_position.x = (self.current_position.x as i32 + x_amount) as usize;
        self.current_position.y = (self.current_position.y as i32 + y_amount) as usize;
    }

    pub fn can_drop(&self, map: &Map) -> bool {
        self.can_move_to(map, &Direction::Down)
    }

    pub fn drop(&mut self) {
        self.move_to(&Direction::Down);
    }

    pub fn can_rotate(&self, map: &Map) -> bool {
        let (current_x, current_y) = self.get_current_position();
        let rotated_shape = self.block_info.peek_rotate();

        for point in &rotated_shape.points {
            if let Some(block_type) = map.get_at(point.x + current_x, point.y + current_y) {
                if block_type != &BlockType::None {
                    return false;
                }
            }
        }

        true
    }

    pub fn rotate(&mut self) {
        self.block_info.rotate();
    }

    //TODO : Refactor this. Too many code duplication and error prone
    pub fn get_shape_at_current_position(&self) -> BlockShape {
        let mut block_shape = BlockShape { points: Vec::new() };
        let (current_x, current_y) = self.get_current_position();

        for point in &self.block_info.current_shape().points {
            block_shape.points.push(Point {
                x: point.x + current_x,
                y: point.y + current_y,
            });
        }

        block_shape
    }

    pub fn get_shape(&self) -> &BlockShape {
        self.block_info.current_shape()
    }

    pub fn get_current_position(&self) -> (usize, usize) {
        (self.current_position.x, self.current_position.y)
    }
}

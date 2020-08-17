#[derive(Clone, Copy)]
pub enum BlockType {
    None,
    Border,
    NewBlock,
    ArrivedBlock, //Represents a block which is already on the ground
}
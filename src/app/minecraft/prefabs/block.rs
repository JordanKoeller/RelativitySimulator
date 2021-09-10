use shapes::Block;

pub enum BlockType {
  Empty,
  Grass,
  Rock,
  Water,
  Wood,
  Leaf
}

pub struct MCBlock {
  block_type: BlockType
}
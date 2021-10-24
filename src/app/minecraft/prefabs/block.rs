use app::minecraft::ChunkComponent;
use shapes::{Block, TEXTURE_CUBE_INDICES, TEXTURE_CUBE_VERTICES};
use utils::{Vec3I, Vec3F};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
  Empty,
  Grass,
  Rock,
  Water,
  Wood,
  Leaf,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockFace {
  Left,
  Right,
  Top,
  Bottom,
  Front,
  Back,
}

impl BlockFace {
  pub fn buffer_info(self, translation: Vec3F, starting_index: u32) -> ([f32; 48], [u32; 6]) {
    let mut output_coords = [0f32; 48];
    let mut output_inds = [0u32; 6];
    match self {
      BlockFace::Right => {
        output_coords.clone_from_slice(&TEXTURE_CUBE_VERTICES[144..192]);
        output_inds.clone_from_slice(&TEXTURE_CUBE_INDICES[0..6]);
      },
      BlockFace::Left => {
        output_coords.clone_from_slice(&TEXTURE_CUBE_VERTICES[96..144]);
        output_inds.clone_from_slice(&TEXTURE_CUBE_INDICES[0..6]);
      },
      BlockFace::Front => {
        output_coords.clone_from_slice(&TEXTURE_CUBE_VERTICES[0..48]);
        output_inds.clone_from_slice(&TEXTURE_CUBE_INDICES[0..6]);
      },
      BlockFace::Back => {
        output_coords.clone_from_slice(&TEXTURE_CUBE_VERTICES[48..96]);
        output_inds.clone_from_slice(&TEXTURE_CUBE_INDICES[0..6]);
      },
      BlockFace::Bottom => {
        output_coords.clone_from_slice(&TEXTURE_CUBE_VERTICES[192..240]);
        output_inds.clone_from_slice(&TEXTURE_CUBE_INDICES[0..6]);
      },
      BlockFace::Top => {
        output_coords.clone_from_slice(&TEXTURE_CUBE_VERTICES[240..288]);
        output_inds.clone_from_slice(&TEXTURE_CUBE_INDICES[0..6]);
      },
    };
    for i in 0..6 {
      output_inds[i] += starting_index;
    }
    for i in 0..6 {
      let t_x = i * 8;
      output_coords[t_x] += translation.x;
      output_coords[t_x+1] += translation.y;
      output_coords[t_x+2] += translation.z;
    }
    (output_coords, output_inds)
  }

  pub fn faces_array() -> [BlockFace; 6] {
    [
      BlockFace::Left,
      BlockFace::Right,
      BlockFace::Front,
      BlockFace::Back,
      BlockFace::Top,
      BlockFace::Bottom,
    ]
  }

}

pub struct BlockDescriptor<'a> {
  index: Vec3I,
  parent_chunk: &'a ChunkComponent,
}

impl<'a> BlockDescriptor<'a> {
  pub fn new(index: Vec3I, parent_chunk: &'a ChunkComponent) -> Self {
    Self { index, parent_chunk }
  }

  pub fn block_type(&self) -> &BlockType {
    self.parent_chunk.get(&self.index.x, &self.index.y, &self.index.z)
  }

  pub fn world_coordinate(&self) -> Vec3F {
    self.parent_chunk.world_coordinate(&self.index.x, &self.index.y, &self.index.z).cast::<f32>().expect("Could not get world coord")
  }

  pub fn face_exposed(&self, face: BlockFace) -> bool {
    let is_empty = |x, y, z| !self.parent_chunk.valid_index(x, y, z) || self.parent_chunk.get(x, y, z) == &BlockType::Empty;
    match face {
      BlockFace::Left => {
        self.block_type() != &BlockType::Empty && is_empty(&(self.index.x - 1), &self.index.y, &self.index.z)
      }
      BlockFace::Right => {
        self.block_type() != &BlockType::Empty && is_empty(&(self.index.x + 1), &self.index.y, &self.index.z)
      }
      BlockFace::Front => {
        self.block_type() != &BlockType::Empty && is_empty(&self.index.x, &self.index.y, &(self.index.z - 1))
      }
      BlockFace::Back => {
        self.block_type() != &BlockType::Empty && is_empty(&self.index.x, &self.index.y, &(self.index.z + 1))
      }
      BlockFace::Top => {
        self.block_type() != &BlockType::Empty && is_empty(&self.index.x, &(self.index.y + 1), &self.index.z)
      }
      BlockFace::Bottom => {
        self.block_type() != &BlockType::Empty && is_empty(&self.index.x, &(self.index.y - 1), &self.index.z)
      }
    }
  }
}

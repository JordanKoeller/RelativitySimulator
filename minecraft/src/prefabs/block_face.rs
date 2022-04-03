use engine::shapes::{TEXTURE_CUBE_INDICES, TEXTURE_CUBE_VERTICES};

pub enum BlockFaceEnum {
    Left,
    Right,
    Top,
    Bottom,
    Front,
    Back,
}

impl BlockFaceEnum {
    pub fn buffer_info(self) -> (&'static [f32], &'static [u32]) {
        match self {
            BlockFaceEnum::Left => (&TEXTURE_CUBE_VERTICES[144..192], &TEXTURE_CUBE_INDICES[0..48]),
            BlockFaceEnum::Right => (&TEXTURE_CUBE_VERTICES[96..144], &TEXTURE_CUBE_INDICES[0..48]),
            BlockFaceEnum::Front => (&TEXTURE_CUBE_VERTICES[0..48], &TEXTURE_CUBE_INDICES[0..48]),
            BlockFaceEnum::Back => (&TEXTURE_CUBE_VERTICES[48..96], &TEXTURE_CUBE_INDICES[0..48]),
            BlockFaceEnum::Bottom => (&TEXTURE_CUBE_VERTICES[192..240], &TEXTURE_CUBE_INDICES[0..48]),
            BlockFaceEnum::Top => (&TEXTURE_CUBE_VERTICES[240..288], &TEXTURE_CUBE_INDICES[0..48]),
        }
    }
}

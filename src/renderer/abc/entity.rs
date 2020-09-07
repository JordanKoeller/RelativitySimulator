
use initializers::{AssetSpec, GLSpec};

pub trait Entity {
    fn mesh_spec(&self) -> GLSpec;
    fn asset_spec(&self) -> Option<AssetSpec> {
        None
    }
}
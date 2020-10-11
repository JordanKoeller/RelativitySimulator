
pub struct PhysicsEngineConfig {}

impl Default for PhysicsEngineConfig {
  fn default() -> Self {
    PhysicsEngineConfig {}
  }
}

pub struct PhysicsEngine {
  config: PhysicsEngineConfig,
}

impl PhysicsEngine {

}

impl Default for PhysicsEngine {
  fn default() -> Self {
    Self {
      config: PhysicsEngineConfig::default()
    }
  }
}


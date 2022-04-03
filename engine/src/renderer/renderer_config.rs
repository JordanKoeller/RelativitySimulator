#[derive(Debug, Clone)]
pub enum RelativityMode {
    CLASSICAL,
    LORENTZ,
    RELATIVISTIC,
}

impl RelativityMode {
    pub fn id(&self) -> i32 {
        match self {
            RelativityMode::CLASSICAL => 0,
            RelativityMode::LORENTZ => 1,
            RelativityMode::RELATIVISTIC => 2,
        }
    }

    pub fn rotate(self) -> Self {
        match self {
            RelativityMode::CLASSICAL => RelativityMode::LORENTZ,
            RelativityMode::LORENTZ => RelativityMode::RELATIVISTIC,
            RelativityMode::RELATIVISTIC => RelativityMode::CLASSICAL,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RendererConfig {
    pub mode: RelativityMode,
    pub debug: bool,
}

impl Default for RendererConfig {
    fn default() -> Self {
        RendererConfig {
            mode: RelativityMode::CLASSICAL,
            debug: false,
        }
    }
}

impl RendererConfig {
    pub fn new(mode: RelativityMode) -> Self {
        RendererConfig {
            mode: mode,
            debug: false,
        }
    }

    pub fn relativity_mode(&self) -> i32 {
        self.mode.id()
    }
}

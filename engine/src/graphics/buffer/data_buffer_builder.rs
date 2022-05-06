use super::{BufferConfig, BufferLayout, DataBuffer};

#[derive(Default)]
pub struct DataBufferBuilder {
    layout: Option<BufferLayout>,
    data: Vec<f32>,
    config: Option<BufferConfig>,
}

impl DataBufferBuilder {
    pub fn with_config(mut self, config: BufferConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn with_layout(mut self, layout: BufferLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    pub fn with_data(mut self, data: Vec<f32>) -> Self {
        self.data = data;
        self
    }

    pub fn build(self) -> DataBuffer {
        let buf = DataBuffer::new(self.data, self.layout.unwrap(), self.config.unwrap(), std::u32::MAX);
        buf
    }
}

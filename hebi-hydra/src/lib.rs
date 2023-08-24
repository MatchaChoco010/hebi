mod bridge;
mod hebi_hydra;

use crate::bridge::*;

pub trait RenderDelegate: Send + Sync {
    type RenderBuffer: RenderBuffer;
    fn get_supported_rprim_types(&self) -> Vec<String>;
    fn get_supported_sprim_types(&self) -> Vec<String>;
    fn get_supported_bprim_types(&self) -> Vec<String>;
    fn init(&self);
    fn destroy(&self);
    fn render(&self);
    fn create_render_buffer(&self, id: RenderBufferId) -> Self::RenderBuffer;
}

pub trait RenderBuffer: Send + Sync {
    fn allocate(&self, width: usize, height: usize, format: RenderBufferFormat);
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn get_format(&self) -> RenderBufferFormat;
    fn read(&self) -> Vec<u8>;
    fn write(&self, data: &[u8]);
    fn finalize(&self);
}
impl RenderBuffer for Box<dyn RenderBuffer + 'static> {
    fn allocate(&self, width: usize, height: usize, format: RenderBufferFormat) {
        self.as_ref().allocate(width, height, format)
    }
    fn get_width(&self) -> usize {
        self.as_ref().get_width()
    }
    fn get_height(&self) -> usize {
        self.as_ref().get_height()
    }
    fn get_format(&self) -> RenderBufferFormat {
        self.as_ref().get_format()
    }
    fn read(&self) -> Vec<u8> {
        self.as_ref().read()
    }
    fn write(&self, data: &[u8]) {
        self.as_ref().write(data)
    }
    fn finalize(&self) {
        self.as_ref().finalize()
    }
}

pub fn create_render_delegate() -> impl RenderDelegate + 'static {
    hebi_hydra::HebiRenderDelegate::new()
}

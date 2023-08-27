//! This Module implements Hydra's RenderDelegate to hebi.

use hydra_bridge;
use hydra_bridge::bridge::*;
use hydra_bridge::{RenderBuffer, RenderDelegate};
use parking_lot::{Mutex, RwLock};
use std::collections::HashMap;
use std::sync::Arc;

#[hydra_bridge::ctor]
fn init() {
    hydra_bridge::register_render_delegate_creator(create_render_delegate);
}

fn create_render_delegate() -> HebiRenderDelegate {
    HebiRenderDelegate::new()
}

#[derive(Debug)]
struct HebiRenderDelegateInner {
    buffers: HashMap<RenderBufferId, HebiRenderBuffer>,
}
#[derive(Debug)]
pub struct HebiRenderDelegate {
    inner: Arc<Mutex<HebiRenderDelegateInner>>,
}
impl HebiRenderDelegate {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HebiRenderDelegateInner {
                buffers: HashMap::new(),
            })),
        }
    }
}
impl RenderDelegate for HebiRenderDelegate {
    type RenderBuffer = HebiRenderBuffer;

    fn get_supported_rprim_types(&self) -> Vec<String> {
        vec!["mesh".to_string()]
    }

    fn get_supported_sprim_types(&self) -> Vec<String> {
        vec!["camera".to_string()]
    }

    fn get_supported_bprim_types(&self) -> Vec<String> {
        vec!["renderBuffer".to_string()]
    }

    fn init(&self) {
        println!("HebiRenderDelegate init");
    }

    fn destroy(&self) {
        println!("HebiRenderDelegate destroy");
    }

    fn render(&self) {
        println!("HebiRenderDelegate render");

        for (_id, buffer) in self.inner.lock().buffers.iter_mut() {
            let width = buffer.get_width();
            let height = buffer.get_height();
            let format = buffer.get_format();

            // 雑にRGBの色を緑で塗りつぶす
            if format == RenderBufferFormat::UNorm8Vec4 {
                let mut data = vec![0; width * height * format.component_size()];
                for i in 0..height {
                    for j in 0..width {
                        let index = (i * width + j) * format.component_size();
                        data[index + 0] = 0;
                        data[index + 1] = 255;
                        data[index + 2] = 0;
                        data[index + 3] = 255;
                    }
                }
                buffer.write(&data);
            }
        }
    }

    fn create_render_buffer(&self, id: RenderBufferId) -> Self::RenderBuffer {
        println!("Create render buffer! {id:?}");
        let render_buffer = HebiRenderBuffer::new();
        let mut inner = self.inner.lock();
        inner.buffers.insert(id, render_buffer.clone());
        render_buffer
    }
}

#[derive(Debug)]
struct HebiRenderBufferInner {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
    format: RenderBufferFormat,
}
#[derive(Debug, Clone)]
pub struct HebiRenderBuffer {
    inner: Arc<RwLock<HebiRenderBufferInner>>,
}
impl HebiRenderBuffer {
    fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HebiRenderBufferInner {
                buffer: Vec::new(),
                width: 0,
                height: 0,
                format: RenderBufferFormat::Invalid,
            })),
        }
    }
}
impl RenderBuffer for HebiRenderBuffer {
    fn allocate(&self, width: usize, height: usize, format: RenderBufferFormat) {
        println!("allocate {} {}", width, height);
        let buffer_size = width * height * format.component_size();
        let mut inner = self.inner.write();
        inner.buffer = vec![0; buffer_size];
        // todo 以前のバッファを保持する

        inner.width = width;
        inner.height = height;
        inner.format = format;
    }

    fn get_width(&self) -> usize {
        let inner = self.inner.read();
        inner.width
    }

    fn get_height(&self) -> usize {
        let inner = self.inner.read();
        inner.height
    }

    fn get_format(&self) -> RenderBufferFormat {
        let inner = self.inner.read();
        inner.format
    }

    fn read(&self) -> Vec<u8> {
        let inner = self.inner.read();
        inner.buffer.clone()
    }

    fn write(&self, data: &[u8]) {
        let mut inner = self.inner.write();
        inner.buffer.clone_from_slice(data);
    }

    fn finalize(&self) {
        println!("finalize render buffer");
    }
}

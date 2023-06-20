mod bridge;

pub fn create_render_delegate() -> impl bridge::RenderDelegate + 'static {
    HebiRenderDelegate::new()
}

pub struct HebiRenderDelegate {}
impl HebiRenderDelegate {
    fn new() -> Self {
        Self {}
    }
}
impl bridge::RenderDelegate for HebiRenderDelegate {
    type RenderBuffer = HebiRenderBuffer;
    fn get_supported_rprim_types(&self) -> Vec<String> {
        vec!["mesh".to_string()]
    }

    fn get_supported_sprim_types(&self) -> Vec<String> {
        vec![]
    }

    fn get_supported_bprim_types(&self) -> Vec<String> {
        vec![]
    }

    fn init(&self) {
        println!("HebiRenderDelegate init");
    }

    fn destroy(&self) {
        println!("HebiRenderDelegate destroy");
    }

    fn create_render_buffer(&self, id: bridge::RenderBufferId) -> HebiRenderBuffer {
        HebiRenderBuffer::new()
    }
}

pub struct HebiRenderBuffer {}
impl HebiRenderBuffer {
    fn new() -> Self {
        Self {}
    }
}
impl bridge::RenderBuffer for HebiRenderBuffer {
    fn allocate(&self, width: usize, height: usize, format: bridge::RenderBufferFormat) {
        todo!()
    }

    fn get_width(&self) -> usize {
        todo!()
    }

    fn get_height(&self) -> usize {
        todo!()
    }

    fn get_format(&self) -> bridge::RenderBufferFormat {
        todo!()
    }

    fn read(&self) -> Vec<u8> {
        todo!()
    }

    fn write(&self, data: &[u8]) {
        todo!()
    }

    fn finalize(&self) {
        todo!()
    }
}

#[cxx::bridge]
pub mod ffi {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RenderBufferFormat {
        Float32Vec4,
        Float32Vec3,
        Float32,
        UNorm8Vec4,
        UNorm8Vec3,
        UNorm8,
        Int32,
    }

    extern "Rust" {
        fn new_bridge_render_delegate() -> Box<BridgeRenderDelegate>;

        type BridgeRenderDelegate;
        fn get_supported_rprim_types(self: &BridgeRenderDelegate) -> Vec<String>;
        fn get_supported_sprim_types(self: &BridgeRenderDelegate) -> Vec<String>;
        fn get_supported_bprim_types(self: &BridgeRenderDelegate) -> Vec<String>;
        fn init(self: &BridgeRenderDelegate);
        fn destroy(self: &BridgeRenderDelegate);
        fn create_render_buffer(self: &BridgeRenderDelegate, id: String)
            -> Box<BridgeRenderBuffer>;

        type BridgeRenderBuffer;
        fn allocate(
            self: &BridgeRenderBuffer,
            width: usize,
            height: usize,
            format: RenderBufferFormat,
        );
        fn read(self: &BridgeRenderBuffer) -> Vec<u8>;
        fn finalize(self: &BridgeRenderBuffer);
    }
}

fn new_bridge_render_delegate() -> Box<BridgeRenderDelegate> {
    let render_delegate = crate::create_render_delegate();
    BridgeRenderDelegate::new(render_delegate)
}

struct BridgeRenderDelegate {
    item: Box<dyn RenderDelegate<RenderBuffer = Box<dyn RenderBuffer>>>,
}
impl BridgeRenderDelegate {
    fn new(render_delegate: impl RenderDelegate + 'static) -> Box<Self> {
        Box::new(BridgeRenderDelegate {
            item: Box::new(RenderBufferBoxedRenderDelegate {
                item: render_delegate,
            }),
        })
    }

    fn get_supported_rprim_types(&self) -> Vec<String> {
        self.item.get_supported_rprim_types()
    }

    fn get_supported_sprim_types(&self) -> Vec<String> {
        self.item.get_supported_sprim_types()
    }

    fn get_supported_bprim_types(&self) -> Vec<String> {
        self.item.get_supported_bprim_types()
    }

    fn init(&self) {
        self.item.init()
    }

    fn destroy(&self) {
        self.item.destroy()
    }

    fn create_render_buffer(&self, id: String) -> Box<BridgeRenderBuffer> {
        let id = RenderBufferId(id);
        let item = self.item.create_render_buffer(id);
        BridgeRenderBuffer::new(item)
    }
}

struct BridgeRenderBuffer {
    item: Box<dyn RenderBuffer>,
}
impl BridgeRenderBuffer {
    fn new(render_buffer: Box<dyn RenderBuffer>) -> Box<Self> {
        Box::new(Self {
            item: render_buffer,
        })
    }
    fn allocate(&self, width: usize, height: usize, format: ffi::RenderBufferFormat) {
        self.item.allocate(width, height, format.into())
    }
    fn read(&self) -> Vec<u8> {
        self.item.read()
    }
    fn finalize(&self) {
        self.item.finalize()
    }
}

struct RenderBufferBoxedRenderDelegate<
    RB: RenderBuffer + 'static,
    R: RenderDelegate<RenderBuffer = RB>,
> {
    item: R,
}
impl<RB: RenderBuffer + 'static, R: RenderDelegate<RenderBuffer = RB>> RenderDelegate
    for RenderBufferBoxedRenderDelegate<RB, R>
{
    type RenderBuffer = Box<dyn RenderBuffer>;
    fn get_supported_rprim_types(&self) -> Vec<String> {
        self.item.get_supported_rprim_types()
    }

    fn get_supported_sprim_types(&self) -> Vec<String> {
        self.item.get_supported_sprim_types()
    }

    fn get_supported_bprim_types(&self) -> Vec<String> {
        self.item.get_supported_bprim_types()
    }

    fn init(&self) {
        self.item.init()
    }

    fn destroy(&self) {
        self.item.destroy()
    }

    fn create_render_buffer(&self, id: RenderBufferId) -> Self::RenderBuffer {
        let item = self.item.create_render_buffer(id);
        Box::new(item)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenderBufferId(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderBufferFormat {
    Float32Vec4,
    Float32Vec3,
    Float32,
    UNorm8Vec4,
    UNorm8Vec3,
    UNorm8,
    Int32,
}
impl From<ffi::RenderBufferFormat> for RenderBufferFormat {
    fn from(format: ffi::RenderBufferFormat) -> Self {
        match format {
            ffi::RenderBufferFormat::Float32Vec4 => Self::Float32Vec4,
            ffi::RenderBufferFormat::Float32Vec3 => Self::Float32Vec3,
            ffi::RenderBufferFormat::Float32 => Self::Float32,
            ffi::RenderBufferFormat::UNorm8Vec4 => Self::UNorm8Vec4,
            ffi::RenderBufferFormat::UNorm8Vec3 => Self::UNorm8Vec3,
            ffi::RenderBufferFormat::UNorm8 => Self::UNorm8,
            ffi::RenderBufferFormat::Int32 => Self::Int32,
            _ => Self::Float32Vec3,
        }
    }
}

pub trait RenderDelegate: Send + Sync {
    type RenderBuffer: RenderBuffer;
    fn get_supported_rprim_types(&self) -> Vec<String>;
    fn get_supported_sprim_types(&self) -> Vec<String>;
    fn get_supported_bprim_types(&self) -> Vec<String>;
    fn init(&self);
    fn destroy(&self);
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

#[cxx::bridge]
pub mod ffi {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RenderBufferFormat {
        UNorm8,
        UNorm8Vec2,
        UNorm8Vec3,
        UNorm8Vec4,

        SNorm8,
        SNorm8Vec2,
        SNorm8Vec3,
        SNorm8Vec4,

        Float16,
        Float16Vec2,
        Float16Vec3,
        Float16Vec4,

        Float32,
        Float32Vec2,
        Float32Vec3,
        Float32Vec4,

        Int16,
        Int16Vec2,
        Int16Vec3,
        Int16Vec4,

        UInt16,
        UInt16Vec2,
        UInt16Vec3,
        UInt16Vec4,

        Int32,
        Int32Vec2,
        Int32Vec3,
        Int32Vec4,

        Float32UInt8,
    }

    extern "Rust" {
        fn new_bridge_render_delegate() -> Box<BridgeRenderDelegate>;

        type BridgeRenderDelegate;
        fn get_supported_rprim_types(self: &BridgeRenderDelegate) -> Vec<String>;
        fn get_supported_sprim_types(self: &BridgeRenderDelegate) -> Vec<String>;
        fn get_supported_bprim_types(self: &BridgeRenderDelegate) -> Vec<String>;
        fn init(self: &BridgeRenderDelegate);
        fn destroy(self: &BridgeRenderDelegate);
        fn render(self: &BridgeRenderDelegate);
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

    fn render(&self) {
        self.item.render()
    }

    fn create_render_buffer(&self, id: String) -> Box<BridgeRenderBuffer> {
        println!("create_render_buffer with id: {}", id);
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

    fn render(&self) {
        self.item.render()
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
    UNorm8,
    UNorm8Vec2,
    UNorm8Vec3,
    UNorm8Vec4,

    SNorm8,
    SNorm8Vec2,
    SNorm8Vec3,
    SNorm8Vec4,

    Float16,
    Float16Vec2,
    Float16Vec3,
    Float16Vec4,

    Float32,
    Float32Vec2,
    Float32Vec3,
    Float32Vec4,

    Int16,
    Int16Vec2,
    Int16Vec3,
    Int16Vec4,

    UInt16,
    UInt16Vec2,
    UInt16Vec3,
    UInt16Vec4,

    Int32,
    Int32Vec2,
    Int32Vec3,
    Int32Vec4,

    Float32UInt8,

    Invalid,
}
impl RenderBufferFormat {
    pub fn component_size(&self) -> usize {
        match self {
            RenderBufferFormat::UNorm8 => 1,
            RenderBufferFormat::UNorm8Vec2 => 2,
            RenderBufferFormat::UNorm8Vec3 => 3,
            RenderBufferFormat::UNorm8Vec4 => 4,

            RenderBufferFormat::SNorm8 => 1,
            RenderBufferFormat::SNorm8Vec2 => 2,
            RenderBufferFormat::SNorm8Vec3 => 3,
            RenderBufferFormat::SNorm8Vec4 => 4,

            RenderBufferFormat::Float16 => 2,
            RenderBufferFormat::Float16Vec2 => 4,
            RenderBufferFormat::Float16Vec3 => 6,
            RenderBufferFormat::Float16Vec4 => 8,

            RenderBufferFormat::Float32 => 4,
            RenderBufferFormat::Float32Vec2 => 8,
            RenderBufferFormat::Float32Vec3 => 12,
            RenderBufferFormat::Float32Vec4 => 16,

            RenderBufferFormat::Int16 => 2,
            RenderBufferFormat::Int16Vec2 => 4,
            RenderBufferFormat::Int16Vec3 => 6,
            RenderBufferFormat::Int16Vec4 => 8,

            RenderBufferFormat::UInt16 => 2,
            RenderBufferFormat::UInt16Vec2 => 4,
            RenderBufferFormat::UInt16Vec3 => 6,
            RenderBufferFormat::UInt16Vec4 => 8,

            RenderBufferFormat::Int32 => 4,
            RenderBufferFormat::Int32Vec2 => 8,
            RenderBufferFormat::Int32Vec3 => 12,
            RenderBufferFormat::Int32Vec4 => 16,

            RenderBufferFormat::Float32UInt8 => 5,

            RenderBufferFormat::Invalid => 0,
        }
    }
}
impl From<ffi::RenderBufferFormat> for RenderBufferFormat {
    fn from(format: ffi::RenderBufferFormat) -> Self {
        match format {
            ffi::RenderBufferFormat::UNorm8 => Self::UNorm8,
            ffi::RenderBufferFormat::UNorm8Vec2 => Self::UNorm8Vec2,
            ffi::RenderBufferFormat::UNorm8Vec3 => Self::UNorm8Vec3,
            ffi::RenderBufferFormat::UNorm8Vec4 => Self::UNorm8Vec4,

            ffi::RenderBufferFormat::SNorm8 => Self::SNorm8,
            ffi::RenderBufferFormat::SNorm8Vec2 => Self::SNorm8Vec2,
            ffi::RenderBufferFormat::SNorm8Vec3 => Self::SNorm8Vec3,
            ffi::RenderBufferFormat::SNorm8Vec4 => Self::SNorm8Vec4,

            ffi::RenderBufferFormat::Float16 => Self::Float16,
            ffi::RenderBufferFormat::Float16Vec2 => Self::Float16Vec2,
            ffi::RenderBufferFormat::Float16Vec3 => Self::Float16Vec3,
            ffi::RenderBufferFormat::Float16Vec4 => Self::Float16Vec4,

            ffi::RenderBufferFormat::Float32 => Self::Float32,
            ffi::RenderBufferFormat::Float32Vec2 => Self::Float32Vec2,
            ffi::RenderBufferFormat::Float32Vec3 => Self::Float32Vec3,
            ffi::RenderBufferFormat::Float32Vec4 => Self::Float32Vec4,

            ffi::RenderBufferFormat::Int16 => Self::Int16,
            ffi::RenderBufferFormat::Int16Vec2 => Self::Int16Vec2,
            ffi::RenderBufferFormat::Int16Vec3 => Self::Int16Vec3,
            ffi::RenderBufferFormat::Int16Vec4 => Self::Int16Vec4,

            ffi::RenderBufferFormat::UInt16 => Self::UInt16,
            ffi::RenderBufferFormat::UInt16Vec2 => Self::UInt16Vec2,
            ffi::RenderBufferFormat::UInt16Vec3 => Self::UInt16Vec3,
            ffi::RenderBufferFormat::UInt16Vec4 => Self::UInt16Vec4,

            ffi::RenderBufferFormat::Int32 => Self::Int32,
            ffi::RenderBufferFormat::Int32Vec2 => Self::Int32Vec2,
            ffi::RenderBufferFormat::Int32Vec3 => Self::Int32Vec3,
            ffi::RenderBufferFormat::Int32Vec4 => Self::Int32Vec4,

            ffi::RenderBufferFormat::Float32UInt8 => Self::Float32UInt8,

            _ => unreachable!("Invalid RenderBufferFormat"),
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

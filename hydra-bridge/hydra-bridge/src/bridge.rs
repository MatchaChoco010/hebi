//! Module for interactions between C++ and Rust using CXX.

use std::sync::OnceLock;

mod type_conversion;
pub use type_conversion::*;

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

        // BridgeRenderDelegate is called directly from C++
        type BridgeRenderDelegate;
        fn get_supported_rprim_types(self: &BridgeRenderDelegate) -> Vec<String>;
        fn get_supported_sprim_types(self: &BridgeRenderDelegate) -> Vec<String>;
        fn get_supported_bprim_types(self: &BridgeRenderDelegate) -> Vec<String>;
        fn init(self: &BridgeRenderDelegate);
        fn destroy(self: &BridgeRenderDelegate);
        fn render(self: &BridgeRenderDelegate);
        fn create_render_buffer(self: &BridgeRenderDelegate, id: String)
            -> Box<BridgeRenderBuffer>;

        // BridgeRenderBuffer is called directly from C++
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
    let render_delegate_creator = crate::CREATE_RENDER_DELEGATE_FN
        .get()
        .expect("should be set up create_create_render_delegate");
    (render_delegate_creator.f)()
}

struct BridgeRenderDelegate {
    item: Box<dyn crate::RenderDelegate<RenderBuffer = Box<dyn crate::RenderBuffer>>>,
}
impl BridgeRenderDelegate {
    fn new(render_delegate: impl crate::RenderDelegate + 'static) -> Box<Self> {
        Box::new(Self {
            item: RenderDelegateBoxedDynRenderBuffer::new(render_delegate),
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
        let id = RenderBufferId(id);
        let item = self.item.create_render_buffer(id);
        BridgeRenderBuffer::new(item)
    }
}

struct BridgeRenderBuffer {
    item: Box<dyn crate::RenderBuffer>,
}
impl BridgeRenderBuffer {
    fn new(render_buffer: Box<dyn crate::RenderBuffer>) -> Box<Self> {
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

// Structure that converts generation methods of types implementing RenderDelegate to
// generation methods of BridgeRenderDelegate.
pub static CREATE_RENDER_DELEGATE_FN: OnceLock<BridgeRenderDelegateCreator> = OnceLock::new();
pub struct BridgeRenderDelegateCreator {
    f: Box<dyn Fn() -> Box<BridgeRenderDelegate> + Send + Sync>,
}
impl BridgeRenderDelegateCreator {
    pub fn new<
        RB: crate::RenderBuffer + 'static,
        R: crate::RenderDelegate<RenderBuffer = RB> + 'static,
    >(
        f: fn() -> R,
    ) -> Self {
        Self {
            f: Box::new(move || BridgeRenderDelegate::new(f())),
        }
    }
}

// Wrapper type for converting `impl RenderDelegate` to
// `Box<dyn RenderDelegate<RenderBuffer = Box<dyn RenderBuffer>>>`
struct RenderDelegateBoxedDynRenderBuffer<
    RB: crate::RenderBuffer + 'static,
    R: crate::RenderDelegate<RenderBuffer = RB>,
>(R);
impl<RB: crate::RenderBuffer + 'static, R: crate::RenderDelegate<RenderBuffer = RB>>
    RenderDelegateBoxedDynRenderBuffer<RB, R>
{
    fn new(render_delegate: R) -> Box<Self> {
        Box::new(Self(render_delegate))
    }
}
impl<RB: crate::RenderBuffer + 'static, R: crate::RenderDelegate<RenderBuffer = RB>>
    crate::RenderDelegate for RenderDelegateBoxedDynRenderBuffer<RB, R>
{
    type RenderBuffer = Box<dyn crate::RenderBuffer>;

    fn get_supported_rprim_types(&self) -> Vec<String> {
        self.0.get_supported_rprim_types()
    }

    fn get_supported_sprim_types(&self) -> Vec<String> {
        self.0.get_supported_sprim_types()
    }

    fn get_supported_bprim_types(&self) -> Vec<String> {
        self.0.get_supported_bprim_types()
    }

    fn init(&self) {
        self.0.init()
    }

    fn destroy(&self) {
        self.0.destroy()
    }

    fn render(&self) {
        self.0.render()
    }

    fn create_render_buffer(&self, id: RenderBufferId) -> Self::RenderBuffer {
        let item = self.0.create_render_buffer(id);
        Box::new(item)
    }
}

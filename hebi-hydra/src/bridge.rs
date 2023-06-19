use std::sync::OnceLock;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("hebi-hydra/cpp/entry.h");

        fn entry();
    }
    extern "Rust" {
        fn initialize();

        type BridgeRenderDelegate;
        fn new_bridge_render_delegate() -> &'static BridgeRenderDelegate;
        fn get_supported_rprim_types(self: &BridgeRenderDelegate) -> Vec<String>;
        fn get_supported_sprim_types(self: &BridgeRenderDelegate) -> Vec<String>;
        fn get_supported_bprim_types(self: &BridgeRenderDelegate) -> Vec<String>;
        fn init(self: &BridgeRenderDelegate);
        fn destroy(self: &BridgeRenderDelegate);
    }
}

static RENDER_DELEGATE: OnceLock<BridgeRenderDelegate> = OnceLock::new();

pub fn register(render_delegate: Box<dyn RenderDelegate + Sync + Send>) {
    let bridge = BridgeRenderDelegate::new(render_delegate);
    RENDER_DELEGATE.set(bridge).ok().unwrap();
}

fn initialize() {
    crate::register();
    ffi::entry();
}

struct BridgeRenderDelegate {
    item: Box<dyn RenderDelegate + Sync + Send>,
}
impl BridgeRenderDelegate {
    fn new(render_delegate: Box<dyn RenderDelegate + Sync + Send>) -> Self {
        Self {
            item: render_delegate,
        }
    }

    fn get_supported_rprim_types(self: &BridgeRenderDelegate) -> Vec<String> {
        self.item.get_supported_rprim_types()
    }

    fn get_supported_sprim_types(self: &BridgeRenderDelegate) -> Vec<String> {
        self.item.get_supported_sprim_types()
    }

    fn get_supported_bprim_types(self: &BridgeRenderDelegate) -> Vec<String> {
        self.item.get_supported_bprim_types()
    }

    fn init(self: &BridgeRenderDelegate) {
        self.item.init()
    }

    fn destroy(self: &BridgeRenderDelegate) {
        self.item.destroy()
    }
}

fn new_bridge_render_delegate() -> &'static BridgeRenderDelegate {
    RENDER_DELEGATE.get().unwrap()
}

pub trait RenderDelegate {
    fn get_supported_rprim_types(&self) -> Vec<String>;
    fn get_supported_sprim_types(&self) -> Vec<String>;
    fn get_supported_bprim_types(&self) -> Vec<String>;
    fn init(&self);
    fn destroy(&self);
}

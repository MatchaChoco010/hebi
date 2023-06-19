use std::sync::OnceLock;

#[cxx::bridge]
pub mod ffi {
    extern "Rust" {
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

struct BridgeRenderDelegate {
    item: Box<dyn RenderDelegate>,
}
impl BridgeRenderDelegate {
    fn new(render_delegate: Box<dyn RenderDelegate>) -> Self {
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
    // ffi::dummy();
    RENDER_DELEGATE.get_or_init(|| {
        let render_delegate = crate::register();
        BridgeRenderDelegate::new(render_delegate)
    })
}

pub trait RenderDelegate: Send + Sync + 'static {
    fn get_supported_rprim_types(&self) -> Vec<String>;
    fn get_supported_sprim_types(&self) -> Vec<String>;
    fn get_supported_bprim_types(&self) -> Vec<String>;
    fn init(&self);
    fn destroy(&self);
}

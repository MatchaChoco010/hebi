mod bridge;

pub fn register() {
    bridge::register(HebiRenderDelegate::new());
}

struct HebiRenderDelegate {}
impl HebiRenderDelegate {
    fn new() -> Box<Self> {
        Box::new(Self {})
    }
}
impl bridge::RenderDelegate for HebiRenderDelegate {
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
}

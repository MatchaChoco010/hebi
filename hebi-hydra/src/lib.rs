#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("hebi-hydra/cpp/entry.h");

        fn entry();
    }
    extern "Rust" {
        fn initialize();
    }
}

pub fn initialize() {
    println!("initialize");
    ffi::entry();
}

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let mut vcvars = vcvars::Vcvars::new();
    let vcvars_include = vcvars.get_cached("INCLUDE").unwrap();

    let mut vcvars = vcvars::Vcvars::new();
    let vcvars_lib = vcvars.get_cached("LIB").unwrap();

    let mut vcvars = vcvars::Vcvars::new();
    let vcvars_libpath = vcvars.get_cached("LIBPATH").unwrap();

    let mut vcvars = vcvars::Vcvars::new();
    let vcvars_path = vcvars.get_cached("PATH").unwrap();

    let mut vcvars = vcvars::Vcvars::new();
    let visual_studio_version = vcvars.get_cached("VisualStudioVersion").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let usd_dir = PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .join("OpenUSD");
    let usd_dir = usd_dir.to_str().unwrap();
    let usd_dst = PathBuf::from(&out_dir).join("OpenUSD");

    let pxr_lib_prefix = env::var("PXR_LIB_PREFIX").unwrap_or("usd".to_string());

    let output = Command::new("cmd")
        .env("INCLUDE", &*vcvars_include)
        .env("LIB", &*vcvars_lib)
        .env("LIBPATH", &*vcvars_libpath)
        .env("PATH", &*vcvars_path)
        .env("VisualStudioVersion", &*visual_studio_version)
        .args([
            "/C",
            "python",
            &format!("{usd_dir}/build_scripts/build_usd.py"),
            "--no-python",
            usd_dst.to_str().unwrap(),
            "--build-args",
            &format!("USD,\"-DPXR_LIB_PREFIX={pxr_lib_prefix}\""),
        ])
        .output()
        .expect("failed to execute process");
    if !output.status.success() {
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("failed to build OpenUSD");
    }

    cxx_build::bridge("src/bridge.rs")
        .cpp(true)
        .debug(false)
        .define("NOMINMAX", None)
        .flag_if_supported("-std=c++17")
        .flag_if_supported("/utf-8")
        .includes(env::split_paths(&*vcvars_include))
        .include(usd_dst.join("include"))
        .include(usd_dst.join("include/boost-1_70"))
        .file("cpp/mesh.cpp")
        .file("cpp/renderBuffer.cpp")
        .file("cpp/renderDelegate.cpp")
        .file("cpp/rendererPlugin.cpp")
        .file("cpp/renderPass.cpp")
        .link_lib_modifier("-bundle")
        .link_lib_modifier("+whole-archive")
        .compile("hebi-hydra-cpp");

    println!("cargo:rerun-if-changed=rs");
    println!("cargo:rerun-if-changed=cpp");

    println!("cargo:rustc-link-search={}", usd_dst.join("lib").display());
    for f in fs::read_dir(usd_dst.join("lib")).unwrap() {
        let f = f.unwrap();
        let f = f.file_name();
        let f = f.to_str().unwrap();
        if f.ends_with(".lib") {
            let f = f.trim_end_matches(".lib");
            println!("cargo:rustc-link-lib={}", f);
        }
    }
}

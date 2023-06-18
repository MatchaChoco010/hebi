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

    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let tbb_dir = PathBuf::from(manifest_dir).parent().unwrap().join("oneTBB");

    let boost_dst = cmake::Config::new("../boost")
        .out_dir(format!("{}/build/boost", out_dir))
        .always_configure(false)
        .profile("Release")
        .cxxflag("/EHsc")
        .build();

    let tbb_dst = PathBuf::from(out_dir.clone()).join("build/tbb");
    let tbb_dst = tbb_dst.to_str().unwrap();
    fs::create_dir_all(tbb_dst).unwrap();
    Command::new("make")
        .current_dir(tbb_dir.clone())
        .env("INCLUDE", &*vcvars_include)
        .env("LIB", &*vcvars_lib)
        .env("LIBPATH", &*vcvars_libpath)
        .env("PATH", &*vcvars_path)
        .args([
            &format!("tbb_root={}", tbb_dir.to_str().unwrap()),
            "-C",
            tbb_dst,
            "-r",
            "-f",
            tbb_dir.join("build/Makefile.tbb").to_str().unwrap(),
            "cfg=release",
        ])
        .status()
        .unwrap();

    let usd_dst = cmake::Config::new("../USD")
        .profile("Release")
        .always_configure(false)
        .out_dir(PathBuf::from(out_dir).join("build/usd"))
        .define("PXR_BUILD_EXAMPLES", "false")
        .define("PXR_BUILD_TESTS", "false")
        .define("PXR_BUILD_TUTORIALS", "false")
        .define("PXR_ENABLE_PYTHON_SUPPORT", "false")
        .cxxflag("/EHsc")
        .build();

    cxx_build::bridge("src/lib.rs")
        .cpp(true)
        .debug(false)
        .define("NOMINMAX", None)
        .flag_if_supported("-std=c++17")
        .includes(env::split_paths(&*vcvars_include))
        .include(boost_dst.join("include/boost-1_82"))
        .include("../oneTBB/include")
        .include(usd_dst.join("include"))
        .file("cpp/entry.cpp")
        .file("cpp/mesh.cpp")
        .file("cpp/renderDelegate.cpp")
        .file("cpp/rendererPlugin.cpp")
        .file("cpp/renderPass.cpp")
        .compile("hebi-hydra-cpp");

    println!("cargo:rustc-link-search=native={}", tbb_dst);
    println!(
        "cargo:rustc-link-search=native={}",
        usd_dst.join("lib").to_str().unwrap()
    );

    for f in fs::read_dir(usd_dst.join("lib")).unwrap() {
        let f = f.unwrap();
        let f = f.file_name();
        let f = f.to_str().unwrap();
        if f.ends_with(".lib") {
            let f = f.trim_end_matches(".lib");
            println!("cargo:rustc-link-lib=dylib={}", f);
        }
    }

    println!("cargo:rerun-if-changed=cpp");
}

use bbl_build::Config;
use std::path::PathBuf;

pub fn main() {
    let _dst = Config::new("openusd", "bbl-usd")
        .generator("Ninja")
        .profile("Release")
        .define("BBL_LANGUAGES", "rust")
        .define(
            "CMAKE_PREFIX_PATH",
            std::env::var("CMAKE_PREFIX_PATH").expect("Missing CMAKE_PREFIX_PATH env var"),
        )
        .build();

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindgen::builder()
        .header(out_path.join("build/openusd-c.h").to_str().unwrap())
        .generate()
        .unwrap()
        .write_to_file(out_path.join("build/openusd.rs"))
        .unwrap();

    println!("cargo:rerun-if-changed=bbl-usd");
}

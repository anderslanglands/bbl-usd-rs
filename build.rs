use bbl_build::Config;
use std::path::PathBuf;

pub fn main() {
    let _dst = Config::new("openusd", "bbl-usd")
        .define("BBL_LANGUAGES", "rust")
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

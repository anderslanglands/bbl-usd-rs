use bbl_build::Config;

pub fn main() {
    let _dst = Config::new("openusd", "bbl-usd")
        .generator("Ninja")
        .profile("Release")
        .define("BBL_LANGUAGES", "rust")
        .very_verbose(true)
        .env("CMAKE_C_FLAGS", "")
        .env("CMAKE_CXX_FLAGS", "")
        .build();

    println!("cargo:rerun-if-changed=../bbl-usd");
}
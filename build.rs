use bbl_build::Config;

pub fn main() {
    let _dst = Config::new("openusd", "bbl-usd")
        .define("BBL_LANGUAGES", "rust")
        .build();

    println!("cargo:rerun-if-changed=bbl-usd");
}


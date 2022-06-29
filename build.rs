extern crate rustc_version;

use rustc_version::{version_meta, Channel};

fn main() {
    let meta = version_meta().unwrap();
    if meta.channel == Channel::Nightly {
        println!("cargo:rustc-cfg=feature=\"nightly\"");
    }
}

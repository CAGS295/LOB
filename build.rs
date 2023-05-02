use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    prost_build::Config::new()
        .file_descriptor_set_path(out_dir.join("limitorderbook_descriptor.bin"))
        .compile_protos(&["proto/limit_order_book.proto"], &["proto"])
        .unwrap();
}

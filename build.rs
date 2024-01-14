use std::{env, path::PathBuf};

fn main() {
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");
    prost_build::Config::new()
        .file_descriptor_set_path(&descriptor_path)
        .compile_well_known_types()
        .extern_path(".google.protobuf", "::pbjson_types")
        .compile_protos(&["example.proto"], &[""])
        .unwrap();
    let descriptor_set = std::fs::read(descriptor_path).unwrap();
    pbjson_build::Builder::new()
        .ignore_unknown_fields()
        .register_descriptors(&descriptor_set)
        .unwrap()
        .build(&[".hello"])
        .unwrap();
}

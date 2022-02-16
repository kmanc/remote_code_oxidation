extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["rat.proto"], &["src/../../"]).unwrap();
}
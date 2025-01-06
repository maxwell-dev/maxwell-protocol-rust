use prost_build;

fn main() {
    build_by_prost();
}

fn build_by_prost() {
    let mut prost_build = prost_build::Config::new();
    prost_build
    .out_dir("../src/protocol")
    .type_attribute("RouteGroup", "#[derive(::serde::Serialize)] #[serde(rename_all = \"camelCase\")]")
    .compile_protos(&["proto/maxwell_protocol.normalized.proto"], &["proto/"])
    .unwrap();
}

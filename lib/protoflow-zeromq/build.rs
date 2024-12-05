use std::io::Result;
fn main() -> Result<()> {
    prost_build::Config::default()
        .out_dir("src/")
        .compile_protos(&["proto/transport_event.proto"], &["proto/"])
}

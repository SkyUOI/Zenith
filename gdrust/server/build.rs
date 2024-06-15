fn main() -> shadow_rs::SdResult<()> {
    prost_build::Config::new()
        .out_dir("src/proto")
        .compile_protos(&["proto/connect.proto"], &["proto"])
        .unwrap();
    shadow_rs::new()
}

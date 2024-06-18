fn main() {
    println!("cargo:rerun-if-changed=proto");
    prost_build::Config::new()
        .out_dir("src/proto")
        .compile_protos(&["proto/connect.proto"], &["proto"])
        .unwrap();
}

use std::process::exit;

fn main() {
    let output = std::process::Command::new("protoc")
        .arg("--version")
        .output();
    let mut should_compile = false;
    match output {
        Ok(data) => {
            if !data.status.success() {
                // 不存在
                should_compile = true;
            }
        }
        Err(e) => {
            should_compile = true;
            eprintln!("protoc error:{}", e)
        }
    }
    if should_compile {
        if cfg!(feature = "protobuf_feature") {
            std::env::set_var("PROTOC", protobuf_src::protoc());
        } else {
            eprintln!("protoc not found,you can try use \"cargo build --features protobuf_feature\" to compile a protoc");
            exit(1);
        }
    }
    prost_build::Config::new()
        .out_dir("src/proto")
        .compile_protos(&["proto/connect.proto"], &["proto"])
        .unwrap();
}

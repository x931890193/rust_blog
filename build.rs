
fn main() {
    // std::fs::create_dir_all("src/proto").unwrap();
    // protobuf_codegen::Codegen::new()
    //     // .pure()
    //     .out_dir("src/proto")
    //     .inputs(&["proto/pb.proto"])
    //     .include("proto")
    //     .customize(
    //         protobuf_codegen::Customize::default()
    //             .tokio_bytes(true)
    //     )
    //     .run()
    //     .expect("Codegen failed.");

    prost_build::Config::new()
        .out_dir("src/proto")
        .compile_protos(&["src/proto/pb.proto"], &["src/proto"])
        .unwrap();
}

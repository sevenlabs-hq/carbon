use tonic_build::configure;

fn main() {
    const PROTOC_ENVAR: &str = "PROTOC";
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let protos_dir = std::path::Path::new(&manifest_dir).join("protos");
    let auth_proto = protos_dir.join("auth.proto");
    let shared_proto = protos_dir.join("shared.proto");
    let shredstream_proto = protos_dir.join("shredstream.proto");
    if std::env::var(PROTOC_ENVAR).is_err() {
        #[cfg(not(windows))]
        std::env::set_var(PROTOC_ENVAR, protobuf_src::protoc());
    }

    configure()
        .compile_protos(
            &[auth_proto, shared_proto, shredstream_proto],
            &[protos_dir],
        )
        .expect("Failed to compile protos");
}

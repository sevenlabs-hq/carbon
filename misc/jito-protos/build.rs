use tonic_build::configure;

fn main() {
    const PROTOC_ENVAR: &str = "PROTOC";
    if std::env::var(PROTOC_ENVAR).is_err() {
        #[cfg(not(windows))]
        std::env::set_var(PROTOC_ENVAR, protobuf_src::protoc());
    }

    configure()
        .compile_protos(
            &[
                "protos/auth.proto",
                "protos/shared.proto",
                "protos/shredstream.proto",
            ],
            &["protos"],
        )
        .expect("Failed to compile protos");
}

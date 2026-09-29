fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use the vendored protoc so builds don't depend on a system install.
    // SAFETY: build scripts are single-threaded.
    unsafe {
        std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path()?);
    }

    tonic_prost_build::configure()
        .build_client(std::env::var_os("CARGO_FEATURE_CLIENT").is_some())
        .build_server(std::env::var_os("CARGO_FEATURE_SERVER").is_some())
        .compile_protos(&["proto/spec_provider.proto"], &["proto"])?;

    Ok(())
}

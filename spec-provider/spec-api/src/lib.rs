//! gRPC API of the spec-provider.
//!
//! Enable the `client` feature (default) to call the spec-provider,
//! or the `server` feature to implement it.

tonic::include_proto!("spec_provider");

//! End-to-end tests for the services in this repository.
//!
//! The services must already be running (see `README.md`). Their URLs come from `.env`.

mod client;
mod config;

pub use client::client;
pub use config::Config;

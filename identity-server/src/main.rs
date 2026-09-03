use std::net::SocketAddr;

use tokio::net::TcpListener;

use crate::config::CONFIG;

mod api;
mod application;
mod config;
mod domain;
mod persistence;

#[tokio::main]
async fn main() {
    println!("[STARTUP] Application starting...");

    println!("[STARTUP] Building router...");
    let app = api::router();

    // Specify the address to bind to (0.0.0.0 to listen on all interfaces)
    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.app_port()));

    // Create listener on address
    println!("[STARTUP] Binding to address: {}", addr);
    let listener = TcpListener::bind(addr)
        .await
        .expect(format!("[STARTUP] Failed to create TCP listener: {}", addr).as_str());

    // Start the Axum server
    println!("[STARTUP] Server running at {}", addr);
    axum::serve(listener, app)
        .await
        .expect("[STARTUP] Failed to launch server");
}

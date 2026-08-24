use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;

mod app_state;
mod authorize;
mod domain;
mod error;
mod persistence;
mod service;

#[tokio::main]
async fn main() {
    println!("[STARTUP] Application starting...");

    println!("[STARTUP] Building router...");
    let state = app_state::create_app_state();
    let app = Router::new().merge(authorize::router()).with_state(state);

    // Specify the address to bind to (0.0.0.0 to listen on all interfaces)
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

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

mod api;
mod config;
mod error;
mod models;
mod services;

use api::handle_ai_request;
use axum::{
    routing::{get, post},
    Router,
};
use config::Config;
use error::AppResult;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Initialize configuration from environment
    let config = Config::from_env()?;

    // Create TCP listener
    let listener = TcpListener::bind(&config.server_address)
        .await
        .map_err(|e| error::AppError::Server(format!("Could not create tcp listener: {}", e)))?;

    println!("listening on {}", listener.local_addr().unwrap());

    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/ai/action", post(handle_ai_request))
        .with_state(config)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    println!("Server started successfully");

    // Serve the application
    axum::serve(listener, app)
        .await
        .map_err(|e| error::AppError::Server(format!("Error serving application: {}", e)))?;

    Ok(())
}
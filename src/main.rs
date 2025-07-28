mod auth;
mod handlers;
mod models;
mod state;

use auth::{login, logout, refresh};
use handlers::create::create_data;
use handlers::delete::delete_data;
use handlers::read::{read_all_data, read_data};
use handlers::update::update_data;
use handlers::execute::execute_fn;
use std::env;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::info;

static CALL_COUNTER: AtomicU64 = AtomicU64::new(0);

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();
    
    // Load environment variables from .env file (if it exists)
    dotenv::dotenv().ok();

    // Create the global application state
    let state = state::new_state();

    // Create the Tide app and associate the state
    let mut app = tide::with_state(state);

    // Adiciona um middleware para logar a rota chamada e o contador
    app.with(tide::utils::Before(|req: tide::Request<_>| async move {
        let count = CALL_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
        info!(
            method = %req.method(),
            path = %req.url().path(),
            total_calls = %count,
            "Request received"
        );
        req
    }));

    // Define authentication routes
    app.at("/auth/login").post(login);
    app.at("/auth/refresh").post(refresh);
    app.at("/auth/logout").post(logout);

    // Define CRUD routes (now protected by JWT authentication)
    app.at("/data").post(create_data); // Create
    app.at("/data").get(read_all_data); // Read all
    app.at("/data/:id").get(read_data); // Read one
    app.at("/data/:id").put(update_data); // Update
    app.at("/data/:id").delete(delete_data); // Delete
    app.at("/execute/:id").post(execute_fn); // Executa funções wasm

    // Get server address from environment variable or use default
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    
    info!(
        server_url = format!("http://{}", addr),
        "CRUD server with JWT authentication and refresh tokens started"
    );
    
    info!("Available users for testing:");
    info!(username = "admin", password = "admin123");
    info!(username = "user1", password = "password123");
    info!(username = "user2", password = "password456");
    
    info!(
        access_token_expiration = "1 hour",
        refresh_token_expiration = "30 days",
        "Token configuration"
    );

    // Start the server
    info!(address = %addr, "Starting server...");
    app.listen(addr).await?;
    Ok(())
}
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

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Load environment variables from .env file (if it exists)
    dotenv::dotenv().ok();

    // Create the global application state
    let state = state::new_state();

    // Create the Tide app and associate the state
    let mut app = tide::with_state(state);

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
    println!("CRUD server with JWT authentication and refresh tokens running at: http://{addr}");
    println!("Available users:");
    println!("  - admin/admin123");
    println!("  - user1/password123");
    println!("  - user2/password456");
    println!("Access tokens expire in 1 hour");
    println!("Refresh tokens expire in 30 days");

    // Start the server
    app.listen(addr).await?;
    Ok(())
}
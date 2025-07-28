use std::net::TcpListener;
use std::time::Duration;

// Helper to find an available port
pub fn find_available_port() -> u16 {
    // Start from a higher port range to avoid conflicts
    (9000..10000)
        .find(|port| {
            // Check if port is available
            TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok()
        })
        .unwrap_or(9000)
}

// Helper to wait for server to be ready
pub fn wait_for_server(base_url: &str, max_attempts: u32) -> bool {
    for attempt in 1..=max_attempts {
        // Try to make a GET request to check if server is responding
        match ureq::get(&format!("{}/data", base_url)).call() {
            Ok(response) => {
                // If we received any HTTP response, server is running
                println!("✅ Server responding on attempt {} (status: {})", attempt, response.status());
                return true;
            }
            Err(e) => {
                // Check if error is status 401 (unauthorized) - this means server is running!
                if e.to_string().contains("status code 401") {
                    println!("✅ Server responding on attempt {} (status: 401 - authentication required)", attempt);
                    return true;
                }
                
                if attempt < max_attempts {
                    println!("⏳ Attempt {}: waiting for server... (error: {})", attempt, e);
                    std::thread::sleep(Duration::from_millis(1000)); // Increased to 1 second
                } else {
                    println!("❌ Last attempt failed: {}", e);
                }
            }
        }
    }
    false
}

// Helper to start the server (synchronous)
pub fn start_test_server() -> (String, std::process::Child) {
    let port = find_available_port();
    let base_url = format!("http://127.0.0.1:{}", port);
    
    println!("🚀 Starting test server on port {}", port);
    
    // Set environment variable for server
    std::env::set_var("SERVER_ADDR", format!("127.0.0.1:{}", port));
    
    // Start server process
    let child = std::process::Command::new("cargo")
        .args(&["run", "--bin", "learn-rust-crud"])
        .env("SERVER_ADDR", format!("127.0.0.1:{}", port))
        .spawn()
        .expect("❌ Failed to start server");
    
    println!("⏳ Waiting for server to initialize...");
    
    // Wait for server to initialize and respond
    if !wait_for_server(&base_url, 30) { // Increased to 30 attempts
        panic!("❌ Server did not respond after 30 seconds");
    }
    
    println!("✅ Server started and responding at {}", base_url);
    (base_url, child)
}

// Helper to stop the server
pub fn stop_test_server(mut child: std::process::Child) {
    println!("🛑 Stopping test server...");
    let _ = child.kill();
    let _ = child.wait();
    println!("✅ Server stopped");
}

// Shared data structures
#[derive(serde::Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub username: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(serde::Serialize)]
pub struct TestData {
    pub func_names: Vec<String>,
    pub bytecode: Vec<u8>,
} 
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicU64;
use std::time::{Duration, Instant};

// Import the data model we defined
use crate::models::{DataEntry, RefreshTokenInfo};

pub struct Metrics {
    pub total_executions: AtomicU64,
    pub successful_executions: AtomicU64,
    pub failed_executions: AtomicU64,
    pub function_counts: HashMap<String, AtomicU64>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            total_executions: AtomicU64::new(0),
            successful_executions: AtomicU64::new(0),
            failed_executions: AtomicU64::new(0),
            function_counts: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct RateLimiter {
    #[allow(dead_code)]
    pub user_requests: HashMap<String, Vec<Instant>>,
    #[allow(dead_code)]
    pub max_requests: usize,
    #[allow(dead_code)]
    pub window_duration: Duration,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self {
            user_requests: HashMap::new(),
            max_requests: 100, // 100 requests per window
            window_duration: Duration::from_secs(60), // 1 minute window
        }
    }
}

// AppState is the global state of the application.
// We use Arc<Mutex<...>> to allow safe access between multiple requests.
pub type AppState = Arc<Mutex<AppStateInner>>;

pub struct AppStateInner {
    pub data: HashMap<u32, DataEntry>,
    pub users: HashMap<String, String>, // username -> password (in production, use hash)
    pub refresh_tokens: HashMap<String, RefreshTokenInfo>, // refresh_token -> info
    pub wasm_cache: HashMap<u32, Vec<u8>>, // Cache for compiled WASM modules
    pub metrics: Metrics,
    pub rate_limiter: RateLimiter,
}

// Creates a new empty state
pub fn new_state() -> AppState {
    let mut users = HashMap::new();
    // Add some example users
    users.insert("admin".to_string(), "admin123".to_string());
    users.insert("user1".to_string(), "password123".to_string());
    users.insert("user2".to_string(), "password456".to_string());

    Arc::new(Mutex::new(AppStateInner {
        data: HashMap::new(),
        users,
        refresh_tokens: HashMap::new(),
        wasm_cache: HashMap::new(),
        metrics: Metrics::default(),
        rate_limiter: RateLimiter::default(),
    }))
}
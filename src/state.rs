use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Import the data model we defined
use crate::models::{DataEntry, RefreshTokenInfo};

// AppState is the global state of the application.
// We use Arc<Mutex<...>> to allow safe access between multiple requests.
pub type AppState = Arc<Mutex<AppStateInner>>;

#[derive(Clone)]
pub struct AppStateInner {
    pub data: HashMap<u32, DataEntry>,
    pub users: HashMap<String, String>, // username -> password (in production, use hash)
    pub refresh_tokens: HashMap<String, RefreshTokenInfo>, // refresh_token -> info
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
    }))
}
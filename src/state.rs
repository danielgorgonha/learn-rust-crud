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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::DataEntry;

    #[test]
    fn test_new_state_creation() {
        let state = new_state();
        let state_guard = state.lock().unwrap();
        
        // Test default users
        assert!(state_guard.users.contains_key("admin"));
        assert!(state_guard.users.contains_key("user1"));
        assert!(state_guard.users.contains_key("user2"));
        assert_eq!(state_guard.users.get("admin").unwrap(), "admin123");
        assert_eq!(state_guard.users.get("user1").unwrap(), "password123");
        assert_eq!(state_guard.users.get("user2").unwrap(), "password456");
        
        // Test empty collections
        assert!(state_guard.data.is_empty());
        assert!(state_guard.refresh_tokens.is_empty());
        assert!(state_guard.wasm_cache.is_empty());
        
        // Test metrics initialization
        assert_eq!(state_guard.metrics.total_executions.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(state_guard.metrics.successful_executions.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(state_guard.metrics.failed_executions.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert!(state_guard.metrics.function_counts.is_empty());
        
        // Test rate limiter initialization
        assert_eq!(state_guard.rate_limiter.max_requests, 100);
        assert_eq!(state_guard.rate_limiter.window_duration.as_secs(), 60);
        assert!(state_guard.rate_limiter.user_requests.is_empty());
    }

    #[test]
    fn test_metrics_default() {
        let metrics = Metrics::default();
        
        assert_eq!(metrics.total_executions.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(metrics.successful_executions.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(metrics.failed_executions.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert!(metrics.function_counts.is_empty());
    }

    #[test]
    fn test_rate_limiter_default() {
        let rate_limiter = RateLimiter::default();
        
        assert_eq!(rate_limiter.max_requests, 100);
        assert_eq!(rate_limiter.window_duration.as_secs(), 60);
        assert!(rate_limiter.user_requests.is_empty());
    }

    #[test]
    fn test_data_operations() {
        let state = new_state();
        
        // Test adding data
        {
            let mut state_guard = state.lock().unwrap();
            let entry = DataEntry {
                func_names: vec!["add".to_string(), "mul".to_string()],
                bytecode: vec![1, 2, 3, 4, 5],
                owner: "test_user".to_string(),
            };
            state_guard.data.insert(1, entry);
            assert_eq!(state_guard.data.len(), 1);
        }
        
        // Test reading data
        {
            let state_guard = state.lock().unwrap();
            assert!(state_guard.data.contains_key(&1));
            let entry = state_guard.data.get(&1).unwrap();
            assert_eq!(entry.owner, "test_user");
            assert_eq!(entry.func_names, vec!["add", "mul"]);
            assert_eq!(entry.bytecode, vec![1, 2, 3, 4, 5]);
        }
        
        // Test updating data
        {
            let mut state_guard = state.lock().unwrap();
            let updated_entry = DataEntry {
                func_names: vec!["add".to_string(), "sub".to_string()],
                bytecode: vec![10, 20, 30],
                owner: "test_user".to_string(),
            };
            state_guard.data.insert(1, updated_entry);
            assert_eq!(state_guard.data.len(), 1);
            
            let entry = state_guard.data.get(&1).unwrap();
            assert_eq!(entry.func_names, vec!["add", "sub"]);
            assert_eq!(entry.bytecode, vec![10, 20, 30]);
        }
        
        // Test deleting data
        {
            let mut state_guard = state.lock().unwrap();
            state_guard.data.remove(&1);
            assert!(state_guard.data.is_empty());
        }
    }

    #[test]
    fn test_wasm_cache_operations() {
        let state = new_state();
        
        // Test cache insertion
        {
            let mut state_guard = state.lock().unwrap();
            state_guard.wasm_cache.insert(1, vec![1, 2, 3, 4, 5]);
            assert!(state_guard.wasm_cache.contains_key(&1));
            assert_eq!(state_guard.wasm_cache.get(&1).unwrap().len(), 5);
        }
        
        // Test cache retrieval
        {
            let state_guard = state.lock().unwrap();
            let cached = state_guard.wasm_cache.get(&1).unwrap();
            assert_eq!(cached, &vec![1, 2, 3, 4, 5]);
        }
        
        // Test cache update
        {
            let mut state_guard = state.lock().unwrap();
            state_guard.wasm_cache.insert(1, vec![10, 20, 30]);
            let cached = state_guard.wasm_cache.get(&1).unwrap();
            assert_eq!(cached, &vec![10, 20, 30]);
        }
        
        // Test cache removal
        {
            let mut state_guard = state.lock().unwrap();
            state_guard.wasm_cache.remove(&1);
            assert!(!state_guard.wasm_cache.contains_key(&1));
        }
    }

    #[test]
    fn test_metrics_operations() {
        let state = new_state();
        
        // Test incrementing metrics
        {
            let mut state_guard = state.lock().unwrap();
            state_guard.metrics.total_executions.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            state_guard.metrics.successful_executions.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            state_guard.metrics.function_counts
                .entry("add".to_string())
                .or_insert_with(|| std::sync::atomic::AtomicU64::new(0))
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        
        // Test reading metrics
        {
            let state_guard = state.lock().unwrap();
            assert_eq!(state_guard.metrics.total_executions.load(std::sync::atomic::Ordering::Relaxed), 1);
            assert_eq!(state_guard.metrics.successful_executions.load(std::sync::atomic::Ordering::Relaxed), 1);
            assert_eq!(state_guard.metrics.failed_executions.load(std::sync::atomic::Ordering::Relaxed), 0);
            
            let add_count = state_guard.metrics.function_counts.get("add").unwrap();
            assert_eq!(add_count.load(std::sync::atomic::Ordering::Relaxed), 1);
        }
    }

    #[test]
    fn test_refresh_tokens_operations() {
        let state = new_state();
        
        // Test adding refresh token
        {
            let mut state_guard = state.lock().unwrap();
            let token_info = RefreshTokenInfo {
                username: "test_user".to_string(),
                expires_at: chrono::Utc::now() + chrono::Duration::days(30),
            };
            state_guard.refresh_tokens.insert("test_token".to_string(), token_info);
            assert!(state_guard.refresh_tokens.contains_key("test_token"));
        }
        
        // Test retrieving refresh token
        {
            let state_guard = state.lock().unwrap();
            let token_info = state_guard.refresh_tokens.get("test_token").unwrap();
            assert_eq!(token_info.username, "test_user");
            assert!(token_info.expires_at > chrono::Utc::now());
        }
        
        // Test removing refresh token
        {
            let mut state_guard = state.lock().unwrap();
            state_guard.refresh_tokens.remove("test_token");
            assert!(!state_guard.refresh_tokens.contains_key("test_token"));
        }
    }

    #[test]
    fn test_concurrent_access() {
        use std::thread;
        use std::sync::Arc;
        
        let state = new_state();
        let state_clone = state.clone();
        
        // Spawn multiple threads to test concurrent access
        let handles: Vec<_> = (0..10).map(|i| {
            let state_clone = state_clone.clone();
            thread::spawn(move || {
                let mut state_guard = state_clone.lock().unwrap();
                let entry = DataEntry {
                    func_names: vec![format!("func_{}", i)],
                    bytecode: vec![i as u8],
                    owner: format!("user_{}", i),
                };
                state_guard.data.insert(i, entry);
            })
        }).collect();
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify all data was added correctly
        let state_guard = state.lock().unwrap();
        assert_eq!(state_guard.data.len(), 10);
        
        for i in 0..10 {
            assert!(state_guard.data.contains_key(&i));
            let entry = state_guard.data.get(&i).unwrap();
            assert_eq!(entry.func_names, vec![format!("func_{}", i)]);
            assert_eq!(entry.bytecode, vec![i as u8]);
            assert_eq!(entry.owner, format!("user_{}", i));
        }
    }
}
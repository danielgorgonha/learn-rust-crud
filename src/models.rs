// This struct represents a data record in our CRUD.
// It will be automatically converted to JSON using Serde.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataEntry {
    pub data1: Vec<String>, // List of texts
    pub data2: Vec<u8>,     // List of integers (bytes)
    pub owner: String,      // Record owner
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub username: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateDataRequest {
    pub data1: Vec<String>,
    pub data2: Vec<u8>,
}

// JWT Claims structure for access tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (username)
    pub exp: i64,          // Expiration time
    pub iat: i64,          // Issued at
    pub iss: String,       // Issuer
    pub token_type: String, // "access" or "refresh"
}

// Refresh token storage structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenInfo {
    pub username: String,
    pub expires_at: i64,
}
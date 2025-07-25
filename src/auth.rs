use crate::models::{AuthRequest, AuthResponse, Claims, CreateDataRequest, DataEntry, RefreshRequest, RefreshTokenInfo};
use crate::state::AppState;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use tide::Request;
use std::env;

// JWT Configuration - Get from environment variables with defaults
fn get_jwt_secret() -> Vec<u8> {
    env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string())
        .into_bytes()
}

fn get_jwt_issuer() -> String {
    env::var("JWT_ISSUER")
        .unwrap_or_else(|_| "learn-rust-crud".to_string())
}

fn get_access_token_expiration_hours() -> i64 {
    env::var("ACCESS_TOKEN_EXPIRATION_HOURS")
        .unwrap_or_else(|_| "1".to_string())
        .parse()
        .unwrap_or(1)
}

fn get_refresh_token_expiration_days() -> i64 {
    env::var("REFRESH_TOKEN_EXPIRATION_DAYS")
        .unwrap_or_else(|_| "30".to_string())
        .parse()
        .unwrap_or(30)
}

// Function to authenticate user and generate JWT tokens
pub async fn login(mut req: Request<AppState>) -> tide::Result {
    let auth_req: AuthRequest = req.body_json().await?;
    let state = req.state();
    let mut app_state = state.lock().unwrap();

    // Check if user exists and password is correct
    if let Some(stored_password) = app_state.users.get(&auth_req.username) {
        if stored_password == &auth_req.password {
            // Generate access and refresh tokens
            let access_token = generate_access_token(&auth_req.username)?;
            let refresh_token = generate_refresh_token(&auth_req.username)?;
            
            // Store refresh token
            let refresh_info = RefreshTokenInfo {
                username: auth_req.username.clone(),
                expires_at: (Utc::now() + Duration::days(get_refresh_token_expiration_days())).timestamp(),
            };
            app_state.refresh_tokens.insert(refresh_token.clone(), refresh_info);
            
            let response = AuthResponse {
                access_token,
                refresh_token,
                username: auth_req.username,
                token_type: "Bearer".to_string(),
                expires_in: get_access_token_expiration_hours() * 3600, // Convert hours to seconds
            };

            Ok(tide::Body::from_json(&response)?.into())
        } else {
            Err(tide::Error::from_str(401, "Invalid credentials"))
        }
    } else {
        Err(tide::Error::from_str(401, "User not found"))
    }
}

// Function to refresh access token
pub async fn refresh(mut req: Request<AppState>) -> tide::Result {
    let refresh_req: RefreshRequest = req.body_json().await?;
    let state = req.state();
    let mut app_state = state.lock().unwrap();

    // Check if refresh token exists and is valid
    if let Some(refresh_info) = app_state.refresh_tokens.get(&refresh_req.refresh_token) {
        // Check if refresh token is expired
        if refresh_info.expires_at > Utc::now().timestamp() {
            // Generate new access token
            let new_access_token = generate_access_token(&refresh_info.username)?;
            
            let response = AuthResponse {
                access_token: new_access_token,
                refresh_token: refresh_req.refresh_token, // Keep the same refresh token
                username: refresh_info.username.clone(),
                token_type: "Bearer".to_string(),
                expires_in: get_access_token_expiration_hours() * 3600,
            };

            Ok(tide::Body::from_json(&response)?.into())
        } else {
            // Remove expired refresh token
            app_state.refresh_tokens.remove(&refresh_req.refresh_token);
            Err(tide::Error::from_str(401, "Refresh token expired"))
        }
    } else {
        Err(tide::Error::from_str(401, "Invalid refresh token"))
    }
}

// Function to logout (remove refresh token)
pub async fn logout(mut req: Request<AppState>) -> tide::Result {
    let refresh_req: RefreshRequest = req.body_json().await?;
    let state = req.state();
    let mut app_state = state.lock().unwrap();
    
    // Remove refresh token
    app_state.refresh_tokens.remove(&refresh_req.refresh_token);
    
    Ok(tide::Response::new(200))
}

// Helper function to check if user is authenticated
pub fn get_authenticated_user(req: &Request<AppState>) -> Result<String, tide::Error> {
    if let Some(auth_header) = req.header("Authorization") {
        // Handle the header properly - it might be an array or single value
        let header_str = auth_header.to_string();
        let token = if header_str.starts_with('[') && header_str.ends_with(']') {
            // Remove brackets and quotes if present
            header_str[1..header_str.len()-1].replace("\"", "").replace("Bearer ", "")
        } else {
            header_str.replace("Bearer ", "")
        };
        
        // Decode and validate access token
        match decode_access_token(&token) {
            Ok(claims) => {
                if claims.token_type == "access" {
                    Ok(claims.sub)
                } else {
                    Err(tide::Error::from_str(401, "Invalid token type"))
                }
            },
            Err(_) => Err(tide::Error::from_str(401, "Invalid token")),
        }
    } else {
        Err(tide::Error::from_str(401, "Missing authorization header"))
    }
}

// Function to convert CreateDataRequest to DataEntry
pub fn create_data_entry_from_request(req_data: CreateDataRequest, owner: String) -> DataEntry {
    DataEntry {
        data1: req_data.data1,
        data2: req_data.data2,
        owner,
    }
}

// Generate access JWT token
fn generate_access_token(username: &str) -> Result<String, tide::Error> {
    let now = Utc::now();
    let expires_at = now + Duration::hours(get_access_token_expiration_hours());
    
    let claims = Claims {
        sub: username.to_string(),
        exp: expires_at.timestamp(),
        iat: now.timestamp(),
        iss: get_jwt_issuer(),
        token_type: "access".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_slice()),
    )
    .map_err(|_| tide::Error::from_str(500, "Failed to generate access token"))
}

// Generate refresh JWT token
fn generate_refresh_token(username: &str) -> Result<String, tide::Error> {
    let now = Utc::now();
    let expires_at = now + Duration::days(get_refresh_token_expiration_days());
    
    let claims = Claims {
        sub: username.to_string(),
        exp: expires_at.timestamp(),
        iat: now.timestamp(),
        iss: get_jwt_issuer(),
        token_type: "refresh".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_slice()),
    )
    .map_err(|_| tide::Error::from_str(500, "Failed to generate refresh token"))
}

// Decode and validate access JWT token
fn decode_access_token(token: &str) -> Result<Claims, tide::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_jwt_secret().as_slice()),
        &Validation::default(),
    )
    .map_err(|_| tide::Error::from_str(401, "Invalid token"))?;

    Ok(token_data.claims)
} 
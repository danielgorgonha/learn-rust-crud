mod common;
use common::*;

#[async_std::test]
async fn test_login_success() {
    println!("\n🧪 Test: Login with valid credentials");
    let (base_url, child) = start_test_server();
    let login_data = LoginRequest {
        username: "admin".to_string(),
        password: "admin123".to_string(),
    };
    let response = ureq::post(&format!("{}/auth/login", base_url))
        .send_json(ureq::json!(login_data))
        .expect("❌ Request failed");
    assert_eq!(response.status(), 200, "❌ Status code should be 200");
    let login_response: LoginResponse = response.into_json().expect("❌ Failed to parse JSON");
    assert_eq!(login_response.username, "admin", "❌ Incorrect username");
    assert_eq!(login_response.token_type, "Bearer", "❌ Incorrect token type");
    assert_eq!(login_response.expires_in, 3600, "❌ Incorrect expiration");
    assert!(!login_response.access_token.is_empty(), "❌ Empty access token");
    assert!(!login_response.refresh_token.is_empty(), "❌ Empty refresh token");
    println!("✅ Login successful - Token: {}...", &login_response.access_token[..20]);
    stop_test_server(child);
}

#[async_std::test]
async fn test_login_invalid_credentials() {
    println!("\n🧪 Test: Login with invalid credentials");
    let (base_url, child) = start_test_server();
    let login_data = LoginRequest {
        username: "admin".to_string(),
        password: "wrong_password".to_string(),
    };
    
    // For invalid credentials, we expect status 401
    match ureq::post(&format!("{}/auth/login", base_url))
        .send_json(ureq::json!(login_data)) {
        Ok(response) => {
            // If we received a response, it should be 401
            assert_eq!(response.status(), 401, "❌ Status code should be 401");
        }
        Err(e) => {
            // If we received an error, it should be status 401
            if e.to_string().contains("status code 401") {
                println!("✅ Login with invalid credentials correctly rejected (401)");
            } else {
                panic!("❌ Unexpected error: {}", e);
            }
        }
    }
    
    println!("✅ Login with invalid credentials correctly rejected");
    stop_test_server(child);
} 
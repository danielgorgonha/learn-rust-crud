mod common;
use common::*;

fn login_and_get_token(base_url: &str) -> String {
    let login_data = LoginRequest {
        username: "admin".to_string(),
        password: "admin123".to_string(),
    };
    let response = ureq::post(&format!("{}/auth/login", base_url))
        .send_json(ureq::json!(login_data))
        .expect("❌ Login request failed");
    assert_eq!(response.status(), 200, "❌ Login failed");
    let login_response: LoginResponse = response.into_json().expect("❌ Failed to parse response");
    login_response.access_token
}

#[async_std::test]
async fn test_complete_crud_flow() {
    println!("\n🧪 Test: Complete CRUD flow");
    let (base_url, child) = start_test_server();
    // 1. Login
    let token = login_and_get_token(&base_url);
    // 2. Create
    let test_data = TestData {
        func_names: vec!["add".to_string(), "mul".to_string()],
        bytecode: vec![1, 2, 3, 4, 5],
    };
    let create_response = ureq::post(&format!("{}/data", base_url))
        .set("Authorization", &format!("Bearer {}", token))
        .send_json(ureq::json!(test_data))
        .expect("❌ Failed to create data");
    assert_eq!(create_response.status(), 200, "❌ Failed to create data");
    let create_data: serde_json::Value = create_response.into_json().expect("❌ Failed to parse response");
    let data_id = create_data["id"].as_u64().expect("❌ ID not found");
    // 3. Read
    let read_response = ureq::get(&format!("{}/data/{}", base_url, data_id))
        .set("Authorization", &format!("Bearer {}", token))
        .call()
        .expect("❌ Failed to read data");
    assert_eq!(read_response.status(), 200, "❌ Failed to read data");
    let read_data: serde_json::Value = read_response.into_json().expect("❌ Failed to parse response");
    assert_eq!(read_data["owner"], "admin", "❌ Incorrect owner");
    assert!(read_data["func_names"].is_array(), "❌ func_names is not an array");
    assert!(read_data["bytecode"].is_array(), "❌ bytecode is not an array");
    // 4. Update
    let update_data = TestData {
        func_names: vec!["add".to_string(), "mul".to_string(), "sub".to_string()],
        bytecode: vec![1, 2, 3, 4, 5, 6, 7],
    };
    let update_response = ureq::put(&format!("{}/data/{}", base_url, data_id))
        .set("Authorization", &format!("Bearer {}", token))
        .send_json(ureq::json!(update_data))
        .expect("❌ Failed to update data");
    assert_eq!(update_response.status(), 200, "❌ Failed to update data");
    // 5. Delete
    let delete_response = ureq::delete(&format!("{}/data/{}", base_url, data_id))
        .set("Authorization", &format!("Bearer {}", token))
        .call()
        .expect("❌ Failed to delete data");
    assert_eq!(delete_response.status(), 204, "❌ Failed to delete data - expected 204 (No Content)");
    // 6. Verify deletion
    match ureq::get(&format!("{}/data/{}", base_url, data_id))
        .set("Authorization", &format!("Bearer {}", token))
        .call() {
        Ok(response) => {
            // If we received a response, it should be 404
            assert_eq!(response.status(), 404, "❌ Data still exists - expected 404");
        }
        Err(e) => {
            // If we received an error, it should be status 404
            if e.to_string().contains("status code 404") {
                println!("✅ Data correctly deleted (404)");
            } else {
                panic!("❌ Unexpected error when verifying deletion: {}", e);
            }
        }
    }
    println!("✅ Complete CRUD flow tested successfully");
    stop_test_server(child);
} 
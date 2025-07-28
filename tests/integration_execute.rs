mod common;
use common::*;
use serial_test::serial;

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

fn create_wasm_record(base_url: &str, token: &str) -> u32 {
    // Real WASM bytecode from the math library
    let wasm_bytecode = vec![
        0,97,115,109,1,0,0,0,1,31,6,96,2,127,127,0,96,2,127,127,1,127,96,0,0,96,1,127,1,127,96,1,127,0,96,4,127,127,127,127,0,3,20,19,1,1,1,1,2,1,2,3,1,1,1,0,4,0,5,2,4,4,0,4,5,1,112,1,3,3,5,3,1,0,17,6,25,3,127,1,65,128,128,192,0,11,127,0,65,153,129,192,0,11,127,0,65,160,129,192,0,11,7,91,12,6,109,101,109,111,114,121,2,0,3,97,100,100,0,0,3,109,117,108,0,1,3,115,117,98,0,2,3,100,105,118,0,3,3,114,101,109,0,5,3,97,98,115,0,7,3,109,97,120,0,8,3,109,105,110,0,9,3,112,111,119,0,10,10,95,95,100,97,116,97,95,101,110,100,3,1,11,95,95,104,101,97,112,95,98,97,115,101,3,2,9,8,1,0,65,1,11,2,13,18,10,254,6,19,7,0,32,1,32,0,106,11,7,0,32,1,32,0,108,11,15,0,65,0,32,0,32,1,107,32,0,32,1,72,27,11,49,0,2,64,32,1,13,0,65,0,15,11,2,64,2,64,32,0,65,128,128,128,128,120,71,13,0,32,1,65,127,70,13,1,11,32,0,32,1,109,15,11,16,132,128,128,128,0,0,11,71,1,1,127,35,128,128,128,128,0,65,32,107,34,0,36,128,128,128,128,0,32,0,65,0,54,2,24,32,0,65,1,54,2,12,32,0,65,204,128,192,128,0,54,2,8,32,0,66,4,55,2,16,32,0,65,8,106,65,140,128,192,128,0,16,139,128,128,128,0,0,11,49,0,2,64,32,1,13,0,65,0,15,11,2,64,2,64,32,0,65,128,128,128,128,120,71,13,0,32,1,65,127,70,13,1,11,32,0,32,1,111,15,11,16,134,128,128,128,0,0,11,71,1,1,127,35,128,128,128,128,0,65,32,107,34,0,36,128,128,128,128,0,32,0,65,0,54,2,24,32,0,65,1,54,2,12,32,0,65,132,129,192,128,0,54,2,8,32,0,66,4,55,2,16,32,0,65,8,106,65,156,128,192,128,0,16,139,128,128,128,0,0,11,17,1,1,127,32,0,32,0,65,31,117,34,1,115,32,1,107,11,12,0,32,0,32,1,32,0,32,1,74,27,11,12,0,32,0,32,1,32,0,32,1,72,27,11,78,1,1,127,65,0,33,2,2,64,32,1,65,0,72,13,0,65,1,33,2,2,64,2,64,32,1,14,2,2,0,1,11,32,0,33,2,12,1,11,32,1,65,127,106,33,1,32,0,33,2,3,64,32,1,69,13,1,32,1,65,127,106,33,1,32,2,32,0,108,33,2,12,0,11,11,32,2,11,54,1,1,127,35,128,128,128,128,0,65,16,107,34,2,36,128,128,128,128,0,32,2,65,1,59,1,12,32,2,32,1,54,2,8,32,2,32,0,54,2,4,32,2,65,4,106,16,140,128,128,128,0,0,11,56,2,1,127,1,126,35,128,128,128,128,0,65,16,107,34,1,36,128,128,128,128,0,32,0,41,2,0,33,2,32,1,32,0,54,2,12,32,1,32,2,55,2,4,32,1,65,4,106,16,144,128,128,128,0,0,11,9,0,32,0,65,0,54,2,0,11,153,1,1,2,127,35,128,128,128,128,0,65,16,107,34,4,36,128,128,128,128,0,65,0,65,0,40,2,144,129,192,128,0,34,5,65,1,106,54,2,144,129,192,128,0,2,64,32,5,65,0,72,13,0,2,64,2,64,65,0,45,0,152,129,192,128,0,13,0,65,0,65,0,40,2,148,129,192,128,0,65,1,106,54,2,148,129,192,128,0,65,0,40,2,140,129,192,128,0,65,127,74,13,1,12,2,11,32,4,65,8,106,32,0,32,1,17,128,128,128,128,0,128,128,128,128,0,0,11,65,0,65,0,58,0,152,129,192,128,0,32,2,69,13,0,16,143,128,128,128,0,0,11,0,11,3,0,0,11,11,0,32,0,16,145,128,128,128,0,0,11,186,1,1,3,127,35,128,128,128,128,0,65,16,107,34,1,36,128,128,128,128,0,32,0,40,2,0,34,2,40,2,12,33,3,2,64,2,64,2,64,2,64,32,2,40,2,4,14,2,0,1,2,11,32,3,13,1,65,1,33,2,65,0,33,3,12,2,11,32,3,13,0,32,2,40,2,0,34,2,40,2,4,33,3,32,2,40,2,0,33,2,12,1,11,32,1,65,128,128,128,128,120,54,2,0,32,1,32,0,54,2,12,32,1,65,129,128,128,128,0,32,0,40,2,8,34,0,45,0,8,32,0,45,0,9,16,142,128,128,128,0,0,11,32,1,32,3,54,2,4,32,1,32,2,54,2,0,32,1,65,130,128,128,128,0,32,0,40,2,8,34,0,45,0,8,32,0,45,0,9,16,142,128,128,128,0,0,11,12,0,32,0,32,1,41,2,0,55,3,0,11,11,150,1,1,0,65,128,128,192,0,11,140,1,115,114,99,47,108,105,98,46,114,115,0,0,0,0,16,0,10,0,0,0,61,0,0,0,5,0,0,0,0,0,16,0,10,0,0,0,77,0,0,0,5,0,0,0,97,116,116,101,109,112,116,32,116,111,32,100,105,118,105,100,101,32,119,105,116,104,32,111,118,101,114,102,108,111,119,0,44,0,16,0,31,0,0,0,97,116,116,101,109,112,116,32,116,111,32,99,97,108,99,117,108,97,116,101,32,116,104,101,32,114,101,109,97,105,110,100,101,114,32,119,105,116,104,32,111,118,101,114,102,108,111,119,84,0,16,0,48,0,0,0,0,168,6,4,110,97,109,101,0,10,9,109,97,116,104,46,119,97,115,109,1,244,5,19,0,3,97,100,100,1,3,109,117,108,2,3,115,117,98,3,3,100,105,118,4,77,95,90,78,52,99,111,114,101,57,112,97,110,105,99,107,105,110,103,49,49,112,97,110,105,99,95,99,111,110,115,116,50,52,112,97,110,105,99,95,99,111,110,115,116,95,100,105,118,95,111,118,101,114,102,108,111,119,49,55,104,54,55,100,51,54,49,97,55,48,53,50,56,50,98,53,49,69,5,3,114,101,109,6,77,95,90,78,52,99,111,114,101,57,112,97,110,105,99,107,105,110,103,49,49,112,97,110,105,99,95,99,111,110,115,116,50,52,112,97,110,105,99,95,99,111,110,115,116,95,114,101,109,95,111,118,101,114,102,108,111,119,49,55,104,52,49,102,54,99,102,52,101,52,55,55,49,97,98,51,52,69,7,3,97,98,115,8,3,109,97,120,9,3,109,105,110,10,3,112,111,119,11,48,95,90,78,52,99,111,114,101,57,112,97,110,105,99,107,105,110,103,57,112,97,110,105,99,95,102,109,116,49,55,104,52,49,99,102,101,100,55,57,98,50,100,100,98,102,49,51,69,12,46,95,82,78,118,67,115,54,57,49,114,104,84,98,71,48,69,101,95,55,95,95,95,114,117,115,116,99,49,55,114,117,115,116,95,98,101,103,105,110,95,117,110,119,105,110,100,13,55,95,90,78,52,99,111,114,101,53,112,97,110,105,99,49,50,80,97,110,105,99,80,97,121,108,111,97,100,54,97,115,95,115,116,114,49,55,104,51,53,55,53,101,101,53,55,50,101,53,49,49,56,53,53,69,14,59,95,90,78,51,115,116,100,57,112,97,110,105,99,107,105,110,103,50,48,114,117,115,116,95,112,97,110,105,99,95,119,105,116,104,95,104,111,111,107,49,55,104,99,50,55,54,100,48,53,48,49,97,100,53,98,57,53,52,69,15,39,95,82,78,118,67,115,54,57,49,114,104,84,98,71,48,69,101,95,55,95,95,95,114,117,115,116,99,49,48,114,117,115,116,95,112,97,110,105,99,16,69,95,90,78,51,115,116,100,51,115,121,115,57,98,97,99,107,116,114,97,99,101,50,54,95,95,114,117,115,116,95,101,110,100,95,115,104,111,114,116,95,98,97,99,107,116,114,97,99,101,49,55,104,49,54,97,98,55,50,55,54,53,98,51,50,50,56,50,100,69,17,88,95,90,78,51,115,116,100,57,112,97,110,105,99,107,105,110,103,49,57,98,101,103,105,110,95,112,97,110,105,99,95,104,97,110,100,108,101,114,50,56,95,36,117,55,98,36,36,117,55,98,36,99,108,111,115,117,114,101,36,117,55,100,36,36,117,55,100,36,49,55,104,50,51,102,102,52,49,54,97,57,50,49,52,54,56,98,52,69,18,131,1,95,90,78,57,57,95,36,76,84,36,115,116,100,46,46,112,97,110,105,99,107,105,110,103,46,46,98,101,103,105,110,95,112,97,110,105,99,95,104,97,110,100,108,101,114,46,46,83,116,97,116,105,99,83,116,114,80,97,121,108,111,97,100,36,117,50,48,36,97,115,36,117,50,48,36,99,111,114,101,46,46,112,97,110,105,99,46,46,80,97,110,105,99,80,97,121,108,111,97,100,36,71,84,36,54,97,115,95,115,116,114,49,55,104,52,98,51,97,100,49,98,50,56,54,102,52,49,54,51,97,69,7,18,1,0,15,95,95,115,116,97,99,107,95,112,111,105,110,116,101,114,9,10,1,0,7,46,114,111,100,97,116,97,0,77,9,112,114,111,100,117,99,101,114,115,2,8,108,97,110,103,117,97,103,101,1,4,82,117,115,116,0,12,112,114,111,99,101,115,115,101,100,45,98,121,1,5,114,117,115,116,99,29,49,46,56,56,46,48,32,40,54,98,48,48,98,99,51,56,56,32,50,48,50,53,45,48,54,45,50,51,41,0,148,1,15,116,97,114,103,101,116,95,102,101,97,116,117,114,101,115,8,43,11,98,117,108,107,45,109,101,109,111,114,121,43,15,98,117,108,107,45,109,101,109,111,114,121,45,111,112,116,43,22,99,97,108,108,45,105,110,100,105,114,101,99,116,45,111,118,101,114,108,111,110,103,43,10,109,117,108,116,105,118,97,108,117,101,43,15,109,117,116,97,98,108,101,45,103,108,111,98,97,108,115,43,19,110,111,110,116,114,97,112,112,105,110,103,45,102,112,116,111,105,110,116,43,15,114,101,102,101,114,101,110,99,101,45,116,121,112,101,115,43,8,115,105,103,110,45,101,120,116
    ];
    
    let test_data = TestData {
        func_names: vec!["add".to_string(), "mul".to_string(), "sub".to_string(), "div".to_string(), "rem".to_string(), "abs".to_string(), "max".to_string(), "min".to_string(), "pow".to_string()],
        bytecode: wasm_bytecode,
    };
    
    let create_response = ureq::post(&format!("{}/data", base_url))
        .set("Authorization", &format!("Bearer {}", token))
        .send_json(ureq::json!(test_data))
        .expect("❌ Failed to create WASM record");
    
    assert_eq!(create_response.status(), 200, "❌ Failed to create WASM record");
    let create_data: serde_json::Value = create_response.into_json().expect("❌ Failed to parse response");
    create_data["id"].as_u64().expect("❌ ID not found") as u32
}

#[derive(serde::Deserialize)]
struct ExecuteResponse {
    success: bool,
    result: Option<i32>,
    error: Option<String>,
    function: String,
    operands: [i32; 2],
    owner: String,
}

#[async_std::test]
#[serial]
async fn test_wasm_execute_success() {
    println!("\n🧪 Test: WASM execute - successful execution");
    let (base_url, child) = start_test_server();
    
    // Login and get token
    let token = login_and_get_token(&base_url);
    
    // Create WASM record
    let record_id = create_wasm_record(&base_url, &token);
    println!("📝 Created WASM record with ID: {}", record_id);
    
    // Test successful execution
    let execute_data = serde_json::json!({
        "fn": "add",
        "arg": [10, 20]
    });
    
    let execute_response = ureq::post(&format!("{}/execute/{}", base_url, record_id))
        .set("Authorization", &format!("Bearer {}", token))
        .send_json(execute_data)
        .expect("❌ Failed to execute WASM function");
    
    assert_eq!(execute_response.status(), 200, "❌ WASM execution failed");
    
    let response_data: ExecuteResponse = execute_response.into_json().expect("❌ Failed to parse response");
    assert!(response_data.success, "❌ Execution should be successful");
    assert_eq!(response_data.result, Some(30), "❌ Expected result 30, got {:?}", response_data.result);
    assert_eq!(response_data.function, "add", "❌ Function name mismatch");
    assert_eq!(response_data.operands, [10, 20], "❌ Operands mismatch");
    assert_eq!(response_data.owner, "admin", "❌ Owner mismatch");
    assert!(response_data.error.is_none(), "❌ Should not have error");
    
    println!("✅ WASM execution successful - Result: {}", response_data.result.unwrap());
    stop_test_server(child);
}

#[async_std::test]
#[serial]
async fn test_wasm_execute_multiple_functions() {
    println!("\n🧪 Test: WASM execute - multiple functions");
    let (base_url, child) = start_test_server();
    
    // Login and get token
    let token = login_and_get_token(&base_url);
    
    // Create WASM record
    let record_id = create_wasm_record(&base_url, &token);
    
    // Test different functions
    let test_cases = vec![
        ("add", [15, 25], 40),
        ("mul", [6, 7], 42),
        ("sub", [20, 8], 12),
        ("div", [100, 5], 20),
    ];
    
    for (func_name, args, expected_result) in test_cases {
        println!("  📊 Testing {} function with args {:?}...", func_name, args);
        
        let execute_data = serde_json::json!({
            "fn": func_name,
            "arg": args
        });
        
        let execute_response = ureq::post(&format!("{}/execute/{}", base_url, record_id))
            .set("Authorization", &format!("Bearer {}", token))
            .send_json(execute_data)
            .expect("❌ Failed to execute WASM function");
        
        assert_eq!(execute_response.status(), 200, "❌ WASM execution failed for {}", func_name);
        
        let response_data: ExecuteResponse = execute_response.into_json().expect("❌ Failed to parse response");
        assert!(response_data.success, "❌ Execution should be successful for {}", func_name);
        assert_eq!(response_data.result, Some(expected_result), 
                   "❌ Expected result {} for {}, got {:?}", expected_result, func_name, response_data.result);
        
        println!("    ✅ {} function result: {}", func_name, response_data.result.unwrap());
    }
    
    println!("✅ All WASM functions executed successfully");
    stop_test_server(child);
}

#[async_std::test]
#[serial]
async fn test_wasm_execute_invalid_function() {
    println!("\n🧪 Test: WASM execute - invalid function");
    let (base_url, child) = start_test_server();
    
    // Login and get token
    let token = login_and_get_token(&base_url);
    
    // Create WASM record
    let record_id = create_wasm_record(&base_url, &token);
    
    // Test invalid function
    let execute_data = serde_json::json!({
        "fn": "invalid_function",
        "arg": [10, 20]
    });
    
    match ureq::post(&format!("{}/execute/{}", base_url, record_id))
        .set("Authorization", &format!("Bearer {}", token))
        .send_json(execute_data) {
        Ok(response) => {
            // If we received a response, it should be 400
            assert_eq!(response.status(), 400, "❌ Should return 400 for invalid function");
            let error_text = response.into_string().expect("❌ Failed to read response");
            assert!(error_text.contains("not allowed"), "❌ Should mention function not allowed");
        }
        Err(e) => {
            // If we received an error, it should be status 400
            if e.to_string().contains("status code 400") {
                println!("✅ Invalid function correctly rejected (400)");
            } else {
                panic!("❌ Unexpected error: {}", e);
            }
        }
    }
    
    println!("✅ Invalid function correctly rejected");
    stop_test_server(child);
}

#[async_std::test]
#[serial]
async fn test_wasm_execute_unauthorized_access() {
    println!("\n🧪 Test: WASM execute - unauthorized access");
    let (base_url, child) = start_test_server();
    
    // Login with admin and create record
    let admin_token = login_and_get_token(&base_url);
    let record_id = create_wasm_record(&base_url, &admin_token);
    
    // Login with different user
    let user2_login = LoginRequest {
        username: "user1".to_string(),
        password: "password123".to_string(),
    };
    let user2_response = ureq::post(&format!("{}/auth/login", base_url))
        .send_json(ureq::json!(user2_login))
        .expect("❌ User2 login failed");
    let user2_token: LoginResponse = user2_response.into_json().expect("❌ Failed to parse user2 response");
    
    // Try to execute with different user's token
    let execute_data = serde_json::json!({
        "fn": "add",
        "arg": [10, 20]
    });
    
    match ureq::post(&format!("{}/execute/{}", base_url, record_id))
        .set("Authorization", &format!("Bearer {}", user2_token.access_token))
        .send_json(execute_data) {
        Ok(response) => {
            // If we received a response, it should be 403
            assert_eq!(response.status(), 403, "❌ Should return 403 for unauthorized access");
            let error_text = response.into_string().expect("❌ Failed to read response");
            assert!(error_text.contains("Access denied"), "❌ Should mention access denied");
        }
        Err(e) => {
            // If we received an error, it should be status 403
            if e.to_string().contains("status code 403") {
                println!("✅ Unauthorized access correctly rejected (403)");
            } else {
                panic!("❌ Unexpected error: {}", e);
            }
        }
    }
    
    println!("✅ Unauthorized access correctly rejected");
    stop_test_server(child);
}

#[async_std::test]
#[serial]
async fn test_wasm_execute_record_not_found() {
    println!("\n🧪 Test: WASM execute - record not found");
    let (base_url, child) = start_test_server();
    
    // Login and get token
    let token = login_and_get_token(&base_url);
    
    // Try to execute with non-existent record ID
    let execute_data = serde_json::json!({
        "fn": "add",
        "arg": [10, 20]
    });
    
    match ureq::post(&format!("{}/execute/999", base_url))
        .set("Authorization", &format!("Bearer {}", token))
        .send_json(execute_data) {
        Ok(response) => {
            // If we received a response, it should be 404
            assert_eq!(response.status(), 404, "❌ Should return 404 for non-existent record");
            let error_text = response.into_string().expect("❌ Failed to read response");
            assert!(error_text.contains("not found"), "❌ Should mention record not found");
        }
        Err(e) => {
            // If we received an error, it should be status 404
            if e.to_string().contains("status code 404") {
                println!("✅ Non-existent record correctly rejected (404)");
            } else {
                panic!("❌ Unexpected error: {}", e);
            }
        }
    }
    
    println!("✅ Non-existent record correctly rejected");
    stop_test_server(child);
}

#[async_std::test]
#[serial]
async fn test_wasm_execute_invalid_json() {
    println!("\n🧪 Test: WASM execute - invalid JSON");
    let (base_url, child) = start_test_server();
    
    // Login and get token
    let token = login_and_get_token(&base_url);
    
    // Create WASM record
    let record_id = create_wasm_record(&base_url, &token);
    
    // Test with invalid JSON
    match ureq::post(&format!("{}/execute/{}", base_url, record_id))
        .set("Authorization", &format!("Bearer {}", token))
        .set("Content-Type", "application/json")
        .send_string("invalid json") {
        Ok(response) => {
            // If we received a response, it should be 400
            assert_eq!(response.status(), 400, "❌ Should return 400 for invalid JSON");
            let error_text = response.into_string().expect("❌ Failed to read response");
            assert!(error_text.contains("Invalid JSON"), "❌ Should mention invalid JSON");
        }
        Err(e) => {
            // If we received an error, it should be status 400
            if e.to_string().contains("status code 400") {
                println!("✅ Invalid JSON correctly rejected (400)");
            } else {
                panic!("❌ Unexpected error: {}", e);
            }
        }
    }
    
    println!("✅ Invalid JSON correctly rejected");
    stop_test_server(child);
}

#[async_std::test]
#[serial]
async fn test_wasm_execute_missing_authentication() {
    println!("\n🧪 Test: WASM execute - missing authentication");
    let (base_url, child) = start_test_server();
    
    // Create WASM record with admin (we need a valid record)
    let admin_token = login_and_get_token(&base_url);
    let record_id = create_wasm_record(&base_url, &admin_token);
    
    // Try to execute without authentication
    let execute_data = serde_json::json!({
        "fn": "add",
        "arg": [10, 20]
    });
    
    match ureq::post(&format!("{}/execute/{}", base_url, record_id))
        .send_json(execute_data) {
        Ok(response) => {
            // If we received a response, it should be 401
            assert_eq!(response.status(), 401, "❌ Should return 401 for missing authentication");
        }
        Err(e) => {
            // If we received an error, it should be status 401
            if e.to_string().contains("status code 401") {
                println!("✅ Missing authentication correctly rejected (401)");
            } else {
                panic!("❌ Unexpected error: {}", e);
            }
        }
    }
    
    println!("✅ Missing authentication correctly rejected");
    stop_test_server(child);
} 
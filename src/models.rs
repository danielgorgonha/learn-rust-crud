// This struct represents a data record in our CRUD.
// It will be automatically converted to JSON using Serde.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataEntry {
    pub func_names: Vec<String>, // Lista de textos
    pub bytecode: Vec<u8>,       // Lista de números inteiros (bytes)
    pub owner: String,           // Record owner
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
    pub func_names: Vec<String>,
    pub bytecode: Vec<u8>,
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
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

// ===== WEBASSEMBLY MODELS =====

/// Request for executing WebAssembly operations
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WasmExecuteRequest {
    pub operation: String,           // "add", "mul", "sub", "div"
    pub operands: Vec<i32>,          // [10, 20] para add(10, 20)
    pub module_name: Option<String>, // Opcional: qual módulo usar
}

/// Response from WebAssembly operations
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WasmExecuteResponse {
    pub success: bool,               // Operação foi bem-sucedida?
    pub result: Option<i32>,         // Resultado da operação
    pub error: Option<String>,       // Mensagem de erro se houver
    pub operation: String,           // Operação executada
    pub operands: Vec<i32>,          // Operandos usados
}

/// Request for batch WebAssembly operations
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WasmBatchRequest {
    pub operations: Vec<WasmExecuteRequest>, // Múltiplas operações
}

/// Response for batch WebAssembly operations
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WasmBatchResponse {
    pub results: Vec<WasmExecuteResponse>,
    pub total_operations: usize,
    pub successful_operations: usize,
}

/// Request for data processing with WebAssembly
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WasmDataProcessRequest {
    pub data: Vec<i32>,              // Dados para processar
    pub operations: Vec<String>,     // Operações a aplicar
    pub owner: String,               // Quem solicitou
}

/// Response for data processing with WebAssembly
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WasmDataProcessResponse {
    pub original_data: Vec<i32>,
    pub processed_data: Vec<i32>,
    pub operations_applied: Vec<String>,
    pub processing_id: String,
    pub owner: String,
}

/// WebAssembly module information
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WasmModuleInfo {
    pub name: String,                // Nome do módulo
    pub available_functions: Vec<String>, // Funções disponíveis
    pub loaded: bool,                // Se está carregado
}

/// Request for loading a WebAssembly module
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WasmLoadModuleRequest {
    pub module_name: String,         // Nome do módulo
    pub module_data: String,         // Dados WASM em Base64
}

/// Response for loading a WebAssembly module
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WasmLoadModuleResponse {
    pub success: bool,
    pub module_name: String,
    pub error: Option<String>,
    pub available_functions: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_data_entry_serialization() {
        let entry = DataEntry {
            func_names: vec!["add".to_string(), "mul".to_string()],
            bytecode: vec![1, 2, 3, 4, 5],
            owner: "test_user".to_string(),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: DataEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(entry.func_names, deserialized.func_names);
        assert_eq!(entry.bytecode, deserialized.bytecode);
        assert_eq!(entry.owner, deserialized.owner);
    }

    #[test]
    fn test_create_data_request_serialization() {
        let request = CreateDataRequest {
            func_names: vec!["add".to_string(), "sub".to_string()],
            bytecode: vec![10, 20, 30, 40, 50],
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CreateDataRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request.func_names, deserialized.func_names);
        assert_eq!(request.bytecode, deserialized.bytecode);
    }

    #[test]
    fn test_login_request_serialization() {
        let request = AuthRequest {
            username: "test_user".to_string(),
            password: "test_pass".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: AuthRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request.username, deserialized.username);
        assert_eq!(request.password, deserialized.password);
    }

    #[test]
    fn test_login_response_serialization() {
        let response = AuthResponse {
            access_token: "access_token_123".to_string(),
            refresh_token: "refresh_token_456".to_string(),
            username: "test_user".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: AuthResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response.access_token, deserialized.access_token);
        assert_eq!(response.refresh_token, deserialized.refresh_token);
        assert_eq!(response.username, deserialized.username);
        assert_eq!(response.token_type, deserialized.token_type);
        assert_eq!(response.expires_in, deserialized.expires_in);
    }

    #[test]
    fn test_refresh_token_request_serialization() {
        let request = RefreshRequest {
            refresh_token: "refresh_token_123".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: RefreshRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request.refresh_token, deserialized.refresh_token);
    }

    #[test]
    fn test_logout_request_serialization() {
        let request = AuthRequest {
            username: "test_user".to_string(),
            password: "test_pass".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: AuthRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request.username, deserialized.username);
        assert_eq!(request.password, deserialized.password);
    }

    #[test]
    fn test_refresh_token_info_creation() {
        let username = "test_user".to_string();
        let expires_at = chrono::Utc::now().timestamp() + 30 * 24 * 60 * 60; // 30 days from now

        let token_info = RefreshTokenInfo {
            username: username.clone(),
            expires_at: chrono::DateTime::from_timestamp(expires_at, 0).unwrap(),
        };

        assert_eq!(token_info.username, username);
        assert!(token_info.expires_at > chrono::Utc::now());
    }
}
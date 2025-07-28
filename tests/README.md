# Integration Tests Documentation

This directory contains comprehensive integration tests for the Rust CRUD application with JWT authentication.

## 📁 Structure

```
tests/
├── common/
│   └── mod.rs          # Shared utilities and helpers
├── integration_auth.rs  # Authentication tests
├── integration_crud.rs  # CRUD operations tests
├── integration_errors.rs # Error handling tests
└── README.md           # This documentation
```

## 🧪 Test Categories

### 1. Authentication Tests (`integration_auth.rs`)
- **`test_login_success`**: Tests successful login with valid credentials
- **`test_login_invalid_credentials`**: Tests login rejection with invalid credentials

### 2. CRUD Operations Tests (`integration_crud.rs`)
- **`test_complete_crud_flow`**: Complete CRUD cycle test (Create, Read, Update, Delete)

### 3. Error Handling Tests (`integration_errors.rs`)
- **`test_unauthorized_access`**: Tests access rejection without authentication
- **`test_invalid_token`**: Tests access rejection with invalid JWT token

## 🚀 How to Run Tests

### Run all integration tests:
```bash
cargo test --test integration_auth --test integration_crud --test integration_errors -- --nocapture
```

### Run specific test categories:
```bash
# Authentication tests only
cargo test --test integration_auth -- --nocapture

# CRUD tests only
cargo test --test integration_crud -- --nocapture

# Error handling tests only
cargo test --test integration_errors -- --nocapture
```

### Run all project tests:
```bash
cargo test -- --nocapture
```

## 🔧 How It Works

### Server Lifecycle Management
Each test automatically:
1. **Starts** a real server instance on a random available port
2. **Waits** for the server to be ready (polling `/data` endpoint)
3. **Executes** the test against the running server
4. **Stops** the server and cleans up

### Test Isolation
- Each test runs in its own server instance
- Tests are completely independent
- No shared state between tests
- Automatic port selection to avoid conflicts

### HTTP Client
- Uses `ureq` for HTTP requests (synchronous, no external dependencies)
- Handles both successful responses and error status codes
- Proper JWT token management

## 📋 Test Data

### Available Test Users
- **admin** / **admin123** (main test user)
- **user1** / **password123**
- **user2** / **password456**

### Test Data Structure
```rust
struct TestData {
    func_names: Vec<String>,  // Function names
    bytecode: Vec<u8>,        // WASM bytecode
}
```

## 🛠️ Shared Utilities (`common/mod.rs`)

### Core Functions
- **`find_available_port()`**: Finds a free port for server startup
- **`wait_for_server()`**: Polls server until ready
- **`start_test_server()`**: Starts server and returns URL + process
- **`stop_test_server()`**: Gracefully stops server

### Data Structures
- **`LoginRequest`**: Login credentials
- **`LoginResponse`**: JWT token response
- **`TestData`**: CRUD test data

## 🎯 What Each Test Validates

### Authentication Tests
- ✅ Valid credentials return JWT tokens
- ✅ Invalid credentials return 401
- ✅ Token structure validation
- ✅ Token expiration validation

### CRUD Tests
- ✅ **Create**: POST `/data` with authentication
- ✅ **Read**: GET `/data/{id}` with authentication
- ✅ **Update**: PUT `/data/{id}` with authentication
- ✅ **Delete**: DELETE `/data/{id}` with authentication
- ✅ **Verification**: Confirms data was actually deleted

### Error Tests
- ✅ **401 Unauthorized**: Access without token
- ✅ **401 Unauthorized**: Access with invalid token
- ✅ Proper error status codes
- ✅ Proper error handling

## 🔍 Debugging

### Enable Verbose Output
```bash
cargo test -- --nocapture --test-threads=1
```

### Run Single Test
```bash
cargo test test_login_success -- --nocapture
```

### Check Server Logs
Tests include detailed logging showing:
- Server startup process
- HTTP requests and responses
- Authentication flow
- CRUD operations

## 🚨 Common Issues

### Port Conflicts
- Tests automatically find available ports
- If you see "Address already in use", wait a moment and retry

### Server Startup Time
- Tests wait up to 30 seconds for server startup
- If tests fail with timeout, check if port is blocked

### JWT Token Issues
- Tokens are automatically generated for each test
- Token expiration is set to 1 hour for tests

## 📈 Test Coverage

These integration tests cover:
- ✅ **Authentication Flow**: Login, token validation
- ✅ **CRUD Operations**: Full data lifecycle
- ✅ **Error Handling**: Invalid requests, unauthorized access
- ✅ **Server Management**: Startup, shutdown, health checks
- ✅ **HTTP Status Codes**: 200, 201, 204, 401, 404
- ✅ **JWT Integration**: Token generation, validation, expiration

## 🎓 Learning Objectives

This test suite demonstrates:
- **Real Integration Testing**: Tests against actual running server
- **Async Testing**: Using `async-std` for asynchronous tests
- **HTTP Client Usage**: Making real HTTP requests
- **Process Management**: Starting/stopping external processes
- **Error Handling**: Proper HTTP error status handling
- **Test Organization**: Modular, maintainable test structure

## 🔮 Future Enhancements

- [ ] **Performance Tests**: Load testing with multiple concurrent requests
- [ ] **Database Tests**: Direct database state verification
- [ ] **API Documentation Tests**: Validate OpenAPI/Swagger specs
- [ ] **Security Tests**: Penetration testing scenarios
- [ ] **Migration to Tokio**: Consider migrating from async-std to Tokio runtime 
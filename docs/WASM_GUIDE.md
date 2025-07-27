# WebAssembly Integration Guide

This guide explains how to use WebAssembly integration in the Rust CRUD project.

## 🎯 Overview

The project now supports WebAssembly module execution through the `/execute/:id` endpoint. This allows you to store compiled code as WASM and execute specific functions securely.

## 🏗️ Architecture

### Data Structure

```rust
pub struct DataEntry {
    pub func_names: Vec<String>, // Available function names
    pub bytecode: Vec<u8>,       // Compiled WASM code
    pub owner: String,           // Module owner
}
```

### Execution Flow

1. **Compilation**: Rust code → WebAssembly
2. **Storage**: WASM as bytes in record
3. **Execution**: Loading and execution via wasmi
4. **Security**: Owner and function validation

## 🔨 Building WASM Modules

### Math Library

The project includes a math library in `math/`:

```bash
cd math
./build.sh
```

This script:
- Compiles Rust code to WASM
- Converts WASM to byte array
- Saves to `BYTES_RESULT.txt`

### Available Functions

| Function | Description | Example |
|----------|-------------|---------|
| `add(x, y)` | Addition | `add(10, 20) = 30` |
| `mul(x, y)` | Multiplication | `mul(6, 7) = 42` |
| `sub(x, y)` | Subtraction | `sub(20, 8) = 12` |
| `div(x, y)` | Division | `div(100, 5) = 20` |
| `rem(x, y)` | Remainder | `rem(17, 5) = 2` |
| `abs(x)` | Absolute value | `abs(-15) = 15` |
| `max(x, y)` | Maximum | `max(10, 25) = 25` |
| `min(x, y)` | Minimum | `min(10, 25) = 10` |
| `pow(x, y)` | Power | `pow(2, 3) = 8` |

## 🚀 Using the API

### 1. Create Record with WASM

```bash
# Login first
curl -X POST http://127.0.0.1:8080/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"username": "admin", "password": "admin123"}'

# Create record with WASM
curl -X POST http://127.0.0.1:8080/data \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -d '{
    "func_names": ["add", "mul", "sub", "div"],
    "bytecode": [0,97,115,109,1,0,0,0,...]
  }'
```

### 2. Execute WASM Function

```bash
curl -X POST http://127.0.0.1:8080/execute/1 \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -d '{"fn": "add", "arg": [10, 20]}'
```

**Response:**
```json
{
  "success": true,
  "result": 30,
  "error": null,
  "function": "add",
  "operands": [10, 20],
  "owner": "admin"
}
```

## 🔒 Security

### Implemented Controls

1. **JWT Authentication**: All executions require a valid token
2. **Authorization**: Only the owner can execute their module
3. **Function Validation**: Only allowed functions are executed
4. **Isolation**: Each execution is isolated and secure

### Validations

- ✅ Valid JWT token
- ✅ User is the module owner
- ✅ Function exists in the allowed list
- ✅ Arguments are valid
- ✅ WASM is valid and can be loaded

## 🧪 Tests

### Basic Tests

```bash
./test/10_test_wasm_execute.sh
```

### Advanced Tests

```bash
./test/11_test_advanced_wasm.sh
```

### Complete Demonstration

```bash
./examples/wasm_demo.sh
```

## 📊 Practical Examples

### Example 1: Simple Calculator

```bash
# Create module with basic operations
curl -X POST http://127.0.0.1:8080/data \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "func_names": ["add", "mul", "sub", "div"],
    "bytecode": [WASM_BYTES_HERE]
  }'

# Use as calculator
curl -X POST http://127.0.0.1:8080/execute/1 \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"fn": "mul", "arg": [15, 3]}'
```

### Example 2: Data Processing

```bash
# Create module with processing functions
curl -X POST http://127.0.0.1:8080/data \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "func_names": ["max", "min", "abs"],
    "bytecode": [WASM_BYTES_HERE]
  }'

# Process data
curl -X POST http://127.0.0.1:8080/execute/2 \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"fn": "max", "arg": [100, 250]}'
```

## 🔧 Development

### Creating New WASM Modules

1. **Create a new Rust library**:
```rust
#[no_mangle]
pub extern "C" fn minha_funcao(x: i32, y: i32) -> i32 {
    // Your logic here
    x + y * 2
}
```

2. **Configure Cargo.toml**:
```toml
[lib]
crate-type = ["cdylib"]

[profile.release]
lto = true
codegen-units = 1
opt-level = "z"
```

3. **Compile to WASM**:
```bash
cargo build --target wasm32-unknown-unknown --release
```

4. **Convert to bytes**:
```bash
od -An -v -t uC target/wasm32-unknown-unknown/release/seu_modulo.wasm \
  | tr -s ' ' | tr ' ' ',' | tr -d '\n' | sed 's/^,//;s/,$//g' > bytes.txt
```

### Adding New Functions

1. **Add the function** to the handler:
```rust
let allowed_functions = ["add", "mul", "sub", "div", "sua_funcao"];
```

2. **Update tests** to include the new function
3. **Document** the new functionality

## 🚨 Troubleshooting

### Common Errors

**"Function not found"**
- Check if the function is in the `func_names` list
- Confirm that the WASM was compiled correctly

**"Access denied"**
- Check if you are the module owner
- Confirm that the JWT token is valid

**"Invalid WASM"**
- Recompile the WASM module
- Check if the bytecode is correct

**"Function signature error"**
- Confirm that the function has the signature `(i32, i32) -> i32`
- Check if `#[no_mangle]` is used

### Debug

For debugging, check the server logs:
```bash
RUST_LOG=debug cargo run
```

## 📚 Additional Resources

- [WebAssembly Specification](https://webassembly.github.io/spec/)
- [wasmi Documentation](https://docs.rs/wasmi/)
- [Rust WASM Guide](https://rustwasm.github.io/docs/book/)
- [Road to Meridian](https://github.com/nrxschool/road-to-meridian)

## 🤝 Contributing

To contribute to improving WebAssembly:

1. Add new functions to the `math/` module
2. Update tests
3. Document changes
4. Run all tests
5. Submit a pull request 
# Postman Setup for Learn Rust CRUD API

This guide explains how to configure Postman to test the Rust CRUD API with WebAssembly support.

## Provided Files

1. `postman_collection.json` - Collection with all requests (including WebAssembly)
2. `postman_environment.json` - Local development environment
3. `postman_environment_production.json` - Production environment (Railway)

## How to Import

### 1. Import the Collection
1. Open Postman
2. Click "Import" (button in the upper left corner)
3. Drag the `postman_collection.json` file or click "Upload Files"
4. Select the file and click "Import"

### 2. Import the Environments
1. Click "Import" again
2. Drag the `postman_environment.json` and `postman_environment_production.json` files or click "Upload Files"
3. Select the files and click "Import"

### 3. Select the Environment
1. In the upper right corner of Postman, click the environment dropdown
2. Select:
   - **"Learn Rust CRUD Development Environment"** for local development
   - **"Learn Rust CRUD Production Environment"** for production

## Collection Structure

### Authentication
- **Login**: POST `/auth/login` - Logs in and returns tokens
- **Refresh Token**: POST `/auth/refresh` - Renews access token
- **Logout**: POST `/auth/logout` - Logs out

### CRUD Operations
- **Create Data**: POST `/data` - Creates new record with WebAssembly module
- **Read All Data**: GET `/data` - Lists all records
- **Read Data by ID**: GET `/data/:id` - Searches for specific record
- **Update Data**: PUT `/data/:id` - Updates record
- **Delete Data**: DELETE `/data/:id` - Removes record

### WebAssembly Operations
- **Execute WASM Function**: POST `/execute/:id` - Executes WebAssembly function

> **Note**: The WebAssembly endpoint requires JWT authentication and only the module owner can execute their functions.
> 
> **Important**: All records now contain WebAssembly modules (func_names and bytecode).

## How to Use

### 1. Start the Server (Local Development)
```bash
cargo run
```

**For Production:**
The API is already available at: https://learn-rust-crud-production.up.railway.app

### 1.1. Prepare WebAssembly (Optional)
```bash
cd math
./build.sh
```
This generates the `BYTES_RESULT.txt` file with WASM bytes for testing.

### 2. Login
1. Execute the "Login" request in the "Authentication" folder
2. Use one of the available users:
   - `admin/admin123`
   - `user1/password123`
   - `user2/password456`
3. The access_token and refresh_token will be automatically saved in the variables

### 3. Test CRUD Operations
1. **Create data**: Execute "Create Data" with a JSON like:
   ```json
   {
       "func_names": ["add", "mul", "sub", "div"],
       "bytecode": [0,97,115,109,1,0,0,0,1,6,1,96,2,127,127,1,127,3,2,1,0,7,7,1,3,97,100,100,0,0,10,9,1,7,0,32,0,32,1,106,11]
   }
   ```

2. **List data**: Execute "Read All Data" to see all records

3. **Search by ID**: Copy an ID from the previous response and update the `data_id` variable in the environment

4. **Update data**: Execute "Update Data" with new data

5. **Delete data**: Execute "Delete Data" to remove a record

### 4. Execute WebAssembly Functions
1. **Execute WASM function**: Execute "Execute WASM Function" with:
   ```json
   {
       "fn": "add",
       "arg": [10, 20]
   }
   ```

2. **Test other functions**: Try with `mul`, `sub`, `div`, `rem`, `abs`, `max`, `min`, `pow`

## Environment Variables

### Local Development
- `base_url`: http://127.0.0.1:8080
- `access_token`: JWT access token (automatically filled after login)
- `refresh_token`: Refresh token (automatically filled after login)
- `data_id`: Record ID for specific operations

### Production
- `base_url`: https://learn-rust-crud-production.up.railway.app
- `access_token`: JWT access token (automatically filled after login)
- `refresh_token`: Refresh token (automatically filled after login)
- `data_id`: Record ID for specific operations

## Automatic Scripts

The collection includes scripts that:
- Automatically capture tokens after login
- Save tokens in environment variables
- Allow using tokens in subsequent requests

## Usage Examples

### Complete Test Flow

1. **Login** → Receive tokens
2. **Create Data** → Create a record with WebAssembly module
3. **Read All Data** → List all records
4. **Read Data by ID** → Search for the created record (copy the ID from the previous response)
5. **Update Data** → Update the record (can modify func_names or bytecode)
6. **Delete Data** → Remove the record
7. **Logout** → Invalidate tokens

### WebAssembly Test Flow

1. **Login** → Receive tokens
2. **Create Data** → Create record with WebAssembly module (using func_names and bytecode)
3. **Execute WASM Function** → Test function execution (add, mul, sub, div, etc.)
4. **Test different functions** → Try all available operations
5. **Logout** → Invalidate tokens

### Token Renewal

If the access_token expires:
1. Execute "Refresh Token" using the saved refresh_token
2. The new access_token will be automatically saved
3. Continue testing CRUD operations

## Troubleshooting

### Error 401 Unauthorized
- Check if the access_token is valid
- Execute "Refresh Token" if necessary
- Check if the token is being sent in the Authorization header

### Error 404 Not Found
- Check if the server is running at http://127.0.0.1:8080
- Check if the URL is correct

### Error 400 Bad Request
- Check the JSON format being sent
- Check if all required fields are present
- For WASM: Check if the function is in the `func_names` list and if the bytecode is valid

### Error 403 Forbidden (WASM)
- Check if you are the owner of the WASM module
- Only the record creator can execute their WASM functions

## Server Configuration

Make sure the `.env` file is configured correctly:

```env
SERVER_ADDR=127.0.0.1:8080
JWT_SECRET=your-secret-key-change-in-production
JWT_ISSUER=learn-rust-crud
ACCESS_TOKEN_EXPIRATION_HOURS=1
REFRESH_TOKEN_EXPIRATION_DAYS=30
```

## Available Test Users

- **admin/admin123** - Administrator user
- **user1/password123** - Regular user 1
- **user2/password456** - Regular user 2

Each user has access only to their own data.

## Data Structure

### DataEntry (Record)
```json
{
  "func_names": ["add", "mul", "sub", "div"],
  "bytecode": [0,97,115,109,1,0,0,0,...],
  "owner": "admin"
}
```

- **func_names**: List of function names available in the WASM module
- **bytecode**: Array of bytes of the compiled WebAssembly code
- **owner**: Name of the record owner

### Creation Response Example
```json
{
  "id": 1,
  "func_names": ["add", "mul", "sub", "div"],
  "bytecode": [0,97,115,109,1,0,0,0,...],
  "owner": "admin"
}
```

## Available WebAssembly Functions

The project includes a math library with the following functions:

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

## WASM Response Examples

### Successful Execution
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

### Function Not Found Error
```json
{
  "success": false,
  "result": null,
  "error": "Function 'invalid_func' not found in WASM module",
  "function": "invalid_func",
  "operands": [10, 20],
  "owner": "admin"
}
```

## Tips for WASM Testing

1. **Use the build script**: Run `./math/build.sh` to generate valid WASM bytes
2. **Test all functions**: Try different mathematical operations
3. **Check ownership**: Only the creator can execute their functions
4. **Validate input**: Functions expect `[i32, i32]` arguments
5. **Test special cases**: Division by zero, negative values, etc. 
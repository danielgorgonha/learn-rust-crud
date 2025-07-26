# Learn Rust CRUD

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tide](https://img.shields.io/badge/Tide-0.16.0-blue.svg)](https://github.com/http-rs/tide)

A simple REST CRUD API built with Rust and the Tide framework, designed for learning fundamental web development concepts in Rust. **Now with JWT authentication and refresh tokens!**

## 🎯 About the Project

This project demonstrates how to implement CRUD operations (Create, Read, Update, Delete) using Rust and the Tide web framework. It's ideal for developers who want to learn:

- Asynchronous programming in Rust
- REST API development
- Thread-safe state management
- JSON serialization/deserialization
- Modular Rust project structure
- **JWT authentication and authorization systems**
- **Refresh token implementation**
- **Token-based security with expiration**

## 🚀 Features

- ✅ **CREATE**: Create new records (owner-only)
- ✅ **READ**: List all records or search by ID (authenticated users)
- ✅ **UPDATE**: Update existing records (owner-only)
- ✅ **DELETE**: Remove records (owner-only)
- ✅ **Thread-safe**: Safe shared state between multiple requests
- ✅ **JSON API**: REST interface with JSON
- ✅ **JWT Authentication**: Secure JWT tokens with expiration
- ✅ **Refresh Tokens**: Automatic token refresh system
- ✅ **Authorization**: Owner-only access for sensitive operations
- ✅ **Tests**: Automated test scripts with authentication

## 🛠️ Technologies Used

- **[Rust](https://www.rust-lang.org/)** - Programming language
- **[Tide](https://github.com/http-rs/tide)** - Asynchronous web framework
- **[Serde](https://serde.rs/)** - Serialization/deserialization
- **[async-std](https://async.rs/)** - Asynchronous runtime
- **[jsonwebtoken](https://docs.rs/jsonwebtoken/)** - JWT token generation
- **[chrono](https://docs.rs/chrono/)** - Date and time handling

## 📦 Installation

### Prerequisites

- Rust 1.70+ installed
- Git

### Clone and Run

```bash
# Clone the repository
git clone https://github.com/danielgorgonha/learn-rust-crud.git
cd learn-rust-crud

# Configure environment (optional)
cp env.example .env
# Edit .env to customize settings

# Run the project
cargo run
```

The server will be available at: `http://127.0.0.1:8080` (configurable via `SERVER_ADDR` environment variable)

### Environment Variables

The application supports the following environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `SERVER_ADDR` | `127.0.0.1:8080` | Server address and port |
| `JWT_SECRET` | `your-secret-key-change-in-production` | Secret key for JWT signing |
| `JWT_ISSUER` | `learn-rust-crud` | JWT issuer claim |
| `ACCESS_TOKEN_EXPIRATION_HOURS` | `1` | Access token expiration in hours |
| `REFRESH_TOKEN_EXPIRATION_DAYS` | `30` | Refresh token expiration in days |

**Example `.env` file:**
```bash
SERVER_ADDR=127.0.0.1:3000
JWT_SECRET=my-super-secret-key-for-production
JWT_ISSUER=my-app
ACCESS_TOKEN_EXPIRATION_HOURS=2
REFRESH_TOKEN_EXPIRATION_DAYS=7
```

## 🔐 Authentication

### Default Users

The system comes with pre-configured users for testing:

| Username | Password |
|----------|----------|
| `admin`  | `admin123` |
| `user1`  | `password123` |
| `user2`  | `password456` |

### Authentication Flow

1. **Login** to get access and refresh tokens
2. **Use access token** in Authorization header for all requests
3. **Refresh access token** when it expires (using refresh token)
4. **Logout** to invalidate refresh token

### Token Expiration

- **Access Token**: 1 hour (for security)
- **Refresh Token**: 30 days (for convenience)

## 📚 How to Use

### Authentication Endpoints

#### Login
```bash
curl -X POST http://127.0.0.1:8080/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"username": "admin", "password": "admin123"}'
```

**Response:**
```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "username": "admin",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

#### Refresh Token
```bash
curl -X POST http://127.0.0.1:8080/auth/refresh \
  -H 'Content-Type: application/json' \
  -d '{"refresh_token": "your-refresh-token-here"}'
```

#### Logout
```bash
curl -X POST http://127.0.0.1:8080/auth/logout \
  -H 'Content-Type: application/json' \
  -d '{"refresh_token": "your-refresh-token-here"}'
```

### Data Model

```json
{
  "data1": ["text1", "text2"],
  "data2": [1, 2, 3]
}
```

### API Endpoints

| Method | Endpoint | Description | Auth Required | Owner Only |
|--------|----------|-------------|---------------|------------|
| `POST` | `/auth/login` | Login and get tokens | ❌ | ❌ |
| `POST` | `/auth/refresh` | Refresh access token | ❌ | ❌ |
| `POST` | `/auth/logout` | Logout and invalidate refresh token | ❌ | ❌ |
| `POST` | `/data` | Create new record | ✅ | ✅ |
| `GET` | `/data` | List all records | ✅ | ❌ |
| `GET` | `/data/:id` | Get record by ID | ✅ | ❌ |
| `PUT` | `/data/:id` | Update record | ✅ | ✅ |
| `DELETE` | `/data/:id` | Delete record | ✅ | ✅ |

### Usage Examples

#### 1. Login and get tokens
```bash
# Login
resp=$(curl -s -X POST http://127.0.0.1:8080/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"username": "admin", "password": "admin123"}')

# Extract tokens
access_token=$(echo $resp | grep -oE '"access_token":"[^"]*"' | cut -d'"' -f4)
refresh_token=$(echo $resp | grep -oE '"refresh_token":"[^"]*"' | cut -d'"' -f4)
```

#### 2. Create a record (requires authentication)
```bash
curl -X POST http://127.0.0.1:8080/data \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $access_token" \
  -d '{"data1": ["first", "second"], "data2": [1,2,3]}'
```

#### 3. Refresh access token when expired
```bash
curl -X POST http://127.0.0.1:8080/auth/refresh \
  -H 'Content-Type: application/json' \
  -d "{\"refresh_token\": \"$refresh_token\"}"
```

#### 4. List all records (requires authentication)
```bash
curl -X GET http://127.0.0.1:8080/data \
  -H "Authorization: Bearer $access_token"
```

#### 5. Update record (requires authentication + ownership)
```bash
curl -X PUT http://127.0.0.1:8080/data/1 \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $access_token" \
  -d '{"data1": ["updated"], "data2": [10,20,30]}'
```

#### 6. Delete record (requires authentication + ownership)
```bash
curl -X DELETE http://127.0.0.1:8080/data/1 \
  -H "Authorization: Bearer $access_token"
```

## 🧪 Testing

The project includes automated test scripts in the `test/` folder:

```bash
# Make scripts executable
chmod +x test/*.sh

# Test refresh token system
./test/test_refresh_token.sh

# Run complete test suite
./test/run_all_tests.sh

# Or run individual tests
./test/0_login.sh
./test/1_create.sh
./test/2_read_all.sh
./test/3_read_one.sh
./test/4_update.sh
./test/5_delete.sh
```

## 🏗️ Project Structure

```
src/
├── main.rs          # Entry point and server configuration
├── models.rs        # Data model definitions
├── state.rs         # Global state management
├── auth.rs          # Authentication and authorization logic
└── handlers/        # CRUD operation handlers
    ├── create.rs    # CREATE operation
    ├── read.rs      # READ operations
    ├── update.rs    # UPDATE operation
    └── delete.rs    # DELETE operation

test/                 # Test scripts
├── 0_login.sh       # Authentication test
├── 1_create.sh      # Create test
├── 2_read_all.sh    # Read all test
├── 3_read_one.sh    # Read one test
├── 4_update.sh      # Update test
├── 5_delete.sh      # Delete test
├── test_refresh_token.sh # Refresh token tests
└── run_all_tests.sh # Complete test suite

postman/              # Postman collection and environments
├── postman_collection.json             # Main collection with all requests
├── postman_environment.json            # Development environment (localhost)
├── postman_environment_production.json # Production environment (Railway)
└── POSTMAN_SETUP.md                    # Complete setup guide
```

## 🔧 Development

### Run in development mode

```bash
cargo run
```

## 🧪 Testing

### Using Postman

This project includes a complete Postman collection for testing the API. The collection is located in the `postman/` folder and includes:

#### 📁 Postman Files
- `postman_collection.json` - Complete collection with all API endpoints
- `postman_environment.json` - Development environment (localhost:8080)
- `postman_environment_production.json` - Production environment (Railway)
- `POSTMAN_SETUP.md` - Detailed setup and usage guide

#### 🌍 Available Environments
- **Development**: `http://127.0.0.1:8080` - For local testing
- **Production**: `https://learn-rust-crud-production.up.railway.app` - For production testing

#### 🚀 Quick Setup
1. Import the collection and environment files into Postman
2. Select the desired environment (development or production)
3. Start with the "Login" request to get authentication tokens
4. Test all CRUD operations with automatic token management

#### 📋 Test Flow
1. **Login** → Get access and refresh tokens
2. **Create Data** → Create a new record
3. **Read All Data** → List all records
4. **Read Data by ID** → Get specific record
5. **Update Data** → Modify existing record
6. **Delete Data** → Remove record
7. **Logout** → Invalidate tokens

For detailed instructions, see `postman/POSTMAN_SETUP.md`.

### Using Shell Scripts

The project includes shell scripts in the `test/` folder for automated testing:

```bash
# Run all tests
./test/run_all_tests.sh

# Run individual tests
./test/0_login.sh      # Test authentication
./test/1_create.sh     # Test create operation
./test/2_read_all.sh   # Test read all operation
./test/3_read_one.sh   # Test read one operation
./test/4_update.sh     # Test update operation
./test/5_delete.sh     # Test delete operation
```

### Run tests

```bash
cargo test
```

### Check code

```bash
cargo check
cargo clippy
```

## 🔒 Security Features

- **JWT authentication**: Secure JWT tokens with expiration
- **Refresh tokens**: Long-lived tokens for automatic renewal
- **Access tokens**: Short-lived tokens for security
- **Owner-only operations**: Users can only modify their own data
- **Token invalidation**: Refresh tokens removed on logout
- **Authorization headers**: Bearer token authentication
- **Error handling**: Proper HTTP status codes for auth failures

## 🔄 Refresh Token Flow

1. **Login** → Get access token (1h) + refresh token (30d)
2. **Use access token** → Make API requests
3. **Access token expires** → Use refresh token to get new access token
4. **Refresh token expires** → Login again
5. **Logout** → Invalidate refresh token

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**Daniel R Gorgonha** - [danielgorgonha@gmail.com](mailto:danielgorgonha@gmail.com)

- GitHub: [@danielgorgonha](https://github.com/danielgorgonha)

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the project
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📚 Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tide Documentation](https://docs.rs/tide)
- [Serde Documentation](https://serde.rs/)
- [async-std Documentation](https://docs.rs/async-std)
- [jsonwebtoken Documentation](https://docs.rs/jsonwebtoken/)
- [Chrono Documentation](https://docs.rs/chrono/)

## ⭐ If this project helped you

If this project was useful for your learning, consider giving it a ⭐ on the repository!

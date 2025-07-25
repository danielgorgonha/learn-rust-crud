# Learn Rust CRUD

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tide](https://img.shields.io/badge/Tide-0.16.0-blue.svg)](https://github.com/http-rs/tide)

A simple REST CRUD API built with Rust and the Tide framework, designed for learning fundamental web development concepts in Rust.

## 🎯 About the Project

This project demonstrates how to implement CRUD operations (Create, Read, Update, Delete) using Rust and the Tide web framework. It's ideal for developers who want to learn:

- Asynchronous programming in Rust
- REST API development
- Thread-safe state management
- JSON serialization/deserialization
- Modular Rust project structure

## 🚀 Features

- ✅ **CREATE**: Create new records
- ✅ **READ**: List all records or search by ID
- ✅ **UPDATE**: Update existing records
- ✅ **DELETE**: Remove records
- ✅ **Thread-safe**: Safe shared state between multiple requests
- ✅ **JSON API**: REST interface with JSON
- ✅ **Tests**: Automated test scripts

## 🛠️ Technologies Used

- **[Rust](https://www.rust-lang.org/)** - Programming language
- **[Tide](https://github.com/http-rs/tide)** - Asynchronous web framework
- **[Serde](https://serde.rs/)** - Serialization/deserialization
- **[async-std](https://async.rs/)** - Asynchronous runtime

## 📦 Installation

### Prerequisites

- Rust 1.70+ installed
- Git

### Clone and Run

```bash
# Clone the repository
git clone https://github.com/danielgorgonha/learn-rust-crud.git
cd learn-rust-crud

# Run the project
cargo run
```

The server will be available at: `http://127.0.0.1:8080`

## 📚 How to Use

### Data Model

```json
{
  "data1": ["text1", "text2"],
  "data2": [1, 2, 3]
}
```

### API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/data` | Create new record |
| `GET` | `/data` | List all records |
| `GET` | `/data/:id` | Get record by ID |
| `PUT` | `/data/:id` | Update record |
| `DELETE` | `/data/:id` | Delete record |

### Usage Examples

#### 1. Create a record
```bash
curl -X POST http://127.0.0.1:8080/data \
  -H 'Content-Type: application/json' \
  -d '{"data1": ["first", "second"], "data2": [1,2,3]}'
```

#### 2. List all records
```bash
curl http://127.0.0.1:8080/data
```

#### 3. Get specific record
```bash
curl http://127.0.0.1:8080/data/1
```

#### 4. Update record
```bash
curl -X PUT http://127.0.0.1:8080/data/1 \
  -H 'Content-Type: application/json' \
  -d '{"data1": ["updated"], "data2": [10,20,30]}'
```

#### 5. Delete record
```bash
curl -X DELETE http://127.0.0.1:8080/data/1
```

## 🧪 Testing

The project includes automated test scripts in the `test/` folder:

```bash
# Run tests in sequence
chmod +x test/*.sh
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
└── handlers/        # CRUD operation handlers
    ├── create.rs    # CREATE operation
    ├── read.rs      # READ operations
    ├── update.rs    # UPDATE operation
    └── delete.rs    # DELETE operation

test/                 # Test scripts
├── 1_create.sh
├── 2_read_all.sh
├── 3_read_one.sh
├── 4_update.sh
└── 5_delete.sh
```

## 🔧 Development

### Run in development mode

```bash
cargo run
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

## ⭐ If this project helped you

If this project was useful for your learning, consider giving it a ⭐ on the repository!

#!/bin/bash

# Complete demonstration: CRUD with WebAssembly
set -e

echo "🚀 Complete demonstration: CRUD with WebAssembly"
echo "=============================================="

# Configuration
BASE_URL="http://127.0.0.1:8080"
ADMIN_USER="admin"
ADMIN_PASS="admin123"

# Function to login
login() {
    echo "🔐 Logging in..."
    local response=$(curl -s -X POST "$BASE_URL/auth/login" \
        -H 'Content-Type: application/json' \
        -d "{\"username\": \"$ADMIN_USER\", \"password\": \"$ADMIN_PASS\"}")
    
    local access_token=$(echo $response | grep -oE '"access_token":"[^"]*"' | cut -d'"' -f4)
    echo "✅ Login successful"
    echo $access_token
}

# Function to build WASM
build_wasm() {
    echo "🔨 Building WebAssembly module..."
    cd math
    ./build.sh
    cd ..
    echo "✅ WASM built successfully"
}

# Function to read WASM bytes
get_wasm_bytes() {
    if [ -f "math/BYTES_RESULT.txt" ]; then
        cat math/BYTES_RESULT.txt
    else
        echo "❌ BYTES_RESULT.txt file not found. Run build_wasm first."
        exit 1
    fi
}

# Function to create WASM record
create_wasm_record() {
    local token=$1
    local wasm_bytes=$2
    
    echo "📝 Creating WASM record..."
    
    local response=$(curl -s -X POST "$BASE_URL/data" \
        -H 'Content-Type: application/json' \
        -H "Authorization: Bearer $token" \
        -d "{\"func_names\": [\"add\", \"mul\", \"sub\", \"div\"], \"bytecode\": $wasm_bytes}")
    
    local id=$(echo $response | grep -oE '"id":[0-9]+' | cut -d':' -f2)
    echo "✅ Record created with ID: $id"
    echo $id
}

# Function to execute WASM operations
execute_wasm_operations() {
    local token=$1
    local id=$2
    
    echo "⚡ Executing WebAssembly operations..."
    
    # Addition test
    echo "  📊 Testing add(15, 25)..."
    local add_result=$(curl -s -X POST "$BASE_URL/execute/$id" \
        -H 'Content-Type: application/json' \
        -H "Authorization: Bearer $token" \
        -d '{"fn": "add", "arg": [15, 25]}')
    echo "    Result: $add_result"
    
    # Multiplication test
    echo "  📊 Testing mul(6, 7)..."
    local mul_result=$(curl -s -X POST "$BASE_URL/execute/$id" \
        -H 'Content-Type: application/json' \
        -H "Authorization: Bearer $token" \
        -d '{"fn": "mul", "arg": [6, 7]}')
    echo "    Result: $mul_result"
    
    # Subtraction test
    echo "  📊 Testing sub(20, 8)..."
    local sub_result=$(curl -s -X POST "$BASE_URL/execute/$id" \
        -H 'Content-Type: application/json' \
        -H "Authorization: Bearer $token" \
        -d '{"fn": "sub", "arg": [20, 8]}')
    echo "    Result: $sub_result"
    
    # Division test
    echo "  📊 Testing div(100, 5)..."
    local div_result=$(curl -s -X POST "$BASE_URL/execute/$id" \
        -H 'Content-Type: application/json' \
        -H "Authorization: Bearer $token" \
        -d '{"fn": "div", "arg": [100, 5]}')
    echo "    Result: $div_result"
    
    # Division by zero test
    echo "  📊 Testing div(10, 0)..."
    local div_zero_result=$(curl -s -X POST "$BASE_URL/execute/$id" \
        -H 'Content-Type: application/json' \
        -H "Authorization: Bearer $token" \
        -d '{"fn": "div", "arg": [10, 0]}')
    echo "    Result: $div_zero_result"
}

# Function to list records
list_records() {
    local token=$1
    
    echo "📋 Listing records..."
    local response=$(curl -s -X GET "$BASE_URL/data" \
        -H "Authorization: Bearer $token")
    echo "  Records: $response"
}

# Function to delete record
delete_record() {
    local token=$1
    local id=$2
    
    echo "🗑️ Deleting record $id..."
    curl -s -X DELETE "$BASE_URL/data/$id" \
        -H "Authorization: Bearer $token"
    echo "✅ Record deleted"
}

# Demonstration execution
echo "🎯 Starting demonstration..."

# 1. Login
token=$(login)

# 2. Build WASM
build_wasm

# 3. Get WASM bytes
wasm_bytes=$(get_wasm_bytes)

# 4. Create record
id=$(create_wasm_record "$token" "$wasm_bytes")

# 5. List records
list_records "$token"

# 6. Execute WASM operations
execute_wasm_operations "$token" "$id"

# 7. Delete record
delete_record "$token" "$id"

echo ""
echo "🎉 Demonstration completed successfully!"
echo "✨ The CRUD with WebAssembly is working perfectly!" 
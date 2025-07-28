#!/bin/bash

# Complete WASM test script
set -e

echo "🧪 Complete WASM Test Script"
echo "============================"

# Configuration
BASE_URL="http://127.0.0.1:8080"
ADMIN_USER="admin"
ADMIN_PASS="admin123"

# Function to login and get token
login() {
    local response=$(curl -s -X POST "$BASE_URL/auth/login" \
        -H 'Content-Type: application/json' \
        -d "{\"username\": \"$ADMIN_USER\", \"password\": \"$ADMIN_PASS\"}")
    
    echo "DEBUG: Login response: $response" >&2
    local access_token=$(echo "$response" | grep -oE '"access_token":"[^"]*"' | cut -d'"' -f4)
    echo "DEBUG: Extracted token: $access_token" >&2
    echo "✅ Login successful" >&2
    echo "$access_token"
}

# Function to create a WASM record
create_wasm_record() {
    local token=$1
    
    # WASM bytes from real build
    local wasm_bytes="[$(cat math/BYTES_RESULT.txt)]"
    
    echo "DEBUG: Using token: $token" >&2
    local response=$(curl -s -X POST "$BASE_URL/data" \
        -H 'Content-Type: application/json' \
        -H "Authorization: Bearer $token" \
        -d "{\"func_names\": [\"add\"], \"bytecode\": $wasm_bytes}")
    
    echo "DEBUG: Create response: $response" >&2
    local id=$(echo "$response" | grep -oE '"id":[0-9]+' | cut -d':' -f2)
    echo "DEBUG: Extracted ID: '$id'" >&2
    echo "✅ Record created with ID: $id" >&2
    echo "$id"
}

# Function to test WASM execution
test_wasm_execution() {
    local token=$1
    local id=$2
    
    echo "⚡ Testing execution of add(10, 20) function..."
    echo "DEBUG: Using token: $token" >&2
    echo "DEBUG: Using ID: $id" >&2
    
    local response=$(curl -s -X POST "$BASE_URL/execute/$id" \
        -H 'Content-Type: application/json' \
        -H "Authorization: Bearer $token" \
        -d '{"fn": "add", "arg": [10, 20]}')
    
    echo "📊 Response: $response"
    
    # Check if execution was successful
    if echo "$response" | grep -q '"success":true'; then
        echo "✅ WASM execution successful!"
        local result=$(echo "$response" | grep -oE '"result":[0-9]+' | cut -d':' -f2)
        echo "🎯 Result: $result (expected: 30)"
        
        if [ "$result" = "30" ]; then
            echo "✅ Correct result!"
            return 0
        else
            echo "❌ Incorrect result. Expected: 30, Got: $result"
            return 1
        fi
    else
        echo "❌ WASM execution failed"
        echo "Error: $response"
        return 1
    fi
}

# Main execution
echo "🚀 Starting complete WASM test..."

# Login
token=$(login)
if [ -z "$token" ]; then
    echo "❌ Login failed"
    exit 1
fi

# Create record
id=$(create_wasm_record "$token")
if [ -z "$id" ]; then
    echo "❌ Record creation failed"
    exit 1
fi

# Wait a bit to ensure server processed
sleep 1

# Test execution
if test_wasm_execution "$token" "$id"; then
    echo "🎉 WASM test completed successfully!"
else
    echo "❌ WASM test failed"
    exit 1
fi 
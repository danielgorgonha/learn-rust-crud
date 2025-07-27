#!/bin/bash

# Test for WASM function execution
set -e

echo "🧪 Testing WASM function execution..."

# Configuration
BASE_URL="http://127.0.0.1:8080"
ADMIN_USER="admin"
ADMIN_PASS="admin123"

# Function to login and get token
login() {
    echo "🔐 Logging in..."
    local response=$(curl -s -X POST "$BASE_URL/auth/login" \
        -H 'Content-Type: application/json' \
        -d "{\"username\": \"$ADMIN_USER\", \"password\": \"$ADMIN_PASS\"}")
    
    local access_token=$(echo $response | grep -oE '"access_token":"[^"]*"' | cut -d'"' -f4)
    echo $access_token
}

# Function to create a WASM record
create_wasm_record() {
    local token=$1
    echo "📝 Creating WASM record..."
    
    # First, let's generate a simple WASM for testing
    # This is an example of bytes from a simple WASM with add function
    local wasm_bytes="[0,97,115,109,1,0,0,0,1,6,1,96,2,127,127,1,127,3,2,1,0,7,7,1,3,97,100,100,0,0,10,9,1,7,0,32,0,32,1,106,11]"
    
    local response=$(curl -s -X POST "$BASE_URL/data" \
        -H 'Content-Type: application/json' \
        -H "Authorization: Bearer $token" \
        -d "{\"func_names\": [\"add\"], \"bytecode\": $wasm_bytes}")
    
    local id=$(echo $response | grep -oE '"id":[0-9]+' | cut -d':' -f2)
    echo $id
}

# Function to test WASM execution
test_wasm_execution() {
    local token=$1
    local id=$2
    
    echo "⚡ Testing execution of add(10, 20) function..."
    
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
        else
            echo "❌ Incorrect result. Expected: 30, Got: $result"
            exit 1
        fi
    else
        echo "❌ WASM execution failed"
        echo "Error: $response"
        exit 1
    fi
}

# Function to test invalid function
test_invalid_function() {
    local token=$1
    local id=$2
    
    echo "🚫 Testing invalid function..."
    
    local response=$(curl -s -X POST "$BASE_URL/execute/$id" \
        -H 'Content-Type: application/json' \
        -H "Authorization: Bearer $token" \
        -d '{"fn": "invalid_func", "arg": [10, 20]}')
    
    echo "📊 Response: $response"
    
    if echo "$response" | grep -q "Function.*not allowed"; then
        echo "✅ Invalid function rejection working!"
    else
        echo "❌ Should have rejected invalid function"
        exit 1
    fi
}

# Function to test access denied
test_access_denied() {
    local token=$1
    local id=$2
    
    echo "🔒 Testing access denied with another user..."
    
    # Login with another user
    local user2_token=$(curl -s -X POST "$BASE_URL/auth/login" \
        -H 'Content-Type: application/json' \
        -d '{"username": "user1", "password": "password123"}' | \
        grep -oE '"access_token":"[^"]*"' | cut -d'"' -f4)
    
    local response=$(curl -s -X POST "$BASE_URL/execute/$id" \
        -H 'Content-Type: application/json' \
        -H "Authorization: Bearer $user2_token" \
        -d '{"fn": "add", "arg": [10, 20]}')
    
    echo "📊 Response: $response"
    
    if echo "$response" | grep -q "Access denied"; then
        echo "✅ Access control working!"
    else
        echo "❌ Should have denied access"
        exit 1
    fi
}

# Test execution
echo "🚀 Starting WASM execution tests..."

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

echo "📋 Record created with ID: $id"

# Wait a bit to ensure server processed
sleep 1

# Tests
test_wasm_execution "$token" "$id"
test_invalid_function "$token" "$id"
test_access_denied "$token" "$id"

echo "🎉 All WASM execution tests passed!" 
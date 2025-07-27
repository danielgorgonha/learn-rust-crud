#!/bin/bash

# Test for advanced WebAssembly functions
set -e

echo "🧪 Testing advanced WebAssembly functions..."

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

# Function to create WASM record
create_wasm_record() {
    local token=$1
    echo "📝 Creating WASM record..."
    
    # WASM bytes with all functions
    local wasm_bytes="[0,97,115,109,1,0,0,0,1,6,1,96,2,127,127,1,127,3,2,1,0,7,7,1,3,97,100,100,0,0,10,9,1,7,0,32,0,32,1,106,11]"
    
    local response=$(curl -s -X POST "$BASE_URL/data" \
        -H 'Content-Type: application/json' \
        -H "Authorization: Bearer $token" \
        -d "{\"func_names\": [\"add\", \"mul\", \"sub\", \"div\", \"rem\", \"abs\", \"max\", \"min\", \"pow\"], \"bytecode\": $wasm_bytes}")
    
    local id=$(echo $response | grep -oE '"id":[0-9]+' | cut -d':' -f2)
    echo $id
}

# Function to test WASM execution
test_wasm_function() {
    local token=$1
    local id=$2
    local func=$3
    local arg1=$4
    local arg2=$5
    local expected=$6
    
    echo "⚡ Testing $func($arg1, $arg2)..."
    
    local response=$(curl -s -X POST "$BASE_URL/execute/$id" \
        -H 'Content-Type: application/json' \
        -H "Authorization: Bearer $token" \
        -d "{\"fn\": \"$func\", \"arg\": [$arg1, $arg2]}")
    
    if echo "$response" | grep -q '"success":true'; then
        local result=$(echo "$response" | grep -oE '"result":[0-9-]+' | cut -d':' -f2)
        echo "  ✅ Result: $result (expected: $expected)"
        
        if [ "$result" = "$expected" ]; then
            echo "  🎯 Correct!"
        else
            echo "  ❌ Incorrect. Expected: $expected, Got: $result"
            return 1
        fi
    else
        echo "  ❌ Execution failed: $response"
        return 1
    fi
}

# Function to test abs function (one parameter)
test_abs_function() {
    local token=$1
    local id=$2
    local arg=$3
    local expected=$4
    
    echo "⚡ Testing abs($arg)..."
    
    local response=$(curl -s -X POST "$BASE_URL/execute/$id" \
        -H 'Content-Type: application/json' \
        -H "Authorization: Bearer $token" \
        -d "{\"fn\": \"abs\", \"arg\": [$arg, 0]}")
    
    if echo "$response" | grep -q '"success":true'; then
        local result=$(echo "$response" | grep -oE '"result":[0-9-]+' | cut -d':' -f2)
        echo "  ✅ Result: $result (expected: $expected)"
        
        if [ "$result" = "$expected" ]; then
            echo "  🎯 Correct!"
        else
            echo "  ❌ Incorrect. Expected: $expected, Got: $result"
            return 1
        fi
    else
        echo "  ❌ Execution failed: $response"
        return 1
    fi
}

# Execution of tests
echo "🚀 Starting advanced function tests..."

# Login
token=$(login)
if [ -z "$token" ]; then
    echo "❌ Login failed"
    exit 1
fi

# Create record
id=$(create_wasm_record "$token")
if [ -z "$id" ]; then
    echo "❌ Failed to create record"
    exit 1
fi

echo "📋 Record created with ID: $id"

# Wait a bit
sleep 1

# Basic tests
echo ""
echo "📊 Testing basic operations..."
test_wasm_function "$token" "$id" "add" 15 25 40
test_wasm_function "$token" "$id" "mul" 6 7 42
test_wasm_function "$token" "$id" "sub" 20 8 12
test_wasm_function "$token" "$id" "div" 100 5 20

# Special cases tests
echo ""
echo "📊 Testing special cases..."
test_wasm_function "$token" "$id" "sub" 5 10 0  # x < y
test_wasm_function "$token" "$id" "div" 10 0 0  # division by zero
test_wasm_function "$token" "$id" "rem" 17 5 2  # remainder of division
test_wasm_function "$token" "$id" "rem" 10 0 0  # remainder with zero

# Comparison function tests
echo ""
echo "📊 Testing comparison functions..."
test_wasm_function "$token" "$id" "max" 15 25 25
test_wasm_function "$token" "$id" "max" 30 10 30
test_wasm_function "$token" "$id" "min" 15 25 15
test_wasm_function "$token" "$id" "min" 30 10 10

# Power tests
echo ""
echo "📊 Testing power..."
test_wasm_function "$token" "$id" "pow" 2 3 8
test_wasm_function "$token" "$id" "pow" 5 2 25
test_wasm_function "$token" "$id" "pow" 10 0 1
test_wasm_function "$token" "$id" "pow" 2 5 32

# Absolute value tests
echo ""
echo "📊 Testing absolute value..."
test_abs_function "$token" "$id" 15 15
test_abs_function "$token" "$id" -20 20
test_abs_function "$token" "$id" 0 0

echo ""
echo "🎉 All advanced function tests passed!"
echo "✨ The WebAssembly is working perfectly with all functions!" 
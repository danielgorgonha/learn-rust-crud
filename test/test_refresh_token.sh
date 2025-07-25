#!/bin/bash
# Refresh Token System Tests

echo "=== REFRESH TOKEN SYSTEM TESTS ==="
echo

# Test 1: Login and get both tokens
echo "1. Testing login with refresh token..."
login_response=$(curl -s -X POST http://127.0.0.1:8080/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"username": "admin", "password": "admin123"}')

echo "Login response: $login_response"

# Extract tokens
access_token=$(echo $login_response | grep -oE '"access_token":"[^"]*"' | cut -d'"' -f4)
refresh_token=$(echo $login_response | grep -oE '"refresh_token":"[^"]*"' | cut -d'"' -f4)

echo "Access Token: ${access_token:0:50}..."
echo "Refresh Token: ${refresh_token:0:50}..."
echo

# Test 2: Use access token
echo "2. Testing access token..."
access_response=$(curl -s -X GET http://127.0.0.1:8080/data \
  -H "Authorization: Bearer $access_token")
echo "Access token response: $access_response"
echo

# Test 3: Refresh access token
echo "3. Testing refresh token..."
refresh_response=$(curl -s -X POST http://127.0.0.1:8080/auth/refresh \
  -H 'Content-Type: application/json' \
  -d "{\"refresh_token\": \"$refresh_token\"}")

echo "Refresh response: $refresh_response"

# Extract new access token
new_access_token=$(echo $refresh_response | grep -oE '"access_token":"[^"]*"' | cut -d'"' -f4)
echo "New Access Token: ${new_access_token:0:50}..."
echo

# Test 4: Use new access token
echo "4. Testing new access token..."
new_access_response=$(curl -s -X GET http://127.0.0.1:8080/data \
  -H "Authorization: Bearer $new_access_token")
echo "New access token response: $new_access_response"
echo

# Test 5: Test with invalid refresh token
echo "5. Testing with invalid refresh token..."
invalid_refresh_response=$(curl -s -X POST http://127.0.0.1:8080/auth/refresh \
  -H 'Content-Type: application/json' \
  -d '{"refresh_token": "invalid.refresh.token"}')
echo "Invalid refresh response: $invalid_refresh_response"
echo

# Test 6: Test logout with refresh token
echo "6. Testing logout with refresh token..."
logout_response=$(curl -s -X POST http://127.0.0.1:8080/auth/logout \
  -H 'Content-Type: application/json' \
  -d "{\"refresh_token\": \"$refresh_token\"}")
echo "Logout response: $logout_response"
echo

# Test 7: Test that refresh token no longer works after logout
echo "7. Testing refresh token after logout..."
post_logout_refresh_response=$(curl -s -X POST http://127.0.0.1:8080/auth/refresh \
  -H 'Content-Type: application/json' \
  -d "{\"refresh_token\": \"$refresh_token\"}")
echo "Post-logout refresh response: $post_logout_refresh_response"
echo

echo "=== REFRESH TOKEN TESTS COMPLETED ===" 
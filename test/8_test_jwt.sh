#!/bin/bash
# JWT Authentication Tests

echo "=== JWT AUTHENTICATION TESTS ==="
echo

# Test 1: Login and get JWT token
echo "1. Testing JWT login..."
login_response=$(curl -s -X POST http://127.0.0.1:8080/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"username": "admin", "password": "admin123"}')

echo "Login response: $login_response"

# Extract JWT token (access_token)
jwt_token=$(echo $login_response | grep -oE '"access_token":"[^"]*"' | cut -d'"' -f4)
echo "JWT Token: $jwt_token"
echo

# Test 2: Verify JWT token structure
echo "2. Verifying JWT token structure..."
if [[ $jwt_token == *"."*"."* ]]; then
    echo "✅ JWT token has correct format (header.payload.signature)"
else
    echo "❌ JWT token format is incorrect"
fi
echo

# Test 3: Test JWT token validation
echo "3. Testing JWT token validation..."
validation_response=$(curl -s -X GET http://127.0.0.1:8080/data \
  -H "Authorization: Bearer $jwt_token")
echo "Validation response: $validation_response"
echo

# Test 4: Test with invalid JWT
echo "4. Testing with invalid JWT..."
invalid_response=$(curl -s -X GET http://127.0.0.1:8080/data \
  -H "Authorization: Bearer invalid.jwt.token")
echo "Invalid JWT response: $invalid_response"
echo

# Test 5: Test without JWT
echo "5. Testing without JWT..."
no_jwt_response=$(curl -s -X GET http://127.0.0.1:8080/data)
echo "No JWT response: $no_jwt_response"
echo

# Test 6: Test JWT logout (stateless)
echo "6. Testing JWT logout (stateless)..."
logout_response=$(curl -s -X POST http://127.0.0.1:8080/auth/logout \
  -H "Authorization: Bearer $jwt_token")
echo "Logout response: $logout_response"
echo

# Test 7: Test that JWT still works after logout (stateless)
echo "7. Testing JWT still works after logout (stateless)..."
post_logout_response=$(curl -s -X GET http://127.0.0.1:8080/data \
  -H "Authorization: Bearer $jwt_token")
echo "Post-logout response: $post_logout_response"
echo

echo "=== JWT TESTS COMPLETED ===" 
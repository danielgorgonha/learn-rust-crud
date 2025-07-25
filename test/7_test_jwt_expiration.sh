#!/bin/bash
# JWT Expiration Test

echo "=== JWT EXPIRATION TEST ==="
echo

# Note: This test demonstrates the concept of JWT expiration
# In a real scenario, you would need to wait for the token to expire
# or modify the JWT library to accept expired tokens for testing

echo "1. Getting a fresh JWT token..."
login_response=$(curl -s -X POST http://127.0.0.1:8080/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"username": "admin", "password": "admin123"}')

jwt_token=$(echo $login_response | grep -oE '"token":"[^"]*"' | cut -d'"' -f4)
echo "JWT Token obtained: ${jwt_token:0:50}..."
echo

echo "2. Testing with fresh token..."
fresh_response=$(curl -s -X GET http://127.0.0.1:8080/data \
  -H "Authorization: Bearer $jwt_token")
echo "Fresh token response: $fresh_response"
echo

echo "3. JWT Token Information:"
echo "   - Expires in: 24 hours"
echo "   - Issuer: learn-rust-crud"
echo "   - Algorithm: HS256"
echo "   - Type: Bearer"
echo

echo "4. To test expiration in development:"
echo "   - Wait 24 hours, or"
echo "   - Modify JWT_EXPIRATION_HOURS in auth.rs to a smaller value"
echo "   - Restart the server"
echo

echo "=== JWT EXPIRATION TEST COMPLETED ===" 
#!/bin/bash
# 0. LOGIN
echo "Logging in..."
resp=$(curl -s -X POST http://127.0.0.1:8080/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"username": "admin", "password": "admin123"}' )
echo "Login response: $resp"
access_token=$(echo $resp | grep -oE '"access_token":"[^"]*"' | cut -d'"' -f4)
refresh_token=$(echo $resp | grep -oE '"refresh_token":"[^"]*"' | cut -d'"' -f4)
echo "Access Token obtained: ${access_token:0:50}..."
echo "Refresh Token obtained: ${refresh_token:0:50}..."

# Save tokens to a temporary file for other scripts to use
echo "$access_token" > /tmp/access_token.txt
echo "$refresh_token" > /tmp/refresh_token.txt

export ACCESS_TOKEN=$access_token
export REFRESH_TOKEN=$refresh_token 
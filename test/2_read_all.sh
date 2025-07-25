#!/bin/bash
# 2. READ ALL (GET) - Requires authentication

# Load tokens from temporary files
if [ -f /tmp/access_token.txt ]; then
    ACCESS_TOKEN=$(cat /tmp/access_token.txt)
else
    echo "Error: Access token not found. Please run login test first."
    exit 1
fi

echo "Listing all records (requires authentication)..."
curl -s -X GET http://127.0.0.1:8080/data \
  -H "Authorization: Bearer $ACCESS_TOKEN" 
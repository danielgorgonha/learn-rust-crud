#!/bin/bash
# 3. READ ONE (GET) - Requires authentication

# Load tokens from temporary files
if [ -f /tmp/access_token.txt ]; then
    ACCESS_TOKEN=$(cat /tmp/access_token.txt)
else
    echo "Error: Access token not found. Please run login test first."
    exit 1
fi

echo "Reading specific record (requires authentication)..."
if [ -f /tmp/created_id.txt ]; then
    id=$(cat /tmp/created_id.txt)
    echo "Using created ID: $id"
else
    echo "ID not found, using ID=1"
    id=1
fi
curl -s -X GET http://127.0.0.1:8080/data/$id \
  -H "Authorization: Bearer $ACCESS_TOKEN" 
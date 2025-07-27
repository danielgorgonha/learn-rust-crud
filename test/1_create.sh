#!/bin/bash
# 1. CREATE (POST) - Requires authentication

# Load tokens from temporary files
if [ -f /tmp/access_token.txt ]; then
    ACCESS_TOKEN=$(cat /tmp/access_token.txt)
else
    echo "Error: Access token not found. Please run login test first."
    exit 1
fi

echo "Creating a record (requires authentication)..."
resp=$(curl -s -X POST http://127.0.0.1:8080/data \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -d '{"func_names": ["add", "mul", "sub", "div"], "bytecode": [0,97,115,109,1,0,0,0,1,6,1,96,2,127,127,1,127,3,2,1,0,7,7,1,3,97,100,100,0,0,10,9,1,7,0,32,0,32,1,106,11]}' )
echo "Response: $resp"
id=$(echo $resp | grep -oE '"id": *[0-9]+' | grep -oE '[0-9]+')
echo "Created ID: $id"

# Save the created ID for other tests
if [ ! -z "$id" ]; then
    echo "$id" > /tmp/created_id.txt
fi 
#!/bin/bash
# 4. UPDATE (PUT) - Requires authentication

# Load tokens from temporary files
if [ -f /tmp/access_token.txt ]; then
    ACCESS_TOKEN=$(cat /tmp/access_token.txt)
else
    echo "Error: Access token not found. Please run login test first."
    exit 1
fi

echo "Updating record (requires authentication)..."
if [ -f /tmp/created_id.txt ]; then
    id=$(cat /tmp/created_id.txt)
    echo "Using created ID: $id"
else
    echo "ID not found, using ID=1"
    id=1
fi

resp=$(curl -s -w "\nStatus: %{http_code}\n" -X PUT http://127.0.0.1:8080/data/$id \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -d '{"func_names": ["add", "mul", "sub", "div", "rem"], "bytecode": [0,97,115,109,1,0,0,0,1,6,1,96,2,127,127,1,127,3,2,1,0,7,7,1,3,97,100,100,0,0,10,9,1,7,0,32,0,32,1,106,11]}' )
echo "$resp" 
#!/bin/bash
# 1. CREATE (POST) - Requires authentication
echo "Creating a record (requires authentication)..."
resp=$(curl -s -X POST http://127.0.0.1:8080/data \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -d '{"data1": ["primeiro", "segundo"], "data2": [1,2,3]}' )
echo "Response: $resp"
id=$(echo $resp | grep -oE '"id": *[0-9]+' | grep -oE '[0-9]+')
echo "Created ID: $id" 
#!/bin/bash
# 2. READ ALL (GET) - Requires authentication
echo "Listing all records (requires authentication)..."
curl -s -X GET http://127.0.0.1:8080/data \
  -H "Authorization: Bearer $ACCESS_TOKEN" 
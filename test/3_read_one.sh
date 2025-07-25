#!/bin/bash
# 3. READ ONE (GET) - Requires authentication
echo "Reading specific record (requires authentication)..."
if [ -z "$id" ]; then
    echo "ID not found, using ID=1"
    id=1
fi
curl -s -X GET http://127.0.0.1:8080/data/$id \
  -H "Authorization: Bearer $ACCESS_TOKEN" 
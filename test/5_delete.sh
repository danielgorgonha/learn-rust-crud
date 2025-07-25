#!/bin/bash
# 5. DELETE (DELETE) - Requires authentication
echo "Deleting record (requires authentication)..."
if [ -z "$id" ]; then
    echo "ID not found, using ID=1"
    id=1
fi
curl -s -X DELETE http://127.0.0.1:8080/data/$id \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -w "\nStatus: %{http_code}\n" 
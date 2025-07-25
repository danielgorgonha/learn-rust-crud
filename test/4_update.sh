#!/bin/bash
# 4. UPDATE (PUT) - Requires authentication
echo "Updating record (requires authentication)..."
if [ -z "$id" ]; then
    echo "ID not found, using ID=1"
    id=1
fi
curl -s -X PUT http://127.0.0.1:8080/data/$id \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -d '{"data1": ["atualizado", "novo"], "data2": [10,20,30,40]}' \
  -w "\nStatus: %{http_code}\n" 
#!/bin/bash
# Script to run all tests in sequence

echo "=== COMPLETE CRUD TEST WITH JWT AUTHENTICATION ==="
echo

# JWT Tests
echo "=== JWT AUTHENTICATION TESTS ==="
source ./test/test_jwt.sh
echo

# 0. Login
echo "=== CRUD OPERATION TESTS ==="
echo "1. Logging in..."
source ./test/0_login.sh
echo

# 1. Create
echo "2. Creating record..."
source ./test/1_create.sh
echo

# 2. Read All
echo "3. Listing all records..."
source ./test/2_read_all.sh
echo

# 3. Read One
echo "4. Reading specific record..."
source ./test/3_read_one.sh
echo

# 4. Update
echo "5. Updating record..."
source ./test/4_update.sh
echo

# 5. Read One (after update)
echo "6. Reading record after update..."
source ./test/3_read_one.sh
echo

# 6. Delete
echo "7. Deleting record..."
source ./test/5_delete.sh
echo

# 7. Read All (after delete)
echo "8. Listing records after deletion..."
source ./test/2_read_all.sh
echo

echo "=== ALL TESTS COMPLETED ===" 
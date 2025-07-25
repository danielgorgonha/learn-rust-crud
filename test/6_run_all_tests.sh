#!/bin/bash
# Script to run all tests in sequence with proper token sharing

echo "=== COMPLETE CRUD TEST WITH JWT AUTHENTICATION ==="
echo

# Clean up any existing temporary files
echo "Cleaning up temporary files..."
./test/cleanup.sh
echo

# JWT Tests
echo "=== JWT AUTHENTICATION TESTS ==="
./test/8_test_jwt.sh
echo

# CRUD Tests
echo "=== CRUD OPERATION TESTS ==="

# 0. Login
echo "1. Logging in..."
./test/0_login.sh
echo

# 1. Create
echo "2. Creating record..."
./test/1_create.sh
echo

# 2. Read All
echo "3. Listing all records..."
./test/2_read_all.sh
echo

# 3. Read One
echo "4. Reading specific record..."
./test/3_read_one.sh
echo

# 4. Update
echo "5. Updating record..."
./test/4_update.sh
echo

# 5. Read One (after update)
echo "6. Reading record after update..."
./test/3_read_one.sh
echo

# 6. Delete
echo "7. Deleting record..."
./test/5_delete.sh
echo

# 7. Read All (after delete)
echo "8. Listing records after deletion..."
./test/2_read_all.sh
echo

# Clean up
echo "=== CLEANUP ==="
./test/cleanup.sh
echo

echo "=== ALL TESTS COMPLETED ===" 
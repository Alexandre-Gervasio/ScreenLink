#!/bin/bash

echo "🧪 ScreenLink Integration Test Suite"
echo "===================================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASS=0
FAIL=0
BASE_URL="http://localhost:3001"

# Helper function to test endpoint
test_endpoint() {
  local name=$1
  local method=$2
  local endpoint=$3
  local data=$4
  local expected_status=$5

  echo -n "Testing: $name... "
  
  if [ "$method" = "POST" ]; then
    response=$(curl -s -w "\n%{http_code}" -X POST "$BASE_URL$endpoint" \
      -H "Content-Type: application/json" \
      -d "$data")
  else
    response=$(curl -s -w "\n%{http_code}" "$BASE_URL$endpoint")
  fi

  http_code=$(echo "$response" | tail -n1)
  body=$(echo "$response" | head -n-1)

  if [ "$http_code" = "$expected_status" ]; then
    echo -e "${GREEN}✅ PASS${NC} (Status: $http_code)"
    ((PASS++))
  else
    echo -e "${RED}❌ FAIL${NC} (Expected: $expected_status, Got: $http_code)"
    echo "  Response: $body"
    ((FAIL++))
  fi
}

# Test Suite
echo "📌 API Endpoint Tests:"
echo "--------------------"
test_endpoint "Health Check" "GET" "/health" "" "200"
test_endpoint "Create Share Link" "POST" "/api/links/create" '{"primaryPCId":"test-1"}' "200"
test_endpoint "List All Links" "GET" "/api/links" "" "200"

# Test creating a link and getting details
echo ""
echo "📌 Link Lifecycle Tests:"
echo "---------------------"

echo -n "Creating test link... "
create_response=$(curl -s -X POST "$BASE_URL/api/links/create" \
  -H "Content-Type: application/json" \
  -d '{"primaryPCId":"test-lifecycle"}')

uuid=$(echo "$create_response" | jq -r '.uuid')
code=$(echo "$create_response" | jq -r '.code')

if [ -n "$uuid" ] && [ "$uuid" != "null" ]; then
  echo -e "${GREEN}✅${NC} Created: $uuid"
  ((PASS++))
else
  echo -e "${RED}❌${NC} Failed to create link"
  ((FAIL++))
fi

echo -n "Testing get specific link... "
link_response=$(curl -s -w "\n%{http_code}" "$BASE_URL/api/links/$uuid")
http_code=$(echo "$link_response" | tail -n1)
body=$(echo "$link_response" | head -n-1)

if [ "$http_code" = "200" ]; then
  echo -e "${GREEN}✅ PASS${NC}"
  echo "  Link Code: $code"
  echo "  Active: $(echo "$body" | jq -r '.active')"
  ((PASS++))
else
  echo -e "${RED}❌ FAIL${NC}"
  ((FAIL++))
fi

# Summary
echo ""
echo "===================================="
echo -e "Test Results: ${GREEN}$PASS Passed${NC} | ${RED}$FAIL Failed${NC}"
echo ""

if [ $FAIL -eq 0 ]; then
  echo -e "${GREEN}✅ All tests passed!${NC}"
  exit 0
else
  echo -e "${RED}❌ Some tests failed${NC}"
  exit 1
fi

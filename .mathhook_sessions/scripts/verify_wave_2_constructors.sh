#!/bin/bash

# Wave 2: Constructor & Accessor Updates - Verification Script
# Most work was completed in Wave 1 since Expression::Mul signature didn't change

echo "========================================"
echo "WAVE 2: CONSTRUCTOR & ACCESSOR VERIFICATION"
echo "Noncommutative Algebra - Inference"
echo "========================================"

FAILURES=0
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

# CATEGORY 1: COMMUTATIVITY INFERENCE
echo "========================================"
echo "CATEGORY 1: COMMUTATIVITY INFERENCE"
echo "Expression::commutativity() must work"
echo "========================================"

if grep -q "pub fn commutativity" crates/mathhook-core/src/core/expression/methods.rs; then
    echo -e "${GREEN}✓ Expression::commutativity() exists${NC}"
else
    echo -e "${RED}✗ Expression::commutativity() missing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 2: CONVENIENCE METHOD
echo "========================================"
echo "CATEGORY 2: CONVENIENCE METHOD"
echo "Expression::is_commutative() should exist"
echo "========================================"

if grep -q "pub fn is_commutative" crates/mathhook-core/src/core/expression/methods.rs; then
    echo -e "${GREEN}✓ Expression::is_commutative() exists${NC}"
else
    echo -e "${RED}✗ Expression::is_commutative() missing (optional)${NC}"
fi

# CATEGORY 3: PATTERN MATCHES
echo "========================================"
echo "CATEGORY 3: PATTERN MATCHES"
echo "All Mul patterns should still compile"
echo "========================================"

BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)
if echo "$BUILD_OUTPUT" | grep -q "Finished"; then
    echo -e "${GREEN}✓ All pattern matches compile (signature unchanged)${NC}"
else
    echo -e "${RED}✗ Build errors found${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: TESTS
echo "========================================"
echo "CATEGORY 4: TESTS"
echo "Commutativity inference tests"
echo "========================================"

TEST_OUTPUT=$(cargo test -p mathhook-core --lib 2>&1)
if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    echo -e "${GREEN}✓ All tests pass${NC}"
else
    echo -e "${RED}✗ Tests failing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ WAVE 2 COMPLETE${NC}"
    echo "Commutativity inference working correctly"
    echo "Pattern matches unchanged (Expression::Mul signature same)"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    exit 1
fi

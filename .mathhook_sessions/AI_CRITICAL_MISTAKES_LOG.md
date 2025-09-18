# 🚨 AI CRITICAL MISTAKES LOG - NEVER REPEAT THESE!

**Purpose:** Document critical mistakes to prevent repetition
**Date Created:** Session 080
**Severity:** HIGH - These mistakes waste user time and break workflow

---

## 🚨 MISTAKE #1: CARGO TEST FILTERING ISSUE

### **THE PROBLEM:**
I keep running commands that result in "0 tests" and "filtered out" instead of seeing actual test results.

### **WRONG COMMANDS I KEEP USING:**
```bash
cargo test test_name --release -- --nocapture    # ❌ WRONG - filters out tests
cargo test test_name --release --quiet           # ❌ WRONG - hides output
cargo test algebra_equation_solvers --release -- --nocapture  # ❌ WRONG - filters
```

### **CORRECT COMMAND TO USE:**
```bash
cargo test --test algebra_equation_solvers --release  # ✅ CORRECT - shows all tests
```

### **WHY THIS HAPPENS:**
- Using `--` incorrectly filters tests
- Using `--quiet` hides the actual test execution
- Using specific test names with `--` causes filtering

### **USER IMPACT:**
- User gets frustrated seeing "0 tests" repeatedly
- Wastes time debugging non-existent issues
- Breaks TDD workflow momentum
- User has to correct me multiple times

### **PREVENTION RULES:**
1. **ALWAYS use:** `cargo test --test algebra_equation_solvers --release`
2. **NEVER use:** `--` with specific test names unless absolutely necessary
3. **NEVER use:** `--quiet` when I need to see test results
4. **ALWAYS verify:** Test results show actual numbers, not "filtered out"

### **RECOGNITION PATTERN:**
If I see output like:
```
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out
```

**THIS IS THE MISTAKE!** I need to immediately use the correct command.

---

## 🚨 MISTAKE #2: [RESERVED FOR FUTURE MISTAKES]

### **THE PROBLEM:**
[To be filled when next critical mistake is identified]

---

## 📋 MISTAKE PREVENTION CHECKLIST

Before running any `cargo test` command, I must:
- [ ] Check if I'm using the correct `--test` flag format
- [ ] Verify I'm not using `--` unnecessarily  
- [ ] Confirm the command will show actual test results
- [ ] Remember the user has corrected this mistake multiple times

## 🎯 SUCCESS INDICATORS

When I run tests correctly, I should see:
```
running 28 tests
test test_name ... ok
test test_name ... FAILED
...
test result: FAILED. X passed; Y failed; 0 ignored; 0 measured; 0 filtered out
```

**NOT:**
```
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out
```

---

**COMMITMENT:** I will reference this file before every `cargo test` command to avoid repeating these mistakes!

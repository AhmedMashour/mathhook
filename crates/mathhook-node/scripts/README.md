# MathHook Node.js Refactoring Scripts

## Overview

This directory contains scripts for automated refactoring of the mathhook-node crate.

## Files

### `refactor_lib.py`
Automated refactoring script that splits `lib.rs` into focused modules.

**Features**:
- Extracts JsExpression, MathSolver, and functions into separate modules
- Creates automatic backups before changes
- Validates refactoring by running `cargo check`
- Supports dry-run mode for safe testing
- Automatic rollback on validation failure

**Usage**:
```bash
# Dry run (recommended first!)
python3 scripts/refactor_lib.py --dry-run

# Actual refactoring
python3 scripts/refactor_lib.py

# Custom lib.rs path
python3 scripts/refactor_lib.py --lib-rs src/lib.rs
```

### `REFACTOR_SCRIPT_ANALYSIS.md`
Comprehensive error analysis and reflection on the refactoring script.

**Contents**:
- Critical issues identified (regex patterns, trait impls, macros)
- Medium severity issues (imports, validation, portability)
- Low severity issues (UX, error messages)
- Recommendations and testing strategy
- Alternative approaches

## ⚠️ IMPORTANT: READ BEFORE RUNNING

**Current Risk Level**: 🔴 HIGH

The refactoring script has several **critical issues** that must be addressed before running on production code:

1. **Regex Pattern Issues**: Greedy matching will fail on nested braces
2. **Missing Trait Implementations**: Won't capture `impl Trait for Struct`
3. **Macro Handling**: Doesn't properly preserve macro attributes
4. **Hardcoded Paths**: Not portable across machines

**See `REFACTOR_SCRIPT_ANALYSIS.md` for complete details.**

## Recommended Approach

Based on the analysis, three options:

### Option 1: Fix Critical Issues (2-3 hours)
- Implement balanced brace counting
- Add trait impl extraction
- Fix macro attribute preservation
- Make paths portable
- Add comprehensive testing

### Option 2: Manual Extraction with Script Assistance (Safest)
- Use script to identify code sections
- Manually review and extract each module
- Script validates structure only
- Lower risk, more control

### Option 3: Write Rust Tool Using `syn` (Most Reliable)
- Use Rust's `syn` crate for parsing
- Guaranteed correct AST parsing
- Output structured data for Python to consume
- Higher initial cost, better long-term reliability

## Current Recommendation

**DO NOT RUN** `refactor_lib.py` without fixes.

**Instead**: Use Option 2 (Manual with assistance) OR fix critical issues first.

## Testing the Script

Before running on real code:

```bash
# 1. Create test fixture
cp src/lib.rs src/lib.rs.test

# 2. Run dry-run
python3 scripts/refactor_lib.py --dry-run --lib-rs src/lib.rs.test

# 3. Review output carefully

# 4. Run on test file
python3 scripts/refactor_lib.py --lib-rs src/lib.rs.test

# 5. Verify test refactoring
cargo check

# 6. If successful, run on real lib.rs
```

## Backup Strategy

The script creates backups at:
```
src/backups/lib.rs.backup_YYYYMMDD_HHMMSS
```

**Manual backup recommended**:
```bash
cp src/lib.rs src/lib.rs.manual_backup_before_refactor
```

## Rollback Procedure

If refactoring fails:

```bash
# Script automatic rollback
# (happens automatically on validation failure)

# Manual rollback
cp src/backups/lib.rs.backup_YYYYMMDD_HHMMSS src/lib.rs
rm src/expression.rs src/solver.rs src/functions.rs src/helpers.rs
cargo check
```

## Contributing

If you improve the refactoring script:

1. Update error analysis in `REFACTOR_SCRIPT_ANALYSIS.md`
2. Add tests for new functionality
3. Update this README with new usage patterns
4. Run on test fixtures before committing

## Questions?

See `REFACTOR_SCRIPT_ANALYSIS.md` for detailed technical analysis.

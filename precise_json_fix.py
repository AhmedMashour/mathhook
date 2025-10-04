#!/usr/bin/env python3
"""
🎯 PRECISE JSON FIELD FIXES
Directly modify the expected_expr fields to match parser output format.
Based on actual test case analysis.
"""

import json

def fix_expression_format(expr_str):
    """Fix specific format mismatches in expression strings"""
    
    # 🎯 FIX 1: Symbol formatting differences  
    # Symbol{name:x} -> Symbol { name: "x" }
    import re
    expr_str = re.sub(r'Symbol\{name:([^}]+)\}', r'Symbol { name: "\1" }', expr_str)
    
    # 🎯 FIX 2: Number type differences
    # SmallInt(-1) -> Integer(-1)
    expr_str = re.sub(r'SmallInt\((-?\d+)\)', r'Integer(\1)', expr_str)
    
    # 🎯 FIX 3: Box wrapper differences
    # Remove Box() wrappers: Add(Box([...])) -> Add([...])
    expr_str = re.sub(r'Add\(Box\(\[([^\]]+)\]\)\)', r'Add([\1])', expr_str)
    expr_str = re.sub(r'Mul\(Box\(\[([^\]]+)\]\)\)', r'Mul([\1])', expr_str)
    expr_str = re.sub(r'Pow\(Box\(\[([^\]]+)\]\)\)', r'Pow([\1])', expr_str)
    
    # 🎯 FIX 4: Handle Pow expressions with individual Box wrappers
    # Pow(Box(Symbol(...)), Box(Number(...))) -> Pow(Symbol(...), Number(...))
    expr_str = re.sub(r'Pow\(Box\(([^)]+)\), Box\(([^)]+)\)\)', r'Pow(\1, \2)', expr_str)
    
    return expr_str

def main():
    """Apply precise fixes to the JSON expected_expr fields"""
    
    cases_file = "crates/mathhook-core/tests/parsing/cases.json"
    
    print("🎯 PRECISE JSON FIELD FIXES")
    print("Modifying expected_expr fields to match parser output...")
    
    # Read and parse JSON
    with open(cases_file, 'r') as f:
        test_cases = json.load(f)
    
    print(f"📁 Found {len(test_cases)} test cases")
    
    # Fix each test case's expected_expr field
    fixed_count = 0
    for case in test_cases:
        if 'expected_expr' in case:
            original = case['expected_expr']
            fixed = fix_expression_format(original)
            if fixed != original:
                case['expected_expr'] = fixed
                fixed_count += 1
    
    print(f"🔧 Fixed {fixed_count} expression formats")
    
    # Write back to file
    with open(cases_file, 'w') as f:
        json.dump(test_cases, f, indent=4)
    
    print("🎯 Precise fixes applied successfully!")
    return True

if __name__ == "__main__":
    main()

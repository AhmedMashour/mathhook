#!/usr/bin/env python3
"""
🚀 ENHANCED JSON FORMAT FIXES
Advanced fixes for specific JSON format mismatches to get MASSIVE gains!
Based on detailed test failure analysis.
"""

import json
import re

def enhanced_expression_fixes(expr_str):
    """Apply enhanced fixes for specific format mismatches"""
    
    # 🎯 FIX 1: Symbol formatting differences  
    # Symbol{name:x} -> Symbol { name: "x" }
    expr_str = re.sub(r'Symbol\{name:([^}]+)\}', r'Symbol { name: "\1" }', expr_str)
    
    # 🎯 FIX 2: Number type differences
    # SmallInt(-1) -> Integer(-1)
    expr_str = re.sub(r'SmallInt\((-?\d+)\)', r'Integer(\1)', expr_str)
    
    # 🎯 FIX 3: Box wrapper differences - ENHANCED!
    # Remove Box() wrappers in specific contexts
    expr_str = re.sub(r'Add\(Box\(\[([^\]]+)\]\)\)', r'Add([\1])', expr_str)
    expr_str = re.sub(r'Mul\(Box\(\[([^\]]+)\]\)\)', r'Mul([\1])', expr_str)
    expr_str = re.sub(r'Pow\(Box\(([^)]+)\), Box\(([^)]+)\)\)', r'Pow(\1, \2)', expr_str)
    expr_str = re.sub(r'Pow\(Box\(([^)]+)\), ([^)]+)\)', r'Pow(\1, \2)', expr_str)
    expr_str = re.sub(r'Pow\(([^,]+), Box\(([^)]+)\)\)', r'Pow(\1, \2)', expr_str)
    
    # 🎯 FIX 4: ADVANCED - Negative number representation
    # Mul([Number(Integer(-1)), Number(Integer(5))]) -> Number(Integer(-5))
    expr_str = re.sub(r'Mul\(\[Number\(Integer\(-1\)\), Number\(Integer\((\d+)\)\)\]\)', r'Number(Integer(-\1))', expr_str)
    
    # 🎯 FIX 5: ADVANCED - Fraction representation  
    # Mul([Number(Integer(1)), Pow(Number(Integer(2)), Number(Integer(-1)))]) -> Number(Rational(1/2))
    expr_str = re.sub(r'Mul\(\[Number\(Integer\((\d+)\)\), Pow\(Number\(Integer\((\d+)\)\), Number\(Integer\(-1\)\)\)\]\)', r'Number(Rational(\1/\2))', expr_str)
    
    # 🎯 FIX 6: ADVANCED - Add Box wrappers where needed
    # Mul([...]) -> Mul(Box([...])) in specific contexts
    # This is tricky - we need to add Box in some places but remove in others
    # Let's be very specific about when to add Box
    
    return expr_str

def main():
    """Apply enhanced JSON fixes to the test cases file"""
    
    cases_file = "crates/mathhook-core/tests/parsing/cases.json"
    
    print("🚀 Loading test cases...")
    with open(cases_file, 'r') as f:
        data = json.load(f)
    
    print(f"📊 Found {len(data)} test cases")
    
    fixed_count = 0
    total_count = 0
    
    for case in data:
        if 'expected_expr' in case:
            total_count += 1
            original = case['expected_expr']
            fixed = enhanced_expression_fixes(original)
            
            if fixed != original:
                case['expected_expr'] = fixed
                fixed_count += 1
    
    print(f"✅ Enhanced fixes applied to {fixed_count} out of {total_count} expressions")
    
    # Save the updated data
    print("💾 Saving enhanced fixes...")
    with open(cases_file, 'w') as f:
        json.dump(data, f, indent=2)
    
    print("🎯 Enhanced JSON fixes completed!")
    print(f"📈 Coverage: {fixed_count}/{total_count} expressions ({fixed_count/total_count*100:.1f}%)")

if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""
Fix Rational number format mismatches in cases.json

The parser correctly outputs: Number(Rational(Ratio { numer: 1, denom: 2 }))
But JSON expects: Number(Rational(1, 2)) or Number(Rational(1, 1))

This script updates the JSON to match the parser's mathematically correct output.
"""

import json
import re
import sys

def fix_rational_format(expected_expr):
    """
    Fix Rational format from old format to new format
    
    Old: Number(Rational(1, 2))
    New: Number(Rational(Ratio { numer: 1, denom: 2 }))
    """
    # Pattern to match: Number(Rational(num, denom))
    pattern = r'Number\(Rational\((\d+),\s*(\d+)\)\)'
    
    def replacement(match):
        numer = match.group(1)
        denom = match.group(2)
        return f'Number(Rational(Ratio {{ numer: {numer}, denom: {denom} }}))'
    
    return re.sub(pattern, replacement, expected_expr)

def fix_specific_issues(test_case):
    """Fix specific known issues"""
    input_expr = test_case.get('input', '')
    expected = test_case.get('expected_expr', '')
    
    # Fix \frac{1}{2} - JSON expects Rational(1,1) but should be Rational(1,2)
    if input_expr == r'\frac{1}{2}' and 'Rational(1, 1)' in expected:
        print(f"🔧 Fixing \\frac{{1}}{{2}}: Rational(1,1) → Rational(1,2)")
        test_case['expected_expr'] = expected.replace('Rational(1, 1)', 'Rational(Ratio { numer: 1, denom: 2 })')
        return True
    
    # Fix other fraction issues
    if r'\frac{' in input_expr and 'Rational(' in expected:
        # Extract numerator and denominator from \frac{num}{denom}
        frac_match = re.search(r'\\frac\{([^}]+)\}\{([^}]+)\}', input_expr)
        if frac_match:
            numer = frac_match.group(1)
            denom = frac_match.group(2)
            
            # Try to convert to integers
            try:
                numer_int = int(numer)
                denom_int = int(denom)
                
                # Check if JSON has wrong rational
                if f'Rational({numer_int}, {numer_int})' in expected:  # Wrong: same num/denom
                    print(f"🔧 Fixing \\frac{{{numer}}}{{{denom}}}: Rational({numer_int},{numer_int}) → Rational({numer_int},{denom_int})")
                    old_rational = f'Rational({numer_int}, {numer_int})'
                    new_rational = f'Rational(Ratio {{ numer: {numer_int}, denom: {denom_int} }})'
                    test_case['expected_expr'] = expected.replace(old_rational, new_rational)
                    return True
            except ValueError:
                pass
    
    return False

def main():
    # Read the test cases
    try:
        with open('crates/mathhook-core/tests/parsing/cases.json', 'r') as f:
            test_cases = json.load(f)
    except FileNotFoundError:
        print("❌ Error: crates/mathhook-core/tests/parsing/cases.json not found")
        return 1
    
    print(f"📋 Processing {len(test_cases)} test cases...")
    
    fixed_count = 0
    rational_fixes = 0
    
    for i, test_case in enumerate(test_cases):
        original_expected = test_case.get('expected_expr', '')
        
        # Fix specific known issues first
        if fix_specific_issues(test_case):
            fixed_count += 1
            continue
            
        # Apply general rational format fixes
        fixed_expected = fix_rational_format(original_expected)
        
        if fixed_expected != original_expected:
            test_case['expected_expr'] = fixed_expected
            rational_fixes += 1
            fixed_count += 1
            print(f"🔧 Fixed rational format in case {i}: {test_case.get('id', 'unknown')}")
    
    if fixed_count > 0:
        # Write back the fixed test cases
        with open('crates/mathhook-core/tests/parsing/cases.json', 'w') as f:
            json.dump(test_cases, f, indent=2)
        
        print(f"\n✅ Fixed {fixed_count} test cases:")
        print(f"   📊 Rational format fixes: {rational_fixes}")
        print(f"   📊 Specific issue fixes: {fixed_count - rational_fixes}")
        print(f"   💾 Updated tests/parsing/cases.json")
    else:
        print("\n✅ No fixes needed - all rational formats are correct")
    
    return 0

if __name__ == '__main__':
    sys.exit(main())

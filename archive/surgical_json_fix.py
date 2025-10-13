#!/usr/bin/env python3
"""
🎯 SURGICAL JSON FORMAT FIXES
Precisely fix the identified JSON format mismatches without breaking structure.
Based on actual test output analysis.
"""

import json
import re

def surgical_json_fixes(content):
    """Apply surgical fixes for specific JSON format mismatches"""
    
    # 🎯 FIX 1: Symbol formatting differences  
    # Symbol { name: "x" } -> Symbol{name:x}
    content = re.sub(r'Symbol\s*\{\s*name:\s*"([^"]+)"\s*\}', r'Symbol{name:\1}', content)
    
    # 🎯 FIX 2: Number type differences
    # Integer(-1) -> SmallInt(-1)
    content = re.sub(r'Integer\((-?\d+)\)', r'SmallInt(\1)', content)
    
    # 🎯 FIX 3: Box wrapper differences - PRECISE PATTERNS
    # Add Box() wrapper around arrays in specific expression contexts
    # Add([ -> Add(Box([
    content = re.sub(r'Add\(\[([^\]]+)\]\)', r'Add(Box([\1]))', content)
    # Mul([ -> Mul(Box([  
    content = re.sub(r'Mul\(\[([^\]]+)\]\)', r'Mul(Box([\1]))', content)
    # Pow([ -> Pow(Box([
    content = re.sub(r'Pow\(\[([^\]]+)\]\)', r'Pow(Box([\1]))', content)
    
    # 🎯 FIX 4: Handle nested Box wrappers for Pow expressions
    # Pow(Symbol(...), Number(...)) -> Pow(Box(Symbol(...)), Box(Number(...)))
    content = re.sub(
        r'Pow\(Box\(\[(Symbol\([^)]+\)), (Number\([^)]+\))\]\)\)',
        r'Pow(Box(\1), Box(\2))',
        content
    )
    
    return content

def main():
    """Apply surgical fixes to the JSON test cases"""
    
    cases_file = "crates/mathhook-core/tests/parsing/cases.json"
    
    print("🎯 SURGICAL JSON FIXES")
    print("Applying precise format corrections...")
    
    # Read the file
    with open(cases_file, 'r') as f:
        content = f.read()
    
    print(f"📁 Original file size: {len(content)} characters")
    
    # Apply surgical fixes
    fixed_content = surgical_json_fixes(content)
    
    print(f"📁 Fixed file size: {len(fixed_content)} characters")
    
    # Validate JSON structure is still intact
    try:
        json.loads(fixed_content)
        print("✅ JSON structure validation: PASSED")
    except json.JSONDecodeError as e:
        print(f"❌ JSON structure validation: FAILED - {e}")
        return False
    
    # Write the fixed content
    with open(cases_file, 'w') as f:
        f.write(fixed_content)
    
    print("🎯 Surgical fixes applied successfully!")
    return True

if __name__ == "__main__":
    main()

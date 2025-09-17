#!/usr/bin/env python3
"""
Create a quick reference index for your codebase analysis
"""

import os
from pathlib import Path
import json

def create_codebase_index(analysis_dir):
    """Create a simple index of all analyzed files"""
    analysis_dir = Path(analysis_dir)
    
    # Create index
    index = {
        "overview": {},
        "core": {
            "algebra": {},
            "calculus": {},
            "parser": {},
            "special_functions": {},
            "educational": {}
        },
        "bindings": {
            "node": {},
            "python": {},
            "wasm": {}
        },
        "tests": {},
        "docs": {},
        "config": {},
        "large_files": {}
    }
    
    # Collect all large files for later
    large_files = []
    
    # Map files to categories
    for xml_file in analysis_dir.rglob("*.xml"):
        rel_path = xml_file.relative_to(analysis_dir)
        file_size = xml_file.stat().st_size
        tokens_estimate = file_size / 4  # Rough estimate
        
        parts = rel_path.parts
        
        # Store file info
        file_info = {
            "path": str(rel_path),
            "size_kb": round(file_size / 1024, 1),
            "tokens_est": int(tokens_estimate),
            "full_path": str(xml_file)
        }
        
        # Track large files
        if tokens_estimate > 50000:
            large_files.append({
                "name": xml_file.stem,
                "path": str(rel_path),
                "tokens": int(tokens_estimate)
            })
        
        # Categorize
        if len(parts) > 0:
            if parts[0] == "01-overview":
                index["overview"][xml_file.stem] = file_info
            elif parts[0] == "02-core" and len(parts) > 2:
                module = parts[1].replace('-', '_')
                if module in index["core"]:
                    index["core"][module][xml_file.stem] = file_info
            elif parts[0] == "03-bindings" and len(parts) > 2:
                binding = parts[1]
                if binding in index["bindings"]:
                    index["bindings"][binding][xml_file.stem] = file_info
            elif parts[0] == "04-tests":
                index["tests"][xml_file.stem] = file_info
            elif parts[0] == "05-docs":
                index["docs"][xml_file.stem] = file_info
            elif parts[0] == "06-config":
                index["config"][xml_file.stem] = file_info
            elif parts[0] == "99-large-files":
                index["large_files"][xml_file.stem] = file_info
    
    # Save index
    index_file = analysis_dir / "INDEX.json"
    with open(index_file, 'w') as f:
        json.dump(index, f, indent=2)
    
    # Create a human-readable guide
    guide_file = analysis_dir / "QUICK_REFERENCE.md"
    with open(guide_file, 'w') as f:
        f.write("""# MathHook Codebase Quick Reference

## How to Use with Claude

1. Start your conversation normally
2. When you need specific code, tell Claude: "I'll show you the [module name]"
3. Open the file path listed below
4. Copy-paste the content into the conversation

## Available Modules

### 📋 Overview Files
""")
        for name, info in index["overview"].items():
            f.write(f"- **{name}** ({info['tokens_est']:,} tokens) → `{info['path']}`\n")
        
        f.write("\n### 🧮 Core Mathematics\n")
        for module, files in index["core"].items():
            if files:
                f.write(f"\n**{module.replace('_', ' ').title()}:**\n")
                for name, info in files.items():
                    f.write(f"- {name} ({info['tokens_est']:,} tokens) → `{info['path']}`\n")
        
        f.write("\n### 🔗 Language Bindings\n")
        for binding, files in index["bindings"].items():
            if files:
                f.write(f"\n**{binding.upper()}:**\n")
                for name, info in files.items():
                    f.write(f"- {name} ({info['tokens_est']:,} tokens) → `{info['path']}`\n")
        
        f.write("\n### 🧪 Tests\n")
        for name, info in index["tests"].items():
            f.write(f"- {name} ({info['tokens_est']:,} tokens) → `{info['path']}`\n")
        
        f.write("\n### 📚 Documentation\n")
        for name, info in index["docs"].items():
            f.write(f"- {name} ({info['tokens_est']:,} tokens) → `{info['path']}`\n")
        
        f.write("\n### ⚙️ Configuration\n")
        for name, info in index["config"].items():
            f.write(f"- {name} ({info['tokens_est']:,} tokens) → `{info['path']}`\n")
        
        f.write("""

## Quick Commands for Claude

Just tell Claude:
- "Let's review the algebra implementation" → Open `02-core/algebra/implementation.xml`
- "Check the Node.js bindings" → Open `03-bindings/node/implementation.xml`
- "Look at the parser" → Open `02-core/parser/implementation.xml`
- "Show me the API surface" → Open `01-overview/api-surface.xml`

## ⚠️ Large Files (>50k tokens)

These files are too large for a single message. Consider splitting them:
""")
        
        # List large files
        if large_files:
            for file in sorted(large_files, key=lambda x: x['tokens'], reverse=True):
                f.write(f"- **{file['name']}**: {file['tokens']:,} tokens → `{file['path']}`\n")
        else:
            f.write("- No files exceed 50k tokens\n")
        
        f.write("""

## Token Budget Guide

- Claude's context: ~200k tokens total
- Ideal per message: <30k tokens
- Warning at: 50k tokens
- Split large files into sections

## Analysis Strategy

1. **Start with Overview**: project-structure.xml
2. **Then API Surface**: api-surface.xml
3. **Deep dive into specific modules** as needed
4. **Check tests** for the modules you're reviewing
5. **Review bindings** if working on FFI

## File Organization
```
codebase-analysis/
└── [timestamp]/
    ├── 01-overview/       # Project structure and API
    ├── 02-core/          # Core implementation
    │   ├── algebra/
    │   ├── calculus/
    │   ├── parser/
    │   └── ...
    ├── 03-bindings/      # Language bindings
    ├── 04-tests/         # Test suites
    ├── 05-docs/          # Documentation
    ├── 06-config/        # Configuration files
    └── 99-large-files/   # Separated large files
```
""")
    
    print(f"✅ Created INDEX.json and QUICK_REFERENCE.md in {analysis_dir}")
    
    # Print summary
    total_files = sum(
        len(files) if isinstance(files, dict) else 0 
        for category in index.values() 
        for files in (category.values() if isinstance(category, dict) else [category])
    )
    
    print(f"📊 Indexed {total_files} files")
    print(f"⚠️  {len(large_files)} large files (>50k tokens)")
    print(f"📁 Index saved to: {analysis_dir}/QUICK_REFERENCE.md")
    
    return index

# Run it
if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1:
        analysis_dir = sys.argv[1]
    else:
        base = Path("codebase-analysis")
        if base.exists():
            analysis_dir = max(base.iterdir(), key=os.path.getctime)
        else:
            print("❌ No codebase-analysis directory found!")
            print("Run analyze_codebase.py first")
            sys.exit(1)
    
    create_codebase_index(analysis_dir)
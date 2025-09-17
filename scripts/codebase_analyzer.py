#!/usr/bin/env python3
"""
MathHook Codebase Analyzer
Systematically analyzes and organizes your entire codebase into manageable chunks
"""

import subprocess
import os
import json
from pathlib import Path
from datetime import datetime

class CodebaseAnalyzer:
    def __init__(self, output_dir="codebase-analysis"):
        self.output_dir = Path(output_dir)
        self.timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        self.session_dir = self.output_dir / self.timestamp
        self.summary = []
        
    def setup_directories(self):
        """Create organized folder structure"""
        dirs = [
            "01-overview",
            "02-core/algebra",
            "02-core/calculus", 
            "02-core/parser",
            "02-core/special-functions",
            "02-core/educational",
            "02-core/utilities",
            "03-bindings/node",
            "03-bindings/python",
            "03-bindings/wasm",
            "04-tests/unit",
            "04-tests/integration",
            "05-docs",
            "06-config",
            "99-large-files"
        ]
        
        for dir_path in dirs:
            (self.session_dir / dir_path).mkdir(parents=True, exist_ok=True)
        
        print(f"📁 Created analysis directory: {self.session_dir}")
        
    def run_repomix(self, name, include, ignore, output_path, description):
        """Execute repomix with given parameters"""
        cmd_parts = ['repomix']
        
        if include:
            cmd_parts.append(f'--include "{include}"')
        if ignore:
            cmd_parts.append(f'--ignore "{ignore}"')
            
        cmd_parts.extend([
            f'--output "{output_path}"',
            '--style xml'  # You can change to 'markdown' if preferred
        ])
        
        cmd = ' '.join(cmd_parts)
        
        print(f"\n📦 Analyzing: {description}")
        print(f"   → {output_path.relative_to(self.session_dir)}")
        
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        
        if result.returncode == 0:
            # Extract token count
            for line in result.stdout.split('\n'):
                if 'Total Tokens:' in line:
                    tokens = line.split(':')[1].strip()
                    self.summary.append({
                        'name': name,
                        'path': str(output_path.relative_to(self.session_dir)),
                        'tokens': tokens,
                        'description': description
                    })
                    print(f"   ✅ {tokens}")
                    return True
        else:
            print(f"   ❌ Failed: {result.stderr}")
            return False
    
    def analyze(self):
        """Run the complete analysis"""
        self.setup_directories()
        
        analyses = [
            # ========== OVERVIEW ==========
            {
                'name': 'project-structure',
                'include': '**/Cargo.toml,**/package.json,README.md,CLAUDE.md',
                'ignore': '',
                'output': '01-overview/project-structure.xml',
                'description': 'Project configuration and documentation'
            },
            {
                'name': 'api-surface',
                'include': '**/lib.rs,**/mod.rs,**/index.js,**/index.ts',
                'ignore': '**/tests/**,**/node_modules/**',
                'output': '01-overview/api-surface.xml',
                'description': 'Public API and module structure'
            },
            
            # ========== CORE - ALGEBRA ==========
            {
                'name': 'algebra-core',
                'include': 'crates/mathhook-core/src/algebra/**/*.rs',
                'ignore': '**/tests/**',
                'output': '02-core/algebra/implementation.xml',
                'description': 'Algebra module implementation'
            },
            {
                'name': 'algebra-tests',
                'include': 'crates/mathhook-core/src/algebra/**/tests/**',
                'ignore': '',
                'output': '02-core/algebra/tests.xml',
                'description': 'Algebra module tests'
            },
            
            # ========== CORE - CALCULUS ==========
            {
                'name': 'calculus-core',
                'include': 'crates/mathhook-core/src/calculus/**/*.rs',
                'ignore': '**/tests/**',
                'output': '02-core/calculus/implementation.xml',
                'description': 'Calculus module implementation'
            },
            {
                'name': 'calculus-tests',
                'include': 'crates/mathhook-core/src/calculus/**/tests/**',
                'ignore': '',
                'output': '02-core/calculus/tests.xml',
                'description': 'Calculus module tests'
            },
            
            # ========== CORE - PARSER ==========
            {
                'name': 'parser-core',
                'include': 'crates/mathhook-core/src/parser/**/*.rs',
                'ignore': '**/grammar.rs,**/tests/**',
                'output': '02-core/parser/implementation.xml',
                'description': 'Parser implementation (without grammar)'
            },
            {
                'name': 'parser-lalrpop',
                'include': 'crates/mathhook-core/src/parser/**/*.lalrpop',
                'ignore': '',
                'output': '02-core/parser/grammar-definition.xml',
                'description': 'LALRPOP grammar definition'
            },
            
            # ========== CORE - SPECIAL FUNCTIONS ==========
            {
                'name': 'special-functions',
                'include': 'crates/mathhook-core/src/special_functions/**/*.rs',
                'ignore': '**/tests/**',
                'output': '02-core/special-functions/implementation.xml',
                'description': 'Special mathematical functions'
            },
            
            # ========== CORE - EDUCATIONAL ==========
            {
                'name': 'educational',
                'include': 'crates/mathhook-core/src/educational/**/*.rs',
                'ignore': '**/tests/**',
                'output': '02-core/educational/implementation.xml',
                'description': 'Step-by-step solutions and educational features'
            },
            
            # ========== BINDINGS ==========
            {
                'name': 'node-bindings',
                'include': 'crates/mathhook-node/**/*.rs,crates/mathhook-node/**/*.js,crates/mathhook-node/**/*.ts',
                'ignore': '**/node_modules/**,**/tests/**',
                'output': '03-bindings/node/implementation.xml',
                'description': 'Node.js bindings and API'
            },
            {
                'name': 'python-bindings',
                'include': 'crates/mathhook-python/**/*.rs,crates/mathhook-python/**/*.py',
                'ignore': '**/tests/**,**/__pycache__/**',
                'output': '03-bindings/python/implementation.xml',
                'description': 'Python bindings and API'
            },
            {
                'name': 'wasm-bindings',
                'include': 'crates/mathhook-wasm/**/*.rs,crates/mathhook-wasm/**/*.js',
                'ignore': '**/tests/**,**/pkg/**',
                'output': '03-bindings/wasm/implementation.xml',
                'description': 'WebAssembly bindings'
            },
            
            # ========== TESTS ==========
            {
                'name': 'integration-tests',
                'include': '**/tests/**/*.rs',
                'ignore': '**/*.json',
                'output': '04-tests/integration/all-tests.xml',
                'description': 'All integration tests'
            },
            
            # ========== DOCUMENTATION ==========
            {
                'name': 'documentation',
                'include': 'docs/**/*.md',
                'ignore': '',
                'output': '05-docs/documentation.xml',
                'description': 'Project documentation'
            },
            
            # ========== LARGE FILES (SEPARATE) ==========
            {
                'name': 'grammar-generated',
                'include': 'crates/mathhook-core/src/parser/grammar.rs',
                'ignore': '',
                'output': '99-large-files/grammar-generated.xml',
                'description': 'Generated grammar file (LARGE - analyze separately)'
            }
        ]
        
        print("\n" + "="*60)
        print("🚀 STARTING MATHHOOK CODEBASE ANALYSIS")
        print("="*60)
        
        for analysis in analyses:
            output_path = self.session_dir / analysis['output']
            self.run_repomix(
                analysis['name'],
                analysis['include'],
                analysis['ignore'],
                output_path,
                analysis['description']
            )
        
        self.create_summary()
        self.create_analysis_guide()
        
    def create_summary(self):
        """Create a summary JSON and markdown file"""
        # JSON summary
        summary_json = self.session_dir / "SUMMARY.json"
        with open(summary_json, 'w') as f:
            json.dump({
                'timestamp': self.timestamp,
                'total_analyses': len(self.summary),
                'analyses': self.summary
            }, f, indent=2)
        
        # Markdown summary
        summary_md = self.session_dir / "README.md"
        with open(summary_md, 'w') as f:
            f.write(f"# MathHook Codebase Analysis\n")
            f.write(f"Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
            f.write("## Analysis Summary\n\n")
            f.write("| Module | Tokens | File |\n")
            f.write("|--------|--------|------|\n")
            
            total_tokens = 0
            for item in self.summary:
                tokens_str = item['tokens'].replace(' tokens', '')
                try:
                    tokens_num = int(tokens_str.replace(',', ''))
                    total_tokens += tokens_num
                except:
                    pass
                f.write(f"| {item['description']} | {item['tokens']} | `{item['path']}` |\n")
            
            f.write(f"\n**Total Tokens: {total_tokens:,}**\n")
        
        print("\n" + "="*60)
        print("📊 ANALYSIS COMPLETE")
        print("="*60)
        print(f"📁 Output directory: {self.session_dir}")
        print(f"📄 Summary available at: {summary_md}")
        
    def create_analysis_guide(self):
        """Create a guide for using the analysis with Claude"""
        guide_path = self.session_dir / "CLAUDE_GUIDE.md"
        with open(guide_path, 'w') as f:
            f.write("""# How to Use This Analysis with Claude

## Recommended Order of Analysis

### 1. Start with Overview (Session 1)
```
First, let's understand the architecture:
[Paste contents of 01-overview/project-structure.xml]

Now the API surface:
[Paste contents of 01-overview/api-surface.xml]

Can you explain the overall architecture and main components?
```

### 2. Core Mathematical Components (Session 2-4)
Analyze each mathematical module in separate sessions:

**Session 2 - Algebra:**
```
Here's the algebra implementation of MathHook:
[Paste contents of 02-core/algebra/implementation.xml]
Can you explain the algebra capabilities and design patterns?
```

**Session 3 - Calculus:**
```
Here's the calculus implementation:
[Paste contents of 02-core/calculus/implementation.xml]
How does the calculus module work?
```

**Session 4 - Parser:**
```
Here's the parser implementation (without the generated grammar):
[Paste contents of 02-core/parser/implementation.xml]
And the grammar definition:
[Paste contents of 02-core/parser/grammar-definition.xml]
Can you explain how parsing works?
```

### 3. Bindings Analysis (Session 5)
```
Here are the Node.js bindings:
[Paste contents of 03-bindings/node/implementation.xml]

And Python bindings:
[Paste contents of 03-bindings/python/implementation.xml]

How do these FFI bindings work with the Rust core?
```

### 4. Deep Dives (As Needed)
- Educational features: `02-core/educational/implementation.xml`
- Special functions: `02-core/special-functions/implementation.xml`  
- Tests: `04-tests/integration/all-tests.xml`
- Documentation: `05-docs/documentation.xml`

## Tips for Claude Analysis

1. **Token Limits**: Each file should be under 150k tokens. Check SUMMARY.json for sizes.

2. **Context Building**: Reference previous sessions when asking follow-up questions:
   "Given the algebra module we discussed earlier, how does this calculus module interact with it?"

3. **Specific Questions**: Be specific about what you want:
   - "Find potential performance bottlenecks"
   - "Suggest improvements to the API design"
   - "Identify missing test cases"
   - "Review error handling patterns"

4. **Large Files**: The grammar.rs file is in `99-large-files/` - analyze it separately if needed.
""")
        
        print(f"📖 Claude guide created at: {guide_path}")

if __name__ == "__main__":
    analyzer = CodebaseAnalyzer()
    analyzer.analyze()
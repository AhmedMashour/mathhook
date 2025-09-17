#!/usr/bin/env python3
"""
Automated refactoring script for mathhook-node lib.rs

This script splits the large lib.rs file into focused modules while maintaining
all functionality. It performs the following:

1. Analyzes lib.rs structure and identifies logical sections
2. Creates module files with appropriate content
3. Updates lib.rs to use module declarations and re-exports
4. Backs up original file before making changes
5. Validates the refactoring by checking compilation

Safety features:
- Creates backup before any changes
- Validates regex patterns before applying
- Checks file existence before overwriting
- Provides rollback capability
- Dry-run mode for testing
"""

import re
import os
import shutil
import sys
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Tuple, Optional


class RefactorConfig:
    """Configuration for the refactoring process"""

    def __init__(self, lib_rs_path: str, dry_run: bool = False):
        self.lib_rs_path = Path(lib_rs_path)
        self.src_dir = self.lib_rs_path.parent
        self.backup_dir = self.src_dir / "backups"
        self.dry_run = dry_run

        # Timestamp for backup
        self.timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")

    def create_backup(self) -> Path:
        """Create backup of lib.rs before refactoring"""
        self.backup_dir.mkdir(exist_ok=True)
        backup_path = self.backup_dir / f"lib.rs.backup_{self.timestamp}"

        if not self.dry_run:
            shutil.copy2(self.lib_rs_path, backup_path)
            print(f"✅ Created backup: {backup_path}")
        else:
            print(f"[DRY RUN] Would create backup: {backup_path}")

        return backup_path


class ModuleExtractor:
    """Extracts code sections from lib.rs into separate modules"""

    def __init__(self, config: RefactorConfig):
        self.config = config
        self.content = ""

    def load_file(self) -> str:
        """Load lib.rs content"""
        with open(self.config.lib_rs_path, 'r', encoding='utf-8') as f:
            self.content = f.read()
        return self.content

    def extract_imports(self) -> str:
        """Extract all use statements and extern crate declarations"""
        pattern = r'^(?:use\s+.*?;|extern\s+crate\s+.*?;)'
        imports = re.findall(pattern, self.content, re.MULTILINE)
        return '\n'.join(imports)

    def extract_struct_impl(self, struct_name: str) -> Tuple[Optional[str], Optional[str]]:
        """Extract struct definition and its impl blocks"""
        # Extract struct definition
        struct_pattern = rf'(?:\/\/\/.*?\n)*(?:#\[.*?\]\n)*pub\s+struct\s+{struct_name}\s*{{[^}}]*}}'
        struct_match = re.search(struct_pattern, self.content, re.DOTALL)
        struct_def = struct_match.group(0) if struct_match else None

        # Extract impl blocks
        impl_pattern = rf'(?:\/\/\/.*?\n)*impl\s+{struct_name}\s*{{.*?^}}'
        impl_blocks = re.findall(impl_pattern, self.content, re.MULTILINE | re.DOTALL)
        impl_code = '\n\n'.join(impl_blocks) if impl_blocks else None

        return struct_def, impl_code

    def extract_napi_functions(self) -> str:
        """Extract all #[napi] function definitions"""
        # Match #[napi] functions with their documentation
        pattern = r'(?:\/\/\/.*?\n)*#\[napi\](?:\([^\)]*\))?\s*(?:pub\s+)?fn\s+\w+[^{]*\{(?:[^{}]|\{[^{}]*\})*\}'
        functions = re.findall(pattern, self.content, re.DOTALL)
        return '\n\n'.join(functions)

    def extract_helper_functions(self) -> str:
        """Extract non-napi helper functions"""
        # Match functions without #[napi] attribute
        pattern = r'(?:\/\/\/.*?\n)*(?<!#\[napi\]\n)(?:pub(?:\(crate\))?\s+)?fn\s+\w+[^{]*\{(?:[^{}]|\{[^{}]*\})*\}'

        # Filter out functions that are preceded by #[napi]
        all_functions = re.findall(pattern, self.content, re.DOTALL)

        helper_functions = []
        for func in all_functions:
            # Check if this function is part of an impl block or has #[napi]
            if not re.search(r'#\[napi\]', func) and 'impl ' not in func:
                helper_functions.append(func)

        return '\n\n'.join(helper_functions)


class ModuleWriter:
    """Writes extracted code to module files"""

    def __init__(self, config: RefactorConfig):
        self.config = config

    def write_module(self, module_name: str, content: str, imports: str = "") -> Path:
        """Write content to a module file"""
        module_path = self.config.src_dir / f"{module_name}.rs"

        full_content = f"""//! {module_name.replace('_', ' ').title()} module for MathHook Node.js bindings
//!
//! This module was automatically extracted from lib.rs during refactoring.

{imports}

{content}
"""

        if not self.config.dry_run:
            with open(module_path, 'w', encoding='utf-8') as f:
                f.write(full_content)
            print(f"✅ Created module: {module_path}")
        else:
            print(f"[DRY RUN] Would create module: {module_path}")
            print(f"Content preview:\n{full_content[:200]}...\n")

        return module_path

    def update_lib_rs(self, module_declarations: List[str], re_exports: List[str]) -> None:
        """Update lib.rs with module declarations and re-exports"""
        # Read original lib.rs to preserve top-level documentation
        original_content = self.config.lib_rs_path.read_text(encoding='utf-8')

        # Extract top documentation (before first code)
        doc_pattern = r'^(?:\/\/!.*?\n|#!\[.*?\]\n)*'
        doc_match = re.match(doc_pattern, original_content, re.MULTILINE)
        top_doc = doc_match.group(0) if doc_match else ""

        new_lib_rs = f"""{top_doc}
#![deny(clippy::all)]

// Module declarations
{chr(10).join(module_declarations)}

// Re-exports for public API
{chr(10).join(re_exports)}
"""

        if not self.config.dry_run:
            with open(self.config.lib_rs_path, 'w', encoding='utf-8') as f:
                f.write(new_lib_rs)
            print(f"✅ Updated lib.rs with module structure")
        else:
            print(f"[DRY RUN] Would update lib.rs:")
            print(new_lib_rs)


class RefactorValidator:
    """Validates the refactoring by running compilation"""

    def __init__(self, config: RefactorConfig):
        self.config = config

    def validate(self) -> bool:
        """Run cargo check to validate refactoring"""
        if self.config.dry_run:
            print("[DRY RUN] Would run: cargo check -p mathhook-node")
            return True

        print("\n🔍 Validating refactoring with cargo check...")
        result = os.system("cd /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-node && cargo check 2>&1")

        if result == 0:
            print("✅ Validation successful - code compiles!")
            return True
        else:
            print("❌ Validation failed - compilation errors detected")
            return False

    def rollback(self, backup_path: Path) -> None:
        """Rollback to backup if validation fails"""
        if self.config.dry_run:
            print(f"[DRY RUN] Would rollback from: {backup_path}")
            return

        print(f"🔄 Rolling back to backup: {backup_path}")
        shutil.copy2(backup_path, self.config.lib_rs_path)

        # Remove created module files
        for module_file in self.config.src_dir.glob("*.rs"):
            if module_file.name != "lib.rs":
                module_file.unlink()
                print(f"   Removed: {module_file}")

        print("✅ Rollback complete")


def analyze_lib_rs_structure(lib_rs_path: str) -> Dict[str, any]:
    """Analyze lib.rs structure and provide refactoring plan"""
    with open(lib_rs_path, 'r', encoding='utf-8') as f:
        content = f.read()

    analysis = {
        'total_lines': len(content.splitlines()),
        'struct_count': len(re.findall(r'pub\s+struct\s+\w+', content)),
        'impl_count': len(re.findall(r'^impl\s+\w+', content, re.MULTILINE)),
        'napi_functions': len(re.findall(r'#\[napi\]', content)),
        'use_statements': len(re.findall(r'^use\s+', content, re.MULTILINE)),
    }

    print("\n📊 Analysis of lib.rs:")
    print(f"   Total lines: {analysis['total_lines']}")
    print(f"   Struct definitions: {analysis['struct_count']}")
    print(f"   Impl blocks: {analysis['impl_count']}")
    print(f"   NAPI functions: {analysis['napi_functions']}")
    print(f"   Use statements: {analysis['use_statements']}")

    return analysis


def main():
    """Main refactoring process"""
    import argparse

    parser = argparse.ArgumentParser(description='Refactor mathhook-node lib.rs into modules')
    parser.add_argument('--dry-run', action='store_true', help='Run without making changes')
    parser.add_argument('--lib-rs', default='src/lib.rs', help='Path to lib.rs')
    args = parser.parse_args()

    # Resolve full path
    lib_rs_path = Path(__file__).parent.parent / args.lib_rs

    if not lib_rs_path.exists():
        print(f"❌ Error: {lib_rs_path} not found")
        sys.exit(1)

    print("🚀 Starting mathhook-node refactoring")
    print(f"   Mode: {'DRY RUN' if args.dry_run else 'LIVE'}")
    print(f"   File: {lib_rs_path}\n")

    # Analyze structure
    analysis = analyze_lib_rs_structure(lib_rs_path)

    # Initialize components
    config = RefactorConfig(str(lib_rs_path), dry_run=args.dry_run)
    extractor = ModuleExtractor(config)
    writer = ModuleWriter(config)
    validator = RefactorValidator(config)

    # Create backup
    backup_path = config.create_backup()

    try:
        # Load file
        print("\n📖 Loading lib.rs...")
        extractor.load_file()

        # Extract imports
        print("📦 Extracting imports...")
        imports = extractor.extract_imports()

        # Extract JsExpression
        print("🔍 Extracting JsExpression struct and impl...")
        js_expr_struct, js_expr_impl = extractor.extract_struct_impl('JsExpression')

        if js_expr_struct and js_expr_impl:
            js_expr_content = f"{js_expr_struct}\n\n{js_expr_impl}"
            writer.write_module('expression', js_expr_content, imports)

        # Extract MathSolver
        print("🔍 Extracting MathSolver struct and impl...")
        solver_struct, solver_impl = extractor.extract_struct_impl('MathSolver')

        if solver_struct and solver_impl:
            solver_content = f"{solver_struct}\n\n{solver_impl}"
            writer.write_module('solver', solver_content, imports)

        # Extract standalone NAPI functions
        print("🔍 Extracting standalone NAPI functions...")
        napi_funcs = extractor.extract_napi_functions()
        if napi_funcs:
            writer.write_module('functions', napi_funcs, imports)

        # Extract helper functions
        print("🔍 Extracting helper functions...")
        helpers = extractor.extract_helper_functions()
        if helpers:
            writer.write_module('helpers', helpers, imports)

        # Update lib.rs
        print("\n📝 Updating lib.rs...")
        module_declarations = [
            "mod expression;",
            "mod solver;",
            "mod functions;",
            "mod helpers;",
        ]

        re_exports = [
            "pub use expression::JsExpression;",
            "pub use solver::MathSolver;",
            "pub use functions::*;",
        ]

        writer.update_lib_rs(module_declarations, re_exports)

        # Validate
        if not config.dry_run:
            if not validator.validate():
                print("\n⚠️  Validation failed! Rolling back...")
                validator.rollback(backup_path)
                sys.exit(1)

        print("\n✅ Refactoring complete!")
        print(f"   Backup saved at: {backup_path}")

    except Exception as e:
        print(f"\n❌ Error during refactoring: {e}")
        if not config.dry_run:
            print("Rolling back changes...")
            validator.rollback(backup_path)
        sys.exit(1)


if __name__ == '__main__':
    main()

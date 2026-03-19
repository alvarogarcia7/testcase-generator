#!/usr/bin/env python3
"""
Verify Python 3.14 compatibility for all Python scripts.

This script performs static analysis to check for Python 3.14 compatibility issues.
"""

import ast
import sys
from pathlib import Path
from typing import Any


class Python314CompatibilityChecker(ast.NodeVisitor):
    """AST visitor to check for Python 3.14 compatibility issues."""
    
    def __init__(self, filename: str):
        self.filename = filename
        self.issues: list[str] = []
        self.warnings: list[str] = []
    
    def visit_ImportFrom(self, node: ast.ImportFrom) -> None:
        """Check for deprecated typing imports."""
        if node.module == 'typing':
            deprecated_imports = {'Dict', 'List', 'Tuple', 'Set', 'FrozenSet', 
                                  'Optional', 'Union'}
            for alias in node.names:
                if alias.name in deprecated_imports:
                    self.warnings.append(
                        f"Line {node.lineno}: Consider using built-in types or '|' "
                        f"operator instead of typing.{alias.name}"
                    )
        
        if node.module == 'collections':
            # Check for direct collections imports (should use collections.abc)
            abc_types = {'Mapping', 'MutableMapping', 'Sequence', 'MutableSequence',
                        'Set', 'MutableSet', 'Iterable', 'Iterator'}
            for alias in node.names:
                if alias.name in abc_types:
                    self.issues.append(
                        f"Line {node.lineno}: Import {alias.name} from collections.abc, "
                        f"not collections (deprecated)"
                    )
        
        self.generic_visit(node)
    
    def visit_Call(self, node: ast.Call) -> None:
        """Check for open() calls without explicit encoding."""
        if isinstance(node.func, ast.Name) and node.func.id == 'open':
            # Check if encoding is specified
            has_encoding = False
            for keyword in node.keywords:
                if keyword.arg == 'encoding':
                    has_encoding = True
                    break
            
            # Check if mode is binary
            is_binary = False
            if node.args and len(node.args) >= 2:
                if isinstance(node.args[1], ast.Constant):
                    mode = node.args[1].value
                    if isinstance(mode, str) and 'b' in mode:
                        is_binary = True
            
            for keyword in node.keywords:
                if keyword.arg == 'mode' and isinstance(keyword.value, ast.Constant):
                    mode = keyword.value.value
                    if isinstance(mode, str) and 'b' in mode:
                        is_binary = True
            
            if not has_encoding and not is_binary:
                self.warnings.append(
                    f"Line {node.lineno}: open() without explicit encoding parameter. "
                    f"Consider adding encoding='utf-8'"
                )
        
        self.generic_visit(node)


def check_file(filepath: Path) -> tuple[list[str], list[str]]:
    """Check a Python file for compatibility issues."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        tree = ast.parse(content, filename=str(filepath))
        checker = Python314CompatibilityChecker(str(filepath))
        checker.visit(tree)
        
        return checker.issues, checker.warnings
    except SyntaxError as e:
        return [f"Syntax error: {e}"], []
    except Exception as e:
        return [f"Error parsing file: {e}"], []


def main() -> int:
    """Main entry point."""
    print("🔍 Python 3.14 Compatibility Checker\n")
    
    # Get project root
    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    
    # Find all Python files
    python_files = []
    python_files.extend(project_root.glob('scripts/**/*.py'))
    python_files.extend(project_root.glob('*.py'))
    
    # Exclude this script
    python_files = [f for f in python_files if f.name != 'verify_python_314_compatibility.py']
    
    total_issues = 0
    total_warnings = 0
    files_with_issues = 0
    files_with_warnings = 0
    
    print(f"Checking {len(python_files)} Python file(s)...\n")
    
    for filepath in sorted(python_files):
        relative_path = filepath.relative_to(project_root)
        issues, warnings = check_file(filepath)
        
        if issues:
            files_with_issues += 1
            total_issues += len(issues)
            print(f"❌ {relative_path}")
            for issue in issues:
                print(f"   {issue}")
            print()
        elif warnings:
            files_with_warnings += 1
            total_warnings += len(warnings)
            print(f"⚠️  {relative_path}")
            for warning in warnings:
                print(f"   {warning}")
            print()
    
    # Summary
    print("=" * 70)
    print("\n📊 Summary:\n")
    print(f"Files checked: {len(python_files)}")
    print(f"Files with issues: {files_with_issues}")
    print(f"Files with warnings: {files_with_warnings}")
    print(f"Total issues: {total_issues}")
    print(f"Total warnings: {total_warnings}")
    print()
    
    if total_issues > 0:
        print("❌ Compatibility issues found that must be fixed!")
        return 1
    elif total_warnings > 0:
        print("⚠️  Some warnings found. Consider addressing them for best practices.")
        print("✅ No critical compatibility issues detected.")
        return 0
    else:
        print("✅ All files are Python 3.14 compatible!")
        return 0


if __name__ == '__main__':
    sys.exit(main())

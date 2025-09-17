//! Rust-based refactoring tool for mathhook-node
//!
//! This tool uses the `syn` crate to parse lib.rs with Rust's actual AST parser,
//! guaranteeing correctness in code extraction. It splits lib.rs into focused modules.

use anyhow::{Context, Result};
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{File, Item, ItemImpl, ItemStruct, ItemFn, Attribute};
use quote::ToTokens;

#[derive(Parser, Debug)]
#[command(name = "refactor")]
#[command(about = "Refactor mathhook-node lib.rs into modules", long_about = None)]
struct Args {
    /// Path to lib.rs
    #[arg(short, long, default_value = "../src/lib.rs")]
    lib_rs: PathBuf,

    /// Dry run - don't write files
    #[arg(short, long)]
    dry_run: bool,

    /// Output directory for modules
    #[arg(short, long, default_value = "../src")]
    output: PathBuf,

    /// Create backup before refactoring
    #[arg(short, long, default_value = "true")]
    backup: bool,
}

#[derive(Debug)]
struct Module {
    name: String,
    structs: Vec<ItemStruct>,
    impls: Vec<ItemImpl>,
    functions: Vec<ItemFn>,
    use_statements: Vec<syn::ItemUse>,
    attributes: Vec<Attribute>,
}

impl Module {
    fn new(name: String) -> Self {
        Module {
            name,
            structs: Vec::new(),
            impls: Vec::new(),
            functions: Vec::new(),
            use_statements: Vec::new(),
            attributes: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.structs.is_empty() && self.impls.is_empty() && self.functions.is_empty()
    }

    fn to_module_content(&self) -> String {
        let mut content = String::new();

        // Module documentation
        content.push_str(&format!(
            "//! {} module for MathHook Node.js bindings\n",
            self.name.replace('_', " ").chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_uppercase().to_string() } else { c.to_string() })
                .collect::<String>()
        ));
        content.push_str("//!\n");
        content.push_str("//! This module was automatically extracted from lib.rs using syn-based refactoring.\n\n");

        // Use statements
        if !self.use_statements.is_empty() {
            for use_stmt in &self.use_statements {
                content.push_str(&use_stmt.to_token_stream().to_string());
                content.push('\n');
            }
            content.push('\n');
        }

        // Structs with their attributes
        for struct_item in &self.structs {
            content.push_str(&struct_item.to_token_stream().to_string());
            content.push_str("\n\n");
        }

        // Impl blocks
        for impl_block in &self.impls {
            content.push_str(&impl_block.to_token_stream().to_string());
            content.push_str("\n\n");
        }

        // Standalone functions
        for func in &self.functions {
            content.push_str(&func.to_token_stream().to_string());
            content.push_str("\n\n");
        }

        content
    }
}

struct Refactorer {
    args: Args,
    modules: HashMap<String, Module>,
    lib_rs_content: String,
}

impl Refactorer {
    fn new(args: Args) -> Result<Self> {
        let lib_rs_content = fs::read_to_string(&args.lib_rs)
            .with_context(|| format!("Failed to read {:?}", args.lib_rs))?;

        Ok(Refactorer {
            args,
            modules: HashMap::new(),
            lib_rs_content,
        })
    }

    fn analyze(&mut self) -> Result<()> {
        println!("🔍 Parsing lib.rs with syn...");

        let ast: File = syn::parse_file(&self.lib_rs_content)
            .context("Failed to parse lib.rs - syntax error?")?;

        println!("✅ Successfully parsed lib.rs");
        println!("📊 Found {} top-level items", ast.items.len());

        // Collect use statements for all modules
        let mut global_uses = Vec::new();

        for item in &ast.items {
            match item {
                Item::Use(use_item) => {
                    global_uses.push(use_item.clone());
                }
                _ => {}
            }
        }

        // Analyze items and categorize them
        for item in ast.items {
            match item {
                Item::Struct(struct_item) => {
                    self.categorize_struct(struct_item, &global_uses)?;
                }
                Item::Impl(impl_block) => {
                    self.categorize_impl(impl_block, &global_uses)?;
                }
                Item::Fn(func) => {
                    self.categorize_function(func, &global_uses)?;
                }
                Item::Use(_) => {
                    // Already collected
                }
                _ => {
                    println!("⚠️  Skipping unsupported item type");
                }
            }
        }

        println!("\n📦 Module breakdown:");
        for (name, module) in &self.modules {
            if !module.is_empty() {
                println!("   {} - {} structs, {} impls, {} functions",
                    name,
                    module.structs.len(),
                    module.impls.len(),
                    module.functions.len()
                );
            }
        }

        Ok(())
    }

    fn categorize_struct(&mut self, struct_item: ItemStruct, uses: &[syn::ItemUse]) -> Result<()> {
        let struct_name = struct_item.ident.to_string();

        let module_name = match struct_name.as_str() {
            "JsExpression" => "expression",
            "MathSolver" => "solver",
            _ => "types",
        };

        let module = self.modules.entry(module_name.to_string())
            .or_insert_with(|| Module::new(module_name.to_string()));

        module.structs.push(struct_item);

        // Add use statements if not already present
        for use_stmt in uses {
            if !module.use_statements.iter().any(|u| u.to_token_stream().to_string() == use_stmt.to_token_stream().to_string()) {
                module.use_statements.push(use_stmt.clone());
            }
        }

        Ok(())
    }

    fn categorize_impl(&mut self, impl_block: ItemImpl, uses: &[syn::ItemUse]) -> Result<()> {
        // Extract the type being implemented
        let type_name = match &*impl_block.self_ty {
            syn::Type::Path(type_path) => {
                type_path.path.segments.last()
                    .map(|seg| seg.ident.to_string())
                    .unwrap_or_else(|| "unknown".to_string())
            }
            _ => "unknown".to_string(),
        };

        let module_name = match type_name.as_str() {
            "JsExpression" => "expression",
            "MathSolver" => "solver",
            _ => "types",
        };

        let module = self.modules.entry(module_name.to_string())
            .or_insert_with(|| Module::new(module_name.to_string()));

        module.impls.push(impl_block);

        // Add use statements if not already present
        for use_stmt in uses {
            if !module.use_statements.iter().any(|u| u.to_token_stream().to_string() == use_stmt.to_token_stream().to_string()) {
                module.use_statements.push(use_stmt.clone());
            }
        }

        Ok(())
    }

    fn categorize_function(&mut self, func: ItemFn, uses: &[syn::ItemUse]) -> Result<()> {
        // Check if function has #[napi] attribute
        let has_napi = func.attrs.iter().any(|attr| {
            attr.path().is_ident("napi")
        });

        let module_name = if has_napi {
            "functions"
        } else {
            "helpers"
        };

        let module = self.modules.entry(module_name.to_string())
            .or_insert_with(|| Module::new(module_name.to_string()));

        module.functions.push(func);

        // Add use statements if not already present
        for use_stmt in uses {
            if !module.use_statements.iter().any(|u| u.to_token_stream().to_string() == use_stmt.to_token_stream().to_string()) {
                module.use_statements.push(use_stmt.clone());
            }
        }

        Ok(())
    }

    fn create_backup(&self) -> Result<()> {
        if !self.args.backup {
            return Ok(());
        }

        let backup_dir = self.args.output.join("backups");
        fs::create_dir_all(&backup_dir)?;

        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_path = backup_dir.join(format!("lib.rs.backup_{}", timestamp));

        fs::copy(&self.args.lib_rs, &backup_path)
            .with_context(|| format!("Failed to create backup at {:?}", backup_path))?;

        println!("✅ Created backup: {:?}", backup_path);
        Ok(())
    }

    fn write_modules(&self) -> Result<()> {
        for (name, module) in &self.modules {
            if module.is_empty() {
                continue;
            }

            let module_path = self.args.output.join(format!("{}.rs", name));

            if self.args.dry_run {
                println!("[DRY RUN] Would write module: {:?}", module_path);
                println!("Preview (first 500 chars):\n{}\n",
                    module.to_module_content().chars().take(500).collect::<String>());
            } else {
                fs::write(&module_path, module.to_module_content())
                    .with_context(|| format!("Failed to write module {:?}", module_path))?;
                println!("✅ Created module: {:?}", module_path);
            }
        }

        Ok(())
    }

    fn generate_new_lib_rs(&self) -> Result<String> {
        let mut content = String::new();

        // Preserve top-level documentation
        content.push_str("//! MathHook Node.js bindings\n");
        content.push_str("//!\n");
        content.push_str("//! High-performance symbolic mathematics for Node.js\n\n");
        content.push_str("#![deny(clippy::all)]\n\n");

        // Module declarations
        content.push_str("// Module declarations\n");
        for name in self.modules.keys() {
            if !self.modules[name].is_empty() {
                content.push_str(&format!("mod {};\n", name));
            }
        }
        content.push('\n');

        // Re-exports
        content.push_str("// Public API re-exports\n");

        if self.modules.contains_key("expression") && !self.modules["expression"].is_empty() {
            content.push_str("pub use expression::JsExpression;\n");
        }

        if self.modules.contains_key("solver") && !self.modules["solver"].is_empty() {
            content.push_str("pub use solver::MathSolver;\n");
        }

        if self.modules.contains_key("functions") && !self.modules["functions"].is_empty() {
            content.push_str("pub use functions::*;\n");
        }

        Ok(content)
    }

    fn update_lib_rs(&self) -> Result<()> {
        let new_content = self.generate_new_lib_rs()?;

        if self.args.dry_run {
            println!("[DRY RUN] Would update lib.rs with:");
            println!("{}", new_content);
        } else {
            fs::write(&self.args.lib_rs, new_content)
                .with_context(|| format!("Failed to write new lib.rs at {:?}", self.args.lib_rs))?;
            println!("✅ Updated lib.rs with module structure");
        }

        Ok(())
    }

    fn validate(&self) -> Result<()> {
        if self.args.dry_run {
            println!("[DRY RUN] Would run: cargo check");
            return Ok(());
        }

        println!("\n🔍 Validating refactoring with cargo check...");

        let output = std::process::Command::new("cargo")
            .arg("check")
            .current_dir(self.args.output.parent().unwrap())
            .output()
            .context("Failed to run cargo check")?;

        if output.status.success() {
            println!("✅ Validation successful - code compiles!");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("❌ Validation failed:");
            println!("{}", stderr);
            anyhow::bail!("Compilation failed after refactoring")
        }
    }

    fn run(&mut self) -> Result<()> {
        println!("🚀 Starting mathhook-node refactoring");
        println!("   Mode: {}", if self.args.dry_run { "DRY RUN" } else { "LIVE" });
        println!("   File: {:?}\n", self.args.lib_rs);

        // Create backup
        self.create_backup()?;

        // Parse and analyze
        self.analyze()?;

        // Write modules
        self.write_modules()?;

        // Update lib.rs
        self.update_lib_rs()?;

        // Validate
        self.validate()?;

        println!("\n✅ Refactoring complete!");

        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut refactorer = Refactorer::new(args)?;
    refactorer.run()
}

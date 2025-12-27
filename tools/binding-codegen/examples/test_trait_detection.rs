use binding_codegen::scanner;
use binding_codegen::trait_analyzer::TraitAnalysis;
use std::collections::HashMap;

fn main() {
    let scanned = scanner::scan().expect("Failed to scan");

    let expression_impls: Vec<_> = scanned
        .impls
        .iter()
        .filter(|impl_info| impl_info.target_type == "Expression")
        .collect();

    println!("Expression impl blocks found: {}", expression_impls.len());

    let trait_impls: Vec<_> = expression_impls
        .iter()
        .filter(|impl_info| impl_info.trait_name.is_some())
        .collect();

    println!("Trait impls for Expression: {}", trait_impls.len());

    println!("\nChecking trait paths:");
    for impl_info in &trait_impls {
        if let Some(trait_name) = &impl_info.trait_name {
            let is_mathhook = impl_info.is_mathhook_core_trait();
            let is_std = impl_info.is_std_bindable_trait();
            println!(
                "  - {} (mathhook_core: {}, std_bindable: {})",
                trait_name, is_mathhook, is_std
            );
        }
    }

    let expression_type = scanned.types.iter().find(|t| t.name == "Expression");

    let derived_traits = expression_type
        .map(|t| t.derived_traits.as_slice())
        .unwrap_or(&[]);

    if !derived_traits.is_empty() {
        println!("\nDerived traits from #[derive(...)]:");
        for trait_name in derived_traits {
            println!("  - {}", trait_name);
        }
    }

    let trait_analysis =
        TraitAnalysis::from_impls_and_derives("Expression", &scanned.impls, derived_traits);
    println!("\nSupported traits detected (explicit + derived):");
    for trait_type in &trait_analysis.implemented_traits {
        println!("  - {:?}", trait_type);
    }

    println!("\nDomain trait methods (non-std traits from mathhook-core):");
    for method in &trait_analysis.domain_trait_methods {
        println!(
            "  - {}::{} (static: {})",
            method.trait_name,
            method.method_name(),
            method.is_static
        );
    }

    println!("\nTrait imports needed:");
    let trait_path_map: HashMap<String, String> = HashMap::new();
    for import in trait_analysis.unique_trait_imports(&trait_path_map) {
        println!("  - use {};", import);
    }
}

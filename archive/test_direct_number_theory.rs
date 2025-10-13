fn main() {
    println!("🔍 TESTING DIRECT NUMBER THEORY MODULE");

    // Try to create number theory intelligence directly
    let nt = mathhook_core::functions::number_theory::NumberTheoryIntelligence::new();
    println!("Number theory created successfully!");

    let properties = nt.get_all_properties();
    println!("Number theory properties count: {}", properties.len());

    for (name, _) in &properties {
        println!("  - {}", name);
    }
}

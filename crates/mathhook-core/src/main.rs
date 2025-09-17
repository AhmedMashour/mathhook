use mathhook_core::prelude::*;
fn main() {
    println!("\n=== Simple parsing testing ===\n");
    let parser = Parser::new(&ParserConfig::default());
    let expression = parser.parse("sin(π)");
    match expression {
        Ok(e) => {
            let evaluated = e.evaluate().unwrap();
            let value = evaluated.format().unwrap_or(String::from(""));
            println!("The expressions is {}", value);
        }
        Err(e) => {
            println!("Nothing {}", e);
        }
    }
}

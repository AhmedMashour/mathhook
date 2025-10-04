use mathhook_core::parser::lalrpop::grammar::mathematical;

fn main() {
    let expr: String = "2x+1=3".to_string();
    let expr = mathematical::ExpressionParser::new().parse(&expr);
    match expr {
        Ok(expr) => println!("{:?}", expr),
        Err(e) => println!("Error: {:?}", e),
    }
}

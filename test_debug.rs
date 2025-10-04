use mathhook_core::parser::nom::latex::atoms::parse_command_atom;

fn main() {
    let input = "\\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}";
    println!("Testing parse_command_atom with: {}", input);
    
    match parse_command_atom(input) {
        Ok((remaining, atom)) => {
            println!("SUCCESS!");
            println!("Remaining: '{}'", remaining);
            println!("Atom: {:?}", atom);
        }
        Err(e) => {
            println!("ERROR: {:?}", e);
        }
    }
}

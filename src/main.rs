use mathhook::prelude::*;

fn main() {
    let result = convenience::solve("2x + 6 = 0").unwrap_or(SolverResult::NoSolution);

    println!("{:?}", result);
}

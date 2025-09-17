#[test]
fn test_solver_memory_efficiency() {
    // Expression size must remain 32 bytes
    assert!(
        std::mem::size_of::<Expression>() <= 32,
        "Expression size must remain ≤32 bytes, got {}",
        std::mem::size_of::<Expression>()
    );

    // Solver result types must be memory efficient
    assert!(
        std::mem::size_of::<SolverResult>() <= 64,
        "SolverResult must be ≤64 bytes, got {}",
        std::mem::size_of::<SolverResult>()
    );

    // Solver structs must be lightweight
    assert!(
        std::mem::size_of::<LinearSolver>() <= 128,
        "LinearSolver must be ≤128 bytes, got {}",
        std::mem::size_of::<LinearSolver>()
    );
}

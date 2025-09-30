/// Comprehensive test suite for LALRPOP mathematical parser
///
/// Tests grammar correctness, roundtrip consistency, and performance
/// against the 58 mathematical expression test cases.

#[cfg(test)]
mod grammar_tests;

#[cfg(test)]
mod roundtrip_tests;

#[cfg(test)]
mod performance_tests;

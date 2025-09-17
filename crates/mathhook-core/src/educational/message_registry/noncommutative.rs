//! Noncommutative algebra educational messages
//!
//! Provides messages explaining left/right division, order dependence,
//! and commutativity concepts for matrix, operator, and quaternion algebra.

use super::core::{MessageCategory, MessageKey, MessageTemplate, MessageType};
use std::collections::HashMap;

/// Initialize all noncommutative algebra messages
pub fn initialize_noncommutative_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    initialize_left_division_messages(registry);
    initialize_right_division_messages(registry);
    initialize_commutativity_messages(registry);
    initialize_order_messages(registry);
}

/// Initialize left division messages
fn initialize_left_division_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::LeftMultiplyInverse,
            0,
        ),
        MessageTemplate::new(
            "Left Multiplication by Inverse",
            "Multiply both sides on the LEFT by {inverse}\nFor equation {equation}, we get: {inverse}*({lhs}) = {inverse}*({rhs})",
            &["inverse", "equation", "lhs", "rhs"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::LeftMultiplyInverse,
            1,
        ),
        MessageTemplate::new(
            "Left Division Explanation",
            "For equation {A}*{X} = {B}, multiply LEFT by {A_inv}\nReason: {X} is on the RIGHT of {A}, so we multiply on the LEFT to isolate {X}",
            &["A", "X", "B", "A_inv"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::LeftMultiplyInverse,
            2,
        ),
        MessageTemplate::new(
            "Left Division Step",
            "Step: {A_inv}*({A}*{X}) = {A_inv}*{B}\nUse associativity: ({A_inv}*{A})*{X} = {A_inv}*{B}\nSince {A_inv}*{A} = I (identity): I*{X} = {A_inv}*{B}\nSolution: {X} = {A_inv}*{B}",
            &["A", "X", "B", "A_inv"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::LeftMultiplyInverse,
            3,
        ),
        MessageTemplate::new(
            "Left Division Educational Note",
            "Why multiply on the LEFT?\nIn {A}*{X} = {B}, the variable {X} appears on the RIGHT of {A}.\nFor noncommutative objects, {A_inv}*{B} is NOT equal to {B}*{A_inv}.\nTherefore, we must multiply on the LEFT to preserve equation validity.",
            &["A", "X", "B", "A_inv"],
        ),
    );
}

/// Initialize right division messages
fn initialize_right_division_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::RightMultiplyInverse,
            0,
        ),
        MessageTemplate::new(
            "Right Multiplication by Inverse",
            "Multiply both sides on the RIGHT by {inverse}\nFor equation {equation}, we get: ({lhs})*{inverse} = ({rhs})*{inverse}",
            &["inverse", "equation", "lhs", "rhs"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::RightMultiplyInverse,
            1,
        ),
        MessageTemplate::new(
            "Right Division Explanation",
            "For equation {X}*{A} = {B}, multiply RIGHT by {A_inv}\nReason: {X} is on the LEFT of {A}, so we multiply on the RIGHT to isolate {X}",
            &["X", "A", "B", "A_inv"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::RightMultiplyInverse,
            2,
        ),
        MessageTemplate::new(
            "Right Division Step",
            "Step: ({X}*{A})*{A_inv} = {B}*{A_inv}\nUse associativity: {X}*({A}*{A_inv}) = {B}*{A_inv}\nSince {A}*{A_inv} = I (identity): {X}*I = {B}*{A_inv}\nSolution: {X} = {B}*{A_inv}",
            &["X", "A", "B", "A_inv"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::RightMultiplyInverse,
            3,
        ),
        MessageTemplate::new(
            "Right Division Educational Note",
            "Why multiply on the RIGHT?\nIn {X}*{A} = {B}, the variable {X} appears on the LEFT of {A}.\nFor noncommutative objects, {B}*{A_inv} is NOT equal to {A_inv}*{B}.\nTherefore, we must multiply on the RIGHT to preserve equation validity.",
            &["X", "A", "B", "A_inv"],
        ),
    );
}

/// Initialize commutativity explanation messages
fn initialize_commutativity_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::NoncommutativeWarning,
            0,
        ),
        MessageTemplate::new(
            "Noncommutative Object Warning",
            "WARNING: {symbol} is noncommutative (type: {symbol_type})\nThis means the order of multiplication matters: {symbol}*{other} may NOT equal {other}*{symbol}",
            &["symbol", "symbol_type", "other"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::NoncommutativeWarning,
            1,
        ),
        MessageTemplate::new(
            "Matrix Noncommutativity",
            "Matrices are noncommutative: A*B is generally NOT equal to B*A\nExample: For 2x2 matrices, A*B and B*A often give different results.\nAlways preserve multiplication order in matrix equations.",
            &[],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::NoncommutativeWarning,
            2,
        ),
        MessageTemplate::new(
            "Operator Noncommutativity",
            "Quantum operators are noncommutative: operators do not commute in general\nExample: Position and momentum operators satisfy [x,p] = xp - px = i*hbar (Heisenberg uncertainty)\nOrder matters critically in quantum mechanics.",
            &[],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::NoncommutativeWarning,
            3,
        ),
        MessageTemplate::new(
            "Quaternion Noncommutativity",
            "Quaternions are noncommutative: ij is NOT equal to ji\nExample: i*j = k, but j*i = -k (opposite sign)\nQuaternion multiplication follows strict order rules.",
            &[],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::CommutatorExplanation,
            0,
        ),
        MessageTemplate::new(
            "Commutator Definition",
            "The commutator [{A},{B}] = {A}*{B} - {B}*{A} measures how much {A} and {B} fail to commute.\nIf [{A},{B}] = 0, then {A} and {B} commute (order doesn't matter).\nIf [{A},{B}] is not 0, then order matters.",
            &["A", "B"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::CommutatorExplanation,
            1,
        ),
        MessageTemplate::new(
            "Commutator Significance",
            "Commutators reveal fundamental properties:\nIn quantum mechanics, [{position},{momentum}] = i*hbar (uncertainty principle)\nIn matrix algebra, commutators determine if matrices can be simultaneously diagonalized\nCommutators are central to Lie algebra theory.",
            &[],
        ),
    );
}

/// Initialize order-matters messages
fn initialize_order_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::OrderMatters,
            0,
        ),
        MessageTemplate::new(
            "Order Matters",
            "Order matters because {symbol} is {symbol_type}\nIn noncommutative algebra: {A}*{B} is generally NOT equal to {B}*{A}\nAlways preserve the exact order of multiplication.",
            &["symbol", "symbol_type", "A", "B"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::OrderMatters,
            1,
        ),
        MessageTemplate::new(
            "Left vs Right Division Choice",
            "Choosing left or right division:\n- If variable appears as {A}*{X}, use LEFT division by {A_inv}\n- If variable appears as {X}*{A}, use RIGHT division by {A_inv}\nThe position of the variable determines the multiplication side.",
            &["A", "X", "A_inv"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::OrderMatters,
            2,
        ),
        MessageTemplate::new(
            "Associativity Still Valid",
            "Important: While order matters, associativity still holds:\n({A}*{B})*{C} = {A}*({B}*{C})\nThis allows us to use parentheses to regroup (but not reorder) multiplications.\nAssociativity is the key to solving matrix equations.",
            &["A", "B", "C"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::OrderMatters,
            3,
        ),
        MessageTemplate::new(
            "Common Errors to Avoid",
            "Common mistakes in noncommutative algebra:\n1. Swapping order: {A}*{B} to {B}*{A} (WRONG)\n2. Distributing incorrectly: ({A}+{B})*{C} is NOT {A}*{C}+{C}*{B}\n3. Canceling carelessly: {A}*{X}*{B} = {A}*{Y}*{B} does NOT imply {X} = {Y}\nAlways respect order constraints.",
            &["A", "B", "C", "X", "Y"],
        ),
    );
}

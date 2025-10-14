//! Solver educational messages for equation systems

use super::core::{MessageCategory, MessageKey, MessageTemplate, MessageType};
use std::collections::HashMap;

/// Initialize all solver-related messages
pub fn initialize_solver_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    initialize_substitution_messages(registry);
    initialize_elimination_messages(registry);
    initialize_matrix_method_messages(registry);
    initialize_solution_interpretation_messages(registry);
}

/// Initialize substitution method messages
fn initialize_substitution_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemSubstitution, 0),
        MessageTemplate::new(
            "Substitution Method Introduction",
            "Solve system using substitution method\nStep 1: Solve one equation for one variable\nStep 2: Substitute into other equation",
            &[]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemSubstitution, 1),
        MessageTemplate::new(
            "Substitution Step 1: Isolate Variable",
            "From equation {equation_number}: {equation}\nSolve for {variable}: {variable} = {expression}",
            &["equation_number", "equation", "variable", "expression"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemSubstitution, 2),
        MessageTemplate::new(
            "Substitution Step 2: Substitute",
            "Substitute {variable} = {expression} into equation {equation_number}:\n{original_equation} becomes {substituted_equation}",
            &["variable", "expression", "equation_number", "original_equation", "substituted_equation"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemSubstitution, 3),
        MessageTemplate::new(
            "Substitution Step 3: Solve Single Variable",
            "Now solve for {variable} in the single-variable equation:\n{equation}\nSolution: {variable} = {solution}",
            &["variable", "equation", "solution"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemSubstitution, 4),
        MessageTemplate::new(
            "Substitution Step 4: Back-Substitute",
            "Substitute {variable} = {value} back into {variable_expr} = {expression}:\n{result_variable} = {result_value}",
            &["variable", "value", "variable_expr", "expression", "result_variable", "result_value"]
        )
    );
}

/// Initialize elimination method messages
fn initialize_elimination_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemElimination, 0),
        MessageTemplate::new(
            "Elimination Method Introduction",
            "Solve system using elimination (addition) method\nAlign equations and add/subtract to eliminate one variable",
            &[]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemElimination, 1),
        MessageTemplate::new(
            "Elimination Step 1: Align Equations",
            "Write system in standard form:\nEquation 1: {equation1}\nEquation 2: {equation2}",
            &["equation1", "equation2"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemElimination, 2),
        MessageTemplate::new(
            "Elimination Step 2: Multiply for Elimination",
            "To eliminate {variable}, multiply:\nEquation 1 by {multiplier1}: {new_equation1}\nEquation 2 by {multiplier2}: {new_equation2}",
            &["variable", "multiplier1", "new_equation1", "multiplier2", "new_equation2"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemElimination, 3),
        MessageTemplate::new(
            "Elimination Step 3: Add/Subtract Equations",
            "{operation} the equations to eliminate {variable}:\n({equation1}) {operator} ({equation2})\nResult: {resulting_equation}",
            &["operation", "variable", "equation1", "operator", "equation2", "resulting_equation"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemElimination, 4),
        MessageTemplate::new(
            "Elimination Step 4: Solve for Remaining Variable",
            "Solve simplified equation for {variable}:\n{equation}\n{variable} = {solution}",
            &["variable", "equation", "solution"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemElimination, 5),
        MessageTemplate::new(
            "Elimination Step 5: Find Other Variable",
            "Substitute {variable} = {value} into original equation:\n{equation}\nSolve for {other_variable}: {other_variable} = {other_value}",
            &["variable", "value", "equation", "other_variable", "other_value"]
        )
    );
}

/// Initialize matrix method messages
fn initialize_matrix_method_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemMatrix, 0),
        MessageTemplate::new(
            "Matrix Method Introduction",
            "Solve system using matrix form: Ax = b\nWrite system as augmented matrix and row reduce",
            &[]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemMatrix, 1),
        MessageTemplate::new(
            "Matrix Form Setup",
            "Express system as matrix equation:\nCoefficient matrix A: {coefficient_matrix}\nVariable vector x: {variable_vector}\nConstant vector b: {constant_vector}",
            &["coefficient_matrix", "variable_vector", "constant_vector"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemMatrix, 2),
        MessageTemplate::new(
            "Augmented Matrix",
            "Form augmented matrix [A|b]:\n{augmented_matrix}\nApply row operations to reduce",
            &["augmented_matrix"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemMatrix, 3),
        MessageTemplate::new(
            "Row Reduction Step",
            "Row operation: {operation}\nBefore: {matrix_before}\nAfter: {matrix_after}",
            &["operation", "matrix_before", "matrix_after"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::SystemMatrix, 4),
        MessageTemplate::new(
            "Reduced Row Echelon Form",
            "Matrix in reduced row echelon form:\n{rref_matrix}\nRead solutions directly from final column",
            &["rref_matrix"]
        )
    );
}

/// Initialize solution interpretation messages
fn initialize_solution_interpretation_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::Result, 0),
        MessageTemplate::new(
            "Unique Solution Found",
            "System has unique solution:\n{solution_display}\nThis is the only point that satisfies all equations",
            &["solution_display"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::Result, 1),
        MessageTemplate::new(
            "No Solution",
            "System has no solution (inconsistent)\n{contradiction_explanation}\nThe equations represent parallel lines/planes that never intersect",
            &["contradiction_explanation"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::Result, 2),
        MessageTemplate::new(
            "Infinitely Many Solutions",
            "System has infinitely many solutions (dependent)\nSolution set: {parametric_solution}\nThe equations represent the same line/plane",
            &["parametric_solution"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::Verification, 0),
        MessageTemplate::new(
            "Verify System Solution",
            "Check solution {solution} in all equations:\nEquation 1: {verification1}\nEquation 2: {verification2}\n{verification_conclusion}",
            &["solution", "verification1", "verification2", "verification_conclusion"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::Insight, 0),
        MessageTemplate::new(
            "System Solving Strategy",
            "Choosing solution method:\nSubstitution: Best when one variable is already isolated\nElimination: Best when coefficients line up nicely\nMatrix: Best for large systems (3+ equations)",
            &[]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::Insight, 1),
        MessageTemplate::new(
            "Geometric Interpretation",
            "Geometric meaning for {dimension}D system:\n{geometric_description}\nSolution represents intersection point(s)",
            &["dimension", "geometric_description"]
        )
    );
}

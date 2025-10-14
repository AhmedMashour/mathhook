//! Algebra educational messages for simplification, expansion, and factorization

use super::core::{MessageCategory, MessageKey, MessageTemplate, MessageType};
use std::collections::HashMap;

/// Initialize all algebra-related messages
pub fn initialize_algebra_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    initialize_simplification_messages(registry);
    initialize_expansion_messages(registry);
    initialize_factorization_messages(registry);
    initialize_rational_messages(registry);
    initialize_polynomial_messages(registry);
}

/// Initialize simplification educational messages
fn initialize_simplification_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::SimplifyCombineLike, 0),
        MessageTemplate::new(
            "Combine Like Terms",
            "Combine like terms in {expression}\nLike terms have the same variable parts: {like_terms}",
            &["expression", "like_terms"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::SimplifyCombineLike, 1),
        MessageTemplate::new(
            "Combining Like Terms Step",
            "Combine {term1} and {term2}:\n{term1} + {term2} = ({coeff1} + {coeff2})*{variable_part} = {result}",
            &["term1", "term2", "coeff1", "coeff2", "variable_part", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::SimplifyIdentity, 0),
        MessageTemplate::new(
            "Identity Element",
            "Apply identity property: {property_description}\nSimplify {expression} to {result}",
            &["property_description", "expression", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::SimplifyIdentity, 1),
        MessageTemplate::new(
            "Additive Identity",
            "Additive identity: {expression} + 0 = {expression}\nThe term + 0 can be removed",
            &["expression"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::SimplifyIdentity, 2),
        MessageTemplate::new(
            "Multiplicative Identity",
            "Multiplicative identity: {expression} * 1 = {expression}\nThe factor * 1 can be removed",
            &["expression"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::SimplifyIdentity, 3),
        MessageTemplate::new(
            "Zero Property of Multiplication",
            "Zero property: {expression} * 0 = 0\nAny expression multiplied by zero equals zero",
            &["expression"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::Step, 0),
        MessageTemplate::new(
            "Collect Variable Terms",
            "Collect all terms containing {variable} on one side:\n{before} becomes {after}",
            &["variable", "before", "after"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::Step, 1),
        MessageTemplate::new(
            "Simplification Strategy",
            "Strategy to simplify {expression}:\n{strategy_steps}",
            &["expression", "strategy_steps"]
        )
    );
}

/// Initialize expansion educational messages
fn initialize_expansion_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::ExpandDistributive, 0),
        MessageTemplate::new(
            "Distributive Property",
            "Apply distributive property: a(b + c) = ab + ac\nDistribute {factor} over {expression}",
            &["factor", "expression"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::ExpandDistributive, 1),
        MessageTemplate::new(
            "Distribute Factor",
            "Distribute {factor} across ({sum_expression}):\n{factor}*({term1}) + {factor}*({term2}) = {result}",
            &["factor", "sum_expression", "term1", "term2", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::ExpandFOIL, 0),
        MessageTemplate::new(
            "FOIL Method",
            "Use FOIL (First, Outer, Inner, Last) to expand:\n({first_binomial})*({second_binomial})",
            &["first_binomial", "second_binomial"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::ExpandFOIL, 1),
        MessageTemplate::new(
            "FOIL Expansion Steps",
            "Expanding ({a} + {b})({c} + {d}):\nFirst: {a}*{c} = {first_term}\nOuter: {a}*{d} = {outer_term}\nInner: {b}*{c} = {inner_term}\nLast: {b}*{d} = {last_term}\nSum: {result}",
            &["a", "b", "c", "d", "first_term", "outer_term", "inner_term", "last_term", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::ExpandBinomial, 0),
        MessageTemplate::new(
            "Binomial Expansion",
            "Expand binomial power: ({expression})^{power}\nUse binomial theorem or repeated multiplication",
            &["expression", "power"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::ExpandBinomial, 1),
        MessageTemplate::new(
            "Perfect Square Expansion",
            "Perfect square: ({a} + {b})^2 = {a}^2 + 2*{a}*{b} + {b}^2\nApply to get: {result}",
            &["a", "b", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::ExpandBinomial, 2),
        MessageTemplate::new(
            "Difference of Squares Pattern",
            "Recognize pattern: ({a} + {b})({a} - {b}) = {a}^2 - {b}^2\nResult: {result}",
            &["a", "b", "result"]
        )
    );
}

/// Initialize factorization educational messages
fn initialize_factorization_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::FactorCommon, 0),
        MessageTemplate::new(
            "Common Factor",
            "Factor out greatest common factor (GCF) from {expression}\nGCF = {gcf}",
            &["expression", "gcf"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::FactorCommon, 1),
        MessageTemplate::new(
            "Extract Common Factor",
            "Extract {gcf} from each term:\n{expression} = {gcf}*({factored_expression})",
            &["gcf", "expression", "factored_expression"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::FactorGrouping, 0),
        MessageTemplate::new(
            "Factoring by Grouping",
            "Factor by grouping for {expression}\nGroup terms: ({group1}) + ({group2})",
            &["expression", "group1", "group2"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::FactorGrouping, 1),
        MessageTemplate::new(
            "Grouping Method Steps",
            "Step 1: Group {group1} has common factor {factor1}\nStep 2: Group {group2} has common factor {factor2}\nStep 3: Factor out common binomial: {result}",
            &["group1", "factor1", "group2", "factor2", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::FactorQuadratic, 0),
        MessageTemplate::new(
            "Factor Quadratic",
            "Factor quadratic {expression}\nFind two numbers that multiply to {product} and add to {sum}",
            &["expression", "product", "sum"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::FactorQuadratic, 1),
        MessageTemplate::new(
            "Quadratic Factoring Pattern",
            "Factor {a}*x^2 + {b}*x + {c}\nFind factors of {a}*{c} that sum to {b}\nFactors found: {factor1} and {factor2}\nFactored form: {result}",
            &["a", "b", "c", "factor1", "factor2", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::FactorQuadratic, 2),
        MessageTemplate::new(
            "Difference of Squares Factoring",
            "Recognize difference of squares: {a}^2 - {b}^2\nFactor as: ({a} + {b})({a} - {b})",
            &["a", "b"]
        )
    );
}

/// Initialize rational expression messages
fn initialize_rational_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::RationalSimplify, 0),
        MessageTemplate::new(
            "Simplify Rational Expression",
            "Simplify rational expression: {expression}\nFactor numerator and denominator, then cancel common factors",
            &["expression"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::RationalSimplify, 1),
        MessageTemplate::new(
            "Cancel Common Factors",
            "After factoring:\nNumerator: {numerator_factored}\nDenominator: {denominator_factored}\nCommon factor: {common_factor}\nSimplified: {result}",
            &["numerator_factored", "denominator_factored", "common_factor", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::RationalSimplify, 2),
        MessageTemplate::new(
            "Rational Expression Addition",
            "Add rational expressions: {expr1} + {expr2}\nFind common denominator: {common_denominator}",
            &["expr1", "expr2", "common_denominator"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Algebra, MessageType::RationalSimplify, 3),
        MessageTemplate::new(
            "Rational Expression Multiplication",
            "Multiply rational expressions: ({numerator1}/{denominator1}) * ({numerator2}/{denominator2})\nMultiply numerators and denominators:\n({numerator1}*{numerator2})/({denominator1}*{denominator2}) = {result}",
            &["numerator1", "denominator1", "numerator2", "denominator2", "result"]
        )
    );
}

/// Initialize polynomial equation messages
fn initialize_polynomial_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::PolynomialEquation, MessageType::Introduction, 0),
        MessageTemplate::new(
            "Polynomial Equation",
            "Solve polynomial equation: {equation} = 0\nDegree: {degree} ({degree_name})",
            &["equation", "degree", "degree_name"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::PolynomialEquation, MessageType::Strategy, 0),
        MessageTemplate::new(
            "Polynomial Solution Strategy",
            "Strategy for degree {degree} polynomial:\n{strategy_description}\nWe will use the Rational Root Theorem and factorization",
            &["degree", "strategy_description"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::PolynomialEquation, MessageType::PolynomialRationalRoot, 0),
        MessageTemplate::new(
            "Rational Root Theorem",
            "Apply Rational Root Theorem:\nPossible rational roots are p/q where:\n- p divides constant term: {constant_term}\n- q divides leading coefficient: {leading_coeff}\nCandidates: {candidates}",
            &["constant_term", "leading_coeff", "candidates"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::PolynomialEquation, MessageType::PolynomialRationalRoot, 1),
        MessageTemplate::new(
            "Test Rational Root Candidate",
            "Test {variable} = {candidate}:\nSubstitute into {equation}:\nP({candidate}) = {evaluation} {result}",
            &["variable", "candidate", "equation", "evaluation", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::PolynomialEquation, MessageType::PolynomialSyntheticDivision, 0),
        MessageTemplate::new(
            "Synthetic Division",
            "Factor out ({variable} - {root}) using synthetic division:\nDivide {polynomial} by ({variable} - {root})",
            &["variable", "root", "polynomial"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::PolynomialEquation, MessageType::PolynomialSyntheticDivision, 1),
        MessageTemplate::new(
            "Synthetic Division Result",
            "After dividing by ({variable} - {root}):\n{original_polynomial} = ({variable} - {root}) * ({quotient_polynomial})",
            &["variable", "root", "original_polynomial", "quotient_polynomial"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::PolynomialEquation, MessageType::PolynomialFactorization, 0),
        MessageTemplate::new(
            "Complete Factorization",
            "Complete factorization:\n{original_polynomial} = {factored_form}\nEach factor gives a root",
            &["original_polynomial", "factored_form"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::PolynomialEquation, MessageType::Result, 0),
        MessageTemplate::new(
            "Polynomial Solutions",
            "Solutions to {equation} = 0:\n{solutions}\nTotal: {count} roots found",
            &["equation", "solutions", "count"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::PolynomialEquation, MessageType::Verification, 0),
        MessageTemplate::new(
            "Verify Polynomial Root",
            "Verify {variable} = {root}:\nSubstitute into original equation:\n{verification_expression} = {result}",
            &["variable", "root", "verification_expression", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::PolynomialEquation, MessageType::Insight, 0),
        MessageTemplate::new(
            "Fundamental Theorem of Algebra",
            "By the Fundamental Theorem of Algebra, a polynomial of degree {degree} has exactly {degree} roots (counting multiplicity) in the complex numbers.\nWe found {real_count} real roots",
            &["degree", "real_count"]
        )
    );
}

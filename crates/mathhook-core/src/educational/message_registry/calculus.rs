//! Calculus educational messages for derivatives, integrals, and limits

use super::core::{MessageCategory, MessageKey, MessageTemplate, MessageType};
use std::collections::HashMap;

/// Initialize all calculus-related messages
pub fn initialize_calculus_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    initialize_derivative_messages(registry);
    initialize_integral_messages(registry);
    initialize_limit_messages(registry);
}

/// Initialize derivative educational messages
fn initialize_derivative_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(
            MessageCategory::Calculus,
            MessageType::DerivativePowerRule,
            0,
        ),
        MessageTemplate::new(
            "Power Rule",
            "Apply power rule: d/dx(x^n) = n*x^(n-1)\nFor {expression}, we have n = {exponent}",
            &["expression", "exponent"],
        ),
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativePowerRule, 1),
        MessageTemplate::new(
            "Power Rule Application",
            "Using power rule on {expression}:\nd/dx({base}^{exponent}) = {exponent}*{base}^({exponent_minus_one})",
            &["expression", "base", "exponent", "exponent_minus_one"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativePowerRule, 2),
        MessageTemplate::new(
            "Power Rule for Negative Exponents",
            "For negative exponents: d/dx(x^(-n)) = -n*x^(-n-1)\nApplying to {expression} with exponent {exponent}",
            &["expression", "exponent"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativeChainRule, 0),
        MessageTemplate::new(
            "Chain Rule",
            "Apply chain rule: d/dx(f(g(x))) = f'(g(x)) * g'(x)\nOuter function: {outer_function}\nInner function: {inner_function}",
            &["outer_function", "inner_function"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativeChainRule, 1),
        MessageTemplate::new(
            "Chain Rule Step 1: Identify Functions",
            "Identify composition of functions in {expression}\nOuter function f(u) = {outer_function}\nInner function u = {inner_function}",
            &["expression", "outer_function", "inner_function"]
        )
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::Calculus,
            MessageType::DerivativeChainRule,
            2,
        ),
        MessageTemplate::new(
            "Chain Rule Step 2: Differentiate Outer",
            "Differentiate outer function with respect to inner:\nf'(u) = {outer_derivative}",
            &["outer_derivative"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::Calculus,
            MessageType::DerivativeChainRule,
            3,
        ),
        MessageTemplate::new(
            "Chain Rule Step 3: Differentiate Inner",
            "Differentiate inner function:\ng'(x) = {inner_derivative}",
            &["inner_derivative"],
        ),
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativeChainRule, 4),
        MessageTemplate::new(
            "Chain Rule Step 4: Multiply",
            "Multiply the derivatives:\nd/dx({expression}) = ({outer_derivative}) * ({inner_derivative}) = {result}",
            &["expression", "outer_derivative", "inner_derivative", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativeProductRule, 0),
        MessageTemplate::new(
            "Product Rule",
            "Apply product rule: d/dx(u*v) = u'*v + u*v'\nFirst function: {first_function}\nSecond function: {second_function}",
            &["first_function", "second_function"]
        )
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::Calculus,
            MessageType::DerivativeProductRule,
            1,
        ),
        MessageTemplate::new(
            "Product Rule Step 1: Identify Factors",
            "Identify the two factors in {expression}\nu = {first_function}\nv = {second_function}",
            &["expression", "first_function", "second_function"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::Calculus,
            MessageType::DerivativeProductRule,
            2,
        ),
        MessageTemplate::new(
            "Product Rule Step 2: Differentiate Each",
            "Find derivatives of each factor:\nu' = {first_derivative}\nv' = {second_derivative}",
            &["first_derivative", "second_derivative"],
        ),
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativeProductRule, 3),
        MessageTemplate::new(
            "Product Rule Step 3: Apply Formula",
            "Apply formula u'v + uv':\n({first_derivative})*({second_function}) + ({first_function})*({second_derivative}) = {result}",
            &["first_derivative", "second_function", "first_function", "second_derivative", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativeQuotientRule, 0),
        MessageTemplate::new(
            "Quotient Rule",
            "Apply quotient rule: d/dx(u/v) = (u'*v - u*v') / v^2\nNumerator: {numerator}\nDenominator: {denominator}",
            &["numerator", "denominator"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativeQuotientRule, 1),
        MessageTemplate::new(
            "Quotient Rule Step 1: Identify Parts",
            "Identify numerator and denominator in {expression}\nu = {numerator}\nv = {denominator}",
            &["expression", "numerator", "denominator"]
        )
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::Calculus,
            MessageType::DerivativeQuotientRule,
            2,
        ),
        MessageTemplate::new(
            "Quotient Rule Step 2: Differentiate",
            "Find derivatives:\nu' = {numerator_derivative}\nv' = {denominator_derivative}",
            &["numerator_derivative", "denominator_derivative"],
        ),
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativeQuotientRule, 3),
        MessageTemplate::new(
            "Quotient Rule Step 3: Apply Formula",
            "Apply formula (u'v - uv')/v^2:\n(({numerator_derivative})*({denominator}) - ({numerator})*({denominator_derivative})) / ({denominator})^2 = {result}",
            &["numerator_derivative", "denominator", "numerator", "denominator_derivative", "result"]
        )
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::Calculus,
            MessageType::DerivativeConstant,
            0,
        ),
        MessageTemplate::new(
            "Derivative of Constant",
            "The derivative of a constant is zero.\nd/dx({constant}) = 0",
            &["constant"],
        ),
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativeConstant, 1),
        MessageTemplate::new(
            "Constant Multiple Rule",
            "Constant multiple rule: d/dx(c*f(x)) = c*f'(x)\nConstant: {constant}\nFunction: {function}\nDerivative: {constant}*({derivative}) = {result}",
            &["constant", "function", "derivative", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativeVariable, 0),
        MessageTemplate::new(
            "Derivative of Variable",
            "The derivative of {variable} with respect to {variable} is 1.\nd/d{variable}({variable}) = 1",
            &["variable"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativeImplicit, 0),
        MessageTemplate::new(
            "Implicit Differentiation",
            "For implicit differentiation of {equation}:\nDifferentiate both sides with respect to {variable}",
            &["equation", "variable"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativeImplicit, 1),
        MessageTemplate::new(
            "Implicit Differentiation: Apply Chain Rule",
            "When differentiating {term} containing {dependent_variable}:\nApply chain rule and multiply by d{dependent_variable}/d{independent_variable}",
            &["term", "dependent_variable", "independent_variable"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativeHigherOrder, 0),
        MessageTemplate::new(
            "Higher Order Derivative",
            "Finding {order} order derivative of {expression}\nTake derivative {order} times with respect to {variable}",
            &["order", "expression", "variable"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::DerivativeHigherOrder, 1),
        MessageTemplate::new(
            "Second Derivative",
            "Second derivative d^2/dx^2({expression}):\nFirst find first derivative, then differentiate again\nFirst derivative: {first_derivative}\nSecond derivative: {second_derivative}",
            &["expression", "first_derivative", "second_derivative"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::Introduction, 0),
        MessageTemplate::new(
            "Derivative Introduction",
            "Finding derivative of {expression} with respect to {variable}\nThe derivative measures the rate of change",
            &["expression", "variable"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::Strategy, 0),
        MessageTemplate::new(
            "Differentiation Strategy",
            "To find d/d{variable}({expression}):\n{strategy_description}",
            &["variable", "expression", "strategy_description"],
        ),
    );
}

/// Initialize integral educational messages
fn initialize_integral_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::IntegralPowerRule, 0),
        MessageTemplate::new(
            "Power Rule for Integration",
            "Integral power rule: integral(x^n dx) = x^(n+1)/(n+1) + C, where n != -1\nFor {expression}, we have n = {exponent}",
            &["expression", "exponent"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::IntegralPowerRule, 1),
        MessageTemplate::new(
            "Apply Integration Power Rule",
            "Integrating {expression}:\nintegral({base}^{exponent} d{variable}) = {base}^({exponent_plus_one})/({exponent_plus_one}) + C",
            &["expression", "base", "exponent", "variable", "exponent_plus_one"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::IntegralConstant, 0),
        MessageTemplate::new(
            "Integral of Constant",
            "Integral of a constant: integral(c dx) = c*x + C\nFor constant {constant}: integral({constant} dx) = {constant}*x + C",
            &["constant"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::IntegralConstant, 1),
        MessageTemplate::new(
            "Constant Multiple Rule for Integration",
            "Constant multiple rule: integral(c*f(x) dx) = c*integral(f(x) dx)\nFactor out constant {constant} and integrate {function}",
            &["constant", "function"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::IntegralUSubstitution, 0),
        MessageTemplate::new(
            "U-Substitution Method",
            "Use u-substitution for {expression}\nLet u = {substitution}\nThen du = {du_expression} dx",
            &["expression", "substitution", "du_expression"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::IntegralUSubstitution, 1),
        MessageTemplate::new(
            "U-Substitution Step 1: Choose u",
            "Choose substitution u = {substitution} for {expression}\nThis simplifies the integral structure",
            &["substitution", "expression"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::IntegralUSubstitution, 2),
        MessageTemplate::new(
            "U-Substitution Step 2: Find du",
            "Differentiate u = {substitution}:\ndu = ({du_expression}) dx\nSolve for dx: dx = {dx_in_terms_of_du}",
            &["substitution", "du_expression", "dx_in_terms_of_du"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::IntegralUSubstitution, 3),
        MessageTemplate::new(
            "U-Substitution Step 3: Substitute",
            "Substitute into integral:\nOriginal: integral({original_integrand} dx)\nAfter substitution: integral({new_integrand} du)",
            &["original_integrand", "new_integrand"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::IntegralUSubstitution, 4),
        MessageTemplate::new(
            "U-Substitution Step 4: Back-Substitute",
            "After integrating in terms of u: {result_in_u}\nSubstitute back u = {substitution}: {final_result}",
            &["result_in_u", "substitution", "final_result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::IntegralByParts, 0),
        MessageTemplate::new(
            "Integration by Parts",
            "Use integration by parts: integral(u dv) = uv - integral(v du)\nChoose u = {u_choice}, dv = {dv_choice}",
            &["u_choice", "dv_choice"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::IntegralByParts, 1),
        MessageTemplate::new(
            "Integration by Parts: LIATE Rule",
            "Use LIATE rule to choose u (prioritize: Logarithmic, Inverse trig, Algebraic, Trig, Exponential)\nFor {expression}, choose u = {u_choice}",
            &["expression", "u_choice"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::IntegralDefinite, 0),
        MessageTemplate::new(
            "Definite Integral Evaluation",
            "Evaluate definite integral from {lower_bound} to {upper_bound}\nFirst find antiderivative, then apply bounds",
            &["lower_bound", "upper_bound"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::IntegralDefinite, 1),
        MessageTemplate::new(
            "Fundamental Theorem of Calculus",
            "By Fundamental Theorem: integral from {lower_bound} to {upper_bound} of f(x) dx = F({upper_bound}) - F({lower_bound})\nWhere F is antiderivative: {antiderivative}",
            &["lower_bound", "upper_bound", "antiderivative"]
        )
    );
}

/// Initialize limit educational messages
fn initialize_limit_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::LimitDirect, 0),
        MessageTemplate::new(
            "Direct Substitution",
            "Evaluate limit as {variable} approaches {point} by direct substitution\nSubstitute {variable} = {point} into {expression}",
            &["variable", "point", "expression"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::LimitDirect, 1),
        MessageTemplate::new(
            "Direct Substitution Result",
            "Direct substitution gives:\nlim({variable} -> {point}) {expression} = {result}",
            &["variable", "point", "expression", "result"],
        ),
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::LimitIndeterminate, 0),
        MessageTemplate::new(
            "Indeterminate Form Detected",
            "Direct substitution gives indeterminate form: {indeterminate_form}\nNeed algebraic manipulation or L'Hopital's rule",
            &["indeterminate_form"]
        )
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::Calculus,
            MessageType::LimitIndeterminate,
            1,
        ),
        MessageTemplate::new(
            "Resolve Indeterminate Form",
            "To resolve {indeterminate_form} form:\n{resolution_strategy}",
            &["indeterminate_form", "resolution_strategy"],
        ),
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::LimitLHopital, 0),
        MessageTemplate::new(
            "L'Hopital's Rule",
            "Apply L'Hopital's rule for {indeterminate_form} form:\nlim(f(x)/g(x)) = lim(f'(x)/g'(x))\nNumerator: {numerator}\nDenominator: {denominator}",
            &["indeterminate_form", "numerator", "denominator"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::LimitLHopital, 1),
        MessageTemplate::new(
            "L'Hopital's Rule Application",
            "Differentiate numerator and denominator:\nNumerator derivative: {numerator_derivative}\nDenominator derivative: {denominator_derivative}\nNew limit: lim({variable} -> {point}) ({numerator_derivative})/({denominator_derivative})",
            &["numerator_derivative", "denominator_derivative", "variable", "point"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::LimitLaws, 0),
        MessageTemplate::new(
            "Limit Laws",
            "Apply limit laws:\nlim(f(x) {operator} g(x)) = lim(f(x)) {operator} lim(g(x))\nEvaluate each limit separately",
            &["operator"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::LimitLaws, 1),
        MessageTemplate::new(
            "Product Limit Law",
            "Product law: lim(f(x)*g(x)) = lim(f(x)) * lim(g(x))\nEvaluate:\nlim(f(x)) = {first_limit}\nlim(g(x)) = {second_limit}\nProduct: {result}",
            &["first_limit", "second_limit", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::LimitOneSided, 0),
        MessageTemplate::new(
            "One-Sided Limit",
            "Evaluate one-sided limit as {variable} approaches {point} from {direction}\nNotation: lim({variable} -> {point}{direction_symbol})",
            &["variable", "point", "direction", "direction_symbol"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::Calculus, MessageType::LimitOneSided, 1),
        MessageTemplate::new(
            "Compare One-Sided Limits",
            "Left-hand limit: lim({variable} -> {point}-) = {left_limit}\nRight-hand limit: lim({variable} -> {point}+) = {right_limit}\n{comparison_conclusion}",
            &["variable", "point", "left_limit", "right_limit", "comparison_conclusion"]
        )
    );
}

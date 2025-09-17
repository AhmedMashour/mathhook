//! ODE educational messages for differential equation solving

use super::core::{MessageCategory, MessageKey, MessageTemplate, MessageType};
use std::collections::HashMap;

/// Initialize all ODE-related messages
pub fn initialize_ode_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    initialize_separable_messages(registry);
    initialize_linear_ode_messages(registry);
    initialize_homogeneous_messages(registry);
    initialize_exact_messages(registry);
    initialize_bernoulli_messages(registry);
    initialize_second_order_messages(registry);
}

fn initialize_separable_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODESeparable,
            0,
        ),
        MessageTemplate::new(
            "Separable ODE Identified",
            "This is a separable ODE: dy/dx = {rhs}\nWe can write it as: dy/dx = g({independent})*h({dependent})",
            &["rhs", "independent", "dependent"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODESeparable,
            1,
        ),
        MessageTemplate::new(
            "Separate Variables",
            "Separate variables by dividing both sides:\n(1/h({dependent})) d{dependent} = g({independent}) d{independent}",
            &["dependent", "independent"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODESeparable,
            2,
        ),
        MessageTemplate::new(
            "Integrate Both Sides",
            "Integrate both sides:\n∫ {y_integral} d{dependent} = ∫ {x_integral} d{independent}",
            &["y_integral", "dependent", "x_integral", "independent"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODESeparable,
            3,
        ),
        MessageTemplate::new(
            "General Solution",
            "After integration and adding constant C:\n{solution}",
            &["solution"],
        ),
    );
}

fn initialize_linear_ode_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODELinear,
            0,
        ),
        MessageTemplate::new(
            "Linear First-Order ODE",
            "This is a linear first-order ODE: dy/dx + P({independent}){dependent} = Q({independent})\nWhere P({independent}) = {p_func} and Q({independent}) = {q_func}",
            &["independent", "dependent", "p_func", "q_func"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEIntegratingFactor,
            0,
        ),
        MessageTemplate::new(
            "Compute Integrating Factor",
            "Compute integrating factor μ({independent}) = exp(∫ P({independent}) d{independent})\nμ({independent}) = exp(∫ {p_func} d{independent}) = {integrating_factor}",
            &["independent", "p_func", "integrating_factor"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODELinear,
            1,
        ),
        MessageTemplate::new(
            "Multiply by Integrating Factor",
            "Multiply both sides by μ({independent}) = {integrating_factor}:\nd/d{independent}[μ({independent}) {dependent}] = μ({independent}) Q({independent})",
            &["independent", "integrating_factor", "dependent"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODELinear,
            2,
        ),
        MessageTemplate::new(
            "Integrate to Solve",
            "Integrate both sides:\nμ({independent}) {dependent} = ∫ μ({independent}) Q({independent}) d{independent}\n{dependent} = (1/μ({independent})) [∫ {integral_expr} d{independent} + C]",
            &["independent", "dependent", "integral_expr"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODELinear,
            3,
        ),
        MessageTemplate::new(
            "General Solution",
            "General solution:\n{dependent}({independent}) = {solution}",
            &["dependent", "independent", "solution"],
        ),
    );
}

fn initialize_homogeneous_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEHomogeneous,
            0,
        ),
        MessageTemplate::new(
            "Homogeneous ODE Identified",
            "This is a homogeneous ODE: dy/dx = f({dependent}/{independent})\nThe right-hand side depends only on the ratio {dependent}/{independent}",
            &["dependent", "independent"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODESubstitution,
            0,
        ),
        MessageTemplate::new(
            "Substitution v = y/x",
            "Let v = {dependent}/{independent}, so {dependent} = v{independent}\nThen dy/d{independent} = v + {independent}(dv/d{independent})",
            &["dependent", "independent"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEHomogeneous,
            1,
        ),
        MessageTemplate::new(
            "Transform to Separable",
            "Substituting into the ODE:\nv + {independent}(dv/d{independent}) = f(v)\n{independent}(dv/d{independent}) = f(v) - v\nThis is now separable!",
            &["independent"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEHomogeneous,
            2,
        ),
        MessageTemplate::new(
            "Back-Substitute",
            "After solving for v, substitute back v = {dependent}/{independent}:\n{solution}",
            &["dependent", "independent", "solution"],
        ),
    );
}

fn initialize_exact_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEExact,
            0,
        ),
        MessageTemplate::new(
            "Exact ODE Identified",
            "This is an exact ODE: M({independent}, {dependent})d{independent} + N({independent}, {dependent})d{dependent} = 0\nWhere ∂M/∂{dependent} = ∂N/∂{independent}",
            &["independent", "dependent"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEExact,
            1,
        ),
        MessageTemplate::new(
            "Find Potential Function",
            "Find function F({independent}, {dependent}) such that:\n∂F/∂{independent} = M({independent}, {dependent})\n∂F/∂{dependent} = N({independent}, {dependent})",
            &["independent", "dependent"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEExact,
            2,
        ),
        MessageTemplate::new(
            "Implicit Solution",
            "The solution is given implicitly by:\nF({independent}, {dependent}) = C\nWhere {solution_implicit}",
            &["independent", "dependent", "solution_implicit"],
        ),
    );
}

fn initialize_bernoulli_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEBernoulli,
            0,
        ),
        MessageTemplate::new(
            "Bernoulli Equation",
            "This is a Bernoulli equation: dy/dx + P({independent}){dependent} = Q({independent}){dependent}^n\nWith n = {n_value}, P = {p_func}, Q = {q_func}",
            &["independent", "dependent", "n_value", "p_func", "q_func"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODESubstitution,
            1,
        ),
        MessageTemplate::new(
            "Substitution to Linearize",
            "Let v = {dependent}^(1-n) = {dependent}^{exponent}\nThen dv/d{independent} = (1-n){dependent}^(-n) d{dependent}/d{independent}",
            &["dependent", "exponent", "independent"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEBernoulli,
            1,
        ),
        MessageTemplate::new(
            "Linear ODE in v",
            "After substitution, we get a linear ODE:\ndv/d{independent} + (1-n)P({independent})v = (1-n)Q({independent})\nSolve this linear ODE for v, then substitute back",
            &["independent"],
        ),
    );
}

fn initialize_second_order_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEConstantCoefficients,
            0,
        ),
        MessageTemplate::new(
            "Second-Order Constant Coefficients",
            "This is a second-order linear ODE with constant coefficients:\na{y_double_prime} + b{y_prime} + c{dependent} = {rhs}\nCoefficients: a = {a_val}, b = {b_val}, c = {c_val}",
            &["y_double_prime", "y_prime", "dependent", "rhs", "a_val", "b_val", "c_val"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODECharacteristicEquation,
            0,
        ),
        MessageTemplate::new(
            "Characteristic Equation",
            "Form the characteristic equation by assuming {dependent} = exp(r{independent}):\nar^2 + br + c = 0\n{a_val}r^2 + {b_val}r + {c_val} = 0",
            &["dependent", "independent", "a_val", "b_val", "c_val"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODECharacteristicEquation,
            1,
        ),
        MessageTemplate::new(
            "Solve for Roots",
            "Using quadratic formula:\nr = (-b ± sqrt(b^2 - 4ac)) / (2a)\nr = {roots}\nDiscriminant: {discriminant}",
            &["roots", "discriminant"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEConstantCoefficients,
            1,
        ),
        MessageTemplate::new(
            "Homogeneous Solution (Real Distinct Roots)",
            "Since roots r1 = {r1} and r2 = {r2} are real and distinct:\n{dependent}_h({independent}) = C1*exp({r1}{independent}) + C2*exp({r2}{independent})",
            &["r1", "r2", "dependent", "independent"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEConstantCoefficients,
            2,
        ),
        MessageTemplate::new(
            "Homogeneous Solution (Repeated Root)",
            "Since roots are equal r1 = r2 = {r}:\n{dependent}_h({independent}) = (C1 + C2{independent})*exp({r}{independent})",
            &["r", "dependent", "independent"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEConstantCoefficients,
            3,
        ),
        MessageTemplate::new(
            "Homogeneous Solution (Complex Roots)",
            "Since roots are complex r = {alpha} ± {beta}i:\n{dependent}_h({independent}) = exp({alpha}{independent})[C1*cos({beta}{independent}) + C2*sin({beta}{independent})]",
            &["alpha", "beta", "dependent", "independent"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEUndeterminedCoefficients,
            0,
        ),
        MessageTemplate::new(
            "Method of Undetermined Coefficients",
            "For non-homogeneous ODE with {rhs_type} forcing:\nGuess particular solution form: {particular_guess}",
            &["rhs_type", "particular_guess"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODEVariationParameters,
            0,
        ),
        MessageTemplate::new(
            "Variation of Parameters",
            "Use variation of parameters for general forcing function.\nAssume {dependent}_p = u1({independent}){y1} + u2({independent}){y2}\nWhere {y1}, {y2} are homogeneous solutions",
            &["dependent", "independent", "y1", "y2"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODECauchyEuler,
            0,
        ),
        MessageTemplate::new(
            "Cauchy-Euler Equation",
            "This is a Cauchy-Euler (equidimensional) equation:\na{independent}^2{y_double_prime} + b{independent}{y_prime} + c{dependent} = 0\nSubstitute {dependent} = {independent}^r",
            &["independent", "y_double_prime", "y_prime", "dependent"],
        ),
    );

    registry.insert(
        MessageKey::new(
            MessageCategory::OrdinaryDifferentialEquation,
            MessageType::ODECauchyEuler,
            1,
        ),
        MessageTemplate::new(
            "Indicial Equation",
            "After substitution, the indicial equation is:\nar(r-1) + br + c = 0\n{indicial_eq}\nSolve for r",
            &["indicial_eq"],
        ),
    );
}

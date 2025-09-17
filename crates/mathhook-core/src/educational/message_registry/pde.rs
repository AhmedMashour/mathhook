//! PDE-specific educational messages integrated with the main message registry
//!
//! This module provides educational messages for partial differential equation concepts.

use super::core::{MessageCategory, MessageKey, MessageType};
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Create a PDE message key
pub fn pde_message_key(message_type: MessageType, variant: u8) -> MessageKey {
    MessageKey {
        category: MessageCategory::PartialDifferentialEquation,
        message_type,
        variant,
    }
}

/// PDE message variants
pub struct PdeMessageVariant;

impl PdeMessageVariant {
    // General PDE concepts
    pub const WHAT_IS_PDE: u8 = 1;
    pub const PDE_VS_ODE: u8 = 2;
    pub const PDE_CLASSIFICATION: u8 = 3;

    // PDE types
    pub const ELLIPTIC_EQUATION: u8 = 4;
    pub const PARABOLIC_EQUATION: u8 = 5;
    pub const HYPERBOLIC_EQUATION: u8 = 6;

    // Standard PDEs
    pub const HEAT_EQUATION: u8 = 7;
    pub const WAVE_EQUATION: u8 = 8;
    pub const LAPLACE_EQUATION: u8 = 9;
    pub const POISSON_EQUATION: u8 = 10;

    // Solution methods
    pub const SEPARATION_OF_VARIABLES: u8 = 11;
    pub const METHOD_OF_CHARACTERISTICS: u8 = 12;
    pub const FOURIER_SERIES: u8 = 13;
    pub const GREENS_FUNCTIONS: u8 = 14;

    // Boundary conditions
    pub const DIRICHLET_CONDITION: u8 = 15;
    pub const NEUMANN_CONDITION: u8 = 16;
    pub const ROBIN_CONDITION: u8 = 17;
    pub const PERIODIC_CONDITION: u8 = 18;
}

/// PDE educational messages
pub static PDE_MESSAGES: Lazy<HashMap<MessageKey, &'static str>> = Lazy::new(|| {
    let mut messages = HashMap::new();

    // General PDE concepts
    messages.insert(
        pde_message_key(MessageType::Introduction, PdeMessageVariant::WHAT_IS_PDE),
        "A **Partial Differential Equation (PDE)** is an equation that relates a function of several variables to its partial derivatives. \
        PDEs describe phenomena involving multiple independent variables, such as heat distribution over time and space, \
        wave propagation, or electromagnetic fields. The solution to a PDE is a function (or family of functions) \
        that satisfies the equation and given boundary/initial conditions."
    );

    messages.insert(
        pde_message_key(MessageType::Introduction, PdeMessageVariant::PDE_VS_ODE),
        "**PDEs vs ODEs**: While Ordinary Differential Equations (ODEs) involve functions of a single variable and their derivatives, \
        PDEs involve functions of multiple variables and their partial derivatives. For example, y'(t) = f(t,y) is an ODE \
        (one variable t), while ∂u/∂t = α∂²u/∂x² is a PDE (two variables t and x). PDEs are generally more complex to solve \
        and often require specialized techniques based on their type and boundary conditions."
    );

    messages.insert(
        pde_message_key(MessageType::Strategy, PdeMessageVariant::PDE_CLASSIFICATION),
        "**PDE Classification**: Second-order linear PDEs are classified based on the discriminant B² - 4AC: \
        **Elliptic** (B² - 4AC < 0): Steady-state problems like Laplace's equation. \
        **Parabolic** (B² - 4AC = 0): Diffusion processes like the heat equation. \
        **Hyperbolic** (B² - 4AC > 0): Wave propagation like the wave equation. \
        This classification determines solution behavior and appropriate numerical methods."
    );

    // PDE types
    messages.insert(
        pde_message_key(MessageType::Introduction, PdeMessageVariant::ELLIPTIC_EQUATION),
        "**Elliptic PDEs** describe steady-state phenomena where time is not a factor. The Laplace equation ∇²u = 0 \
        is the prototype, modeling equilibrium states in physics. Solutions are smooth and determined entirely by \
        boundary conditions. Examples include electrostatic potential, steady heat distribution, and incompressible \
        fluid flow. Numerical methods like finite elements work well for elliptic problems."
    );

    messages.insert(
        pde_message_key(MessageType::Introduction, PdeMessageVariant::PARABOLIC_EQUATION),
        "**Parabolic PDEs** model diffusion and dissipative processes that evolve toward equilibrium. The heat equation \
        ∂u/∂t = α∇²u is the classic example, describing how temperature spreads through a material. Solutions smooth out \
        discontinuities over time and exhibit infinite speed of propagation (disturbances affect the entire domain instantly, \
        though with exponentially decreasing magnitude). Require initial conditions and boundary conditions."
    );

    messages.insert(
        pde_message_key(MessageType::Introduction, PdeMessageVariant::HYPERBOLIC_EQUATION),
        "**Hyperbolic PDEs** describe wave propagation and vibrations with finite speed. The wave equation ∂²u/∂t² = c²∇²u \
        is the prototype, modeling sound waves, electromagnetic waves, and vibrating strings. Solutions preserve discontinuities \
        along characteristics (paths of information propagation). D'Alembert's solution shows waves traveling at speed c. \
        Require initial position and velocity, plus boundary conditions."
    );

    // Standard PDEs
    messages.insert(
        pde_message_key(MessageType::Introduction, PdeMessageVariant::HEAT_EQUATION),
        "The **Heat Equation** ∂u/∂t = α∇²u models heat diffusion in materials. Here u(x,t) is temperature, \
        α is thermal diffusivity. Heat flows from hot to cold regions, smoothing out temperature differences. \
        Solutions can be found using separation of variables: u(x,t) = X(x)T(t), leading to Fourier series. \
        The fundamental solution (Green's function) is a Gaussian that spreads over time, showing how point \
        sources of heat diffuse."
    );

    messages.insert(
        pde_message_key(MessageType::Introduction, PdeMessageVariant::WAVE_EQUATION),
        "The **Wave Equation** ∂²u/∂t² = c²∇²u describes wave propagation at speed c. Solutions include \
        traveling waves u = f(x - ct) + g(x + ct) (d'Alembert's formula in 1D). Energy is conserved and \
        waves maintain their shape while traveling. Separation of variables gives standing wave solutions \
        u(x,t) = sin(nπx/L)cos(nπct/L), representing harmonics of a vibrating string. The wave equation \
        appears in acoustics, electromagnetics, and quantum mechanics."
    );

    messages.insert(
        pde_message_key(MessageType::Introduction, PdeMessageVariant::LAPLACE_EQUATION),
        "**Laplace's Equation** ∇²u = 0 describes equilibrium states in physics. Solutions (harmonic functions) \
        have remarkable properties: they satisfy the maximum principle (extrema occur on boundaries), \
        mean value property (value at a point equals average over any surrounding sphere), and are \
        infinitely differentiable. Applications include electrostatics (potential), steady heat flow, \
        incompressible fluid flow, and minimal surfaces. Solved using separation of variables, conformal \
        mapping, or Green's functions."
    );

    messages.insert(
        pde_message_key(MessageType::Introduction, PdeMessageVariant::POISSON_EQUATION),
        "**Poisson's Equation** ∇²u = f is the inhomogeneous version of Laplace's equation, where f \
        represents sources or sinks. In electrostatics, f is charge density and u is potential. \
        Solutions combine the particular solution (accounting for sources) with homogeneous solutions \
        (satisfying boundary conditions). Green's functions provide explicit integral representations. \
        The equation appears in gravity (f is mass density), electromagnetism, and fluid mechanics."
    );

    // Solution methods
    messages.insert(
        pde_message_key(MessageType::Strategy, PdeMessageVariant::SEPARATION_OF_VARIABLES),
        "**Separation of Variables** assumes the solution can be written as a product of single-variable functions: \
        u(x,t) = X(x)T(t). Substituting into the PDE and dividing yields separate ODEs for each function. \
        This works when the PDE and boundary conditions are separable. The method produces eigenvalue problems \
        whose solutions form a complete basis (often trigonometric or special functions). The general solution \
        is a superposition (Fourier series) of these eigenfunctions."
    );

    messages.insert(
        pde_message_key(MessageType::Strategy, PdeMessageVariant::METHOD_OF_CHARACTERISTICS),
        "The **Method of Characteristics** solves first-order PDEs by finding curves (characteristics) along which \
        the PDE becomes an ODE. For the equation a∂u/∂x + b∂u/∂y = c, characteristics satisfy dx/a = dy/b = du/c. \
        The solution is constant along characteristics for homogeneous equations. This method extends to systems \
        and higher-order hyperbolic equations, revealing how information propagates through the domain. \
        Wave fronts and shock waves follow characteristics."
    );

    messages.insert(
        pde_message_key(MessageType::Calculation, PdeMessageVariant::FOURIER_SERIES),
        "**Fourier Series** represent periodic functions as infinite sums of sines and cosines. In PDE solutions, \
        they arise naturally from separation of variables with periodic boundary conditions. For a function on [0,L], \
        f(x) = a₀/2 + Σ(aₙcos(nπx/L) + bₙsin(nπx/L)). Coefficients are found by orthogonality: \
        aₙ = (2/L)∫f(x)cos(nπx/L)dx. Fourier series converge to the function (in L² sense) and provide \
        spectral decomposition, showing which frequencies are present."
    );

    messages.insert(
        pde_message_key(MessageType::Strategy, PdeMessageVariant::GREENS_FUNCTIONS),
        "**Green's Functions** G(x,x';t,t') represent the response at (x,t) to a unit impulse at (x',t'). \
        They convert PDEs into integral equations: u(x,t) = ∫G(x,x';t,0)f(x')dx' for initial value f. \
        Green's functions satisfy the PDE with a delta function source and appropriate boundary conditions. \
        They embody the superposition principle: the solution for any source is the integral of point source \
        solutions. Finding Green's functions is often difficult but provides complete solution formulas."
    );

    // Boundary conditions
    messages.insert(
        pde_message_key(MessageType::Step, PdeMessageVariant::DIRICHLET_CONDITION),
        "**Dirichlet Boundary Conditions** specify the value of the solution on the boundary: u|∂Ω = g. \
        Physically, this fixes temperature (heat equation), displacement (wave equation), or potential \
        (Laplace equation) at boundaries. For uniqueness, Dirichlet conditions completely determine \
        elliptic and parabolic solutions. In separation of variables, they determine the eigenvalues \
        and eigenfunctions. Example: u(0,t) = 0, u(L,t) = 0 for a string fixed at both ends."
    );

    messages.insert(
        pde_message_key(MessageType::Step, PdeMessageVariant::NEUMANN_CONDITION),
        "**Neumann Boundary Conditions** specify the normal derivative on the boundary: ∂u/∂n|∂Ω = h. \
        This prescribes flux: heat flow (heat equation), velocity (wave equation), or electric field \
        (Laplace equation). Pure Neumann problems for Laplace's equation are only solvable if ∫h = 0 \
        (conservation). Solutions are unique up to a constant. Example: ∂u/∂x(0,t) = 0 represents \
        an insulated boundary (no heat flow)."
    );

    messages.insert(
        pde_message_key(MessageType::Step, PdeMessageVariant::ROBIN_CONDITION),
        "**Robin Boundary Conditions** (mixed/third type) combine Dirichlet and Neumann: αu + β∂u/∂n = γ. \
        They model realistic boundaries like convective heat transfer: -k∂u/∂n = h(u - u∞), where heat \
        flux is proportional to temperature difference. Robin conditions often arise from coupling PDEs \
        across interfaces. They ensure unique solutions for elliptic problems when α and β have appropriate \
        signs. Eigenvalue problems with Robin conditions have discrete spectra."
    );

    messages.insert(
        pde_message_key(MessageType::Step, PdeMessageVariant::PERIODIC_CONDITION),
        "**Periodic Boundary Conditions** require u(x + L) = u(x) and ∂u/∂x(x + L) = ∂u/∂x(x), \
        making the solution periodic with period L. They model systems on circles, tori, or with \
        translational symmetry. Eigenfunctions are complex exponentials e^(2πinx/L) or sines/cosines. \
        Periodic conditions lead naturally to Fourier series representations. Applications include \
        crystal lattices, circular membranes, and periodic wave guides."
    );

    messages
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pde_messages_loaded() {
        assert!(!PDE_MESSAGES.is_empty());

        // Test some specific messages exist
        let heat_key = pde_message_key(MessageType::Introduction, PdeMessageVariant::HEAT_EQUATION);
        assert!(PDE_MESSAGES.contains_key(&heat_key));

        let wave_key = pde_message_key(MessageType::Introduction, PdeMessageVariant::WAVE_EQUATION);
        assert!(PDE_MESSAGES.contains_key(&wave_key));
    }

    #[test]
    fn test_pde_message_content() {
        let key = pde_message_key(MessageType::Introduction, PdeMessageVariant::HEAT_EQUATION);
        let message = PDE_MESSAGES.get(&key).unwrap();
        assert!(message.contains("Heat Equation"));
        assert!(message.contains("∂u/∂t = α∇²u"));
    }

    #[test]
    fn test_all_pde_categories() {
        let intro_key = pde_message_key(MessageType::Introduction, PdeMessageVariant::WHAT_IS_PDE);
        let strategy_key = pde_message_key(
            MessageType::Strategy,
            PdeMessageVariant::SEPARATION_OF_VARIABLES,
        );
        let step_key = pde_message_key(MessageType::Step, PdeMessageVariant::DIRICHLET_CONDITION);
        let calc_key = pde_message_key(MessageType::Calculation, PdeMessageVariant::FOURIER_SERIES);

        assert!(PDE_MESSAGES.contains_key(&intro_key));
        assert!(PDE_MESSAGES.contains_key(&strategy_key));
        assert!(PDE_MESSAGES.contains_key(&step_key));
        assert!(PDE_MESSAGES.contains_key(&calc_key));
    }
}

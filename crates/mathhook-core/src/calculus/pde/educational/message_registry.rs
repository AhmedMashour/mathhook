//! PDE-specific educational messages
//!
//! This module provides educational messages for partial differential equation concepts,
//! following the pattern established in the ODE module.

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// PDE-specific message keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PdeMessageKey {
    // General PDE concepts
    WhatIsPde,
    PdeVsOde,
    PdeClassification,

    // PDE types
    EllipticEquation,
    ParabolicEquation,
    HyperbolicEquation,

    // Standard PDEs
    HeatEquation,
    WaveEquation,
    LaplaceEquation,
    PoissonEquation,

    // Solution methods
    SeparationOfVariables,
    MethodOfCharacteristics,
    FourierSeries,
    GreensFunctions,

    // Boundary conditions
    DirichletCondition,
    NeumannCondition,
    RobinCondition,
    PeriodicCondition,

    // Initial conditions
    InitialValueProblem,
    InitialBoundaryValueProblem,

    // Numerical methods
    FiniteDifference,
    FiniteElement,
    SpectralMethods,
}

/// Global PDE message registry
pub static PDE_MESSAGE_REGISTRY: Lazy<HashMap<PdeMessageKey, &'static str>> = Lazy::new(|| {
    let mut messages = HashMap::new();

    // General PDE concepts
    messages.insert(
        PdeMessageKey::WhatIsPde,
        "A **Partial Differential Equation (PDE)** is an equation that relates a function of several variables to its partial derivatives. \
        PDEs describe phenomena involving multiple independent variables, such as heat distribution over time and space, \
        wave propagation, or electromagnetic fields. The solution to a PDE is a function (or family of functions) \
        that satisfies the equation and given boundary/initial conditions."
    );

    messages.insert(
        PdeMessageKey::PdeVsOde,
        "**PDEs vs ODEs**: While Ordinary Differential Equations (ODEs) involve functions of a single variable and their derivatives, \
        PDEs involve functions of multiple variables and their partial derivatives. For example, y'(t) = f(t,y) is an ODE \
        (one variable t), while ∂u/∂t = α∂²u/∂x² is a PDE (two variables t and x). PDEs are generally more complex to solve \
        and often require specialized techniques based on their type and boundary conditions."
    );

    messages.insert(
        PdeMessageKey::PdeClassification,
        "**PDE Classification**: Second-order linear PDEs are classified based on the discriminant B² - 4AC: \
        **Elliptic** (B² - 4AC < 0): Steady-state problems like Laplace's equation. \
        **Parabolic** (B² - 4AC = 0): Diffusion processes like the heat equation. \
        **Hyperbolic** (B² - 4AC > 0): Wave propagation like the wave equation. \
        This classification determines solution behavior and appropriate numerical methods."
    );

    // PDE types
    messages.insert(
        PdeMessageKey::EllipticEquation,
        "**Elliptic PDEs** describe steady-state phenomena where time is not a factor. The Laplace equation ∇²u = 0 \
        is the prototype, modeling equilibrium states in physics. Solutions are smooth and determined entirely by \
        boundary conditions. Examples include electrostatic potential, steady heat distribution, and incompressible \
        fluid flow. Numerical methods like finite elements work well for elliptic problems."
    );

    messages.insert(
        PdeMessageKey::ParabolicEquation,
        "**Parabolic PDEs** model diffusion and dissipative processes that evolve toward equilibrium. The heat equation \
        ∂u/∂t = α∇²u is the classic example, describing how temperature spreads through a material. Solutions smooth out \
        discontinuities over time and exhibit infinite speed of propagation (disturbances affect the entire domain instantly, \
        though with exponentially decreasing magnitude). Require initial conditions and boundary conditions."
    );

    messages.insert(
        PdeMessageKey::HyperbolicEquation,
        "**Hyperbolic PDEs** describe wave propagation and vibrations with finite speed. The wave equation ∂²u/∂t² = c²∇²u \
        is the prototype, modeling sound waves, electromagnetic waves, and vibrating strings. Solutions preserve discontinuities \
        along characteristics (paths of information propagation). D'Alembert's solution shows waves traveling at speed c. \
        Require initial position and velocity, plus boundary conditions."
    );

    // Standard PDEs
    messages.insert(
        PdeMessageKey::HeatEquation,
        "The **Heat Equation** ∂u/∂t = α∇²u models heat diffusion in materials. Here u(x,t) is temperature, \
        α is thermal diffusivity. Heat flows from hot to cold regions, smoothing out temperature differences. \
        Solutions can be found using separation of variables: u(x,t) = X(x)T(t), leading to Fourier series. \
        The fundamental solution (Green's function) is a Gaussian that spreads over time, showing how point \
        sources of heat diffuse."
    );

    messages.insert(
        PdeMessageKey::WaveEquation,
        "The **Wave Equation** ∂²u/∂t² = c²∇²u describes wave propagation at speed c. Solutions include \
        traveling waves u = f(x - ct) + g(x + ct) (d'Alembert's formula in 1D). Energy is conserved and \
        waves maintain their shape while traveling. Separation of variables gives standing wave solutions \
        u(x,t) = sin(nπx/L)cos(nπct/L), representing harmonics of a vibrating string. The wave equation \
        appears in acoustics, electromagnetics, and quantum mechanics."
    );

    messages.insert(
        PdeMessageKey::LaplaceEquation,
        "**Laplace's Equation** ∇²u = 0 describes equilibrium states in physics. Solutions (harmonic functions) \
        have remarkable properties: they satisfy the maximum principle (extrema occur on boundaries), \
        mean value property (value at a point equals average over any surrounding sphere), and are \
        infinitely differentiable. Applications include electrostatics (potential), steady heat flow, \
        incompressible fluid flow, and minimal surfaces. Solved using separation of variables, conformal \
        mapping, or Green's functions."
    );

    messages.insert(
        PdeMessageKey::PoissonEquation,
        "**Poisson's Equation** ∇²u = f is the inhomogeneous version of Laplace's equation, where f \
        represents sources or sinks. In electrostatics, f is charge density and u is potential. \
        Solutions combine the particular solution (accounting for sources) with homogeneous solutions \
        (satisfying boundary conditions). Green's functions provide explicit integral representations. \
        The equation appears in gravity (f is mass density), electromagnetism, and fluid mechanics."
    );

    // Solution methods
    messages.insert(
        PdeMessageKey::SeparationOfVariables,
        "**Separation of Variables** assumes the solution can be written as a product of single-variable functions: \
        u(x,t) = X(x)T(t). Substituting into the PDE and dividing yields separate ODEs for each function. \
        This works when the PDE and boundary conditions are separable. The method produces eigenvalue problems \
        whose solutions form a complete basis (often trigonometric or special functions). The general solution \
        is a superposition (Fourier series) of these eigenfunctions."
    );

    messages.insert(
        PdeMessageKey::MethodOfCharacteristics,
        "The **Method of Characteristics** solves first-order PDEs by finding curves (characteristics) along which \
        the PDE becomes an ODE. For the equation a∂u/∂x + b∂u/∂y = c, characteristics satisfy dx/a = dy/b = du/c. \
        The solution is constant along characteristics for homogeneous equations. This method extends to systems \
        and higher-order hyperbolic equations, revealing how information propagates through the domain. \
        Wave fronts and shock waves follow characteristics."
    );

    messages.insert(
        PdeMessageKey::FourierSeries,
        "**Fourier Series** represent periodic functions as infinite sums of sines and cosines. In PDE solutions, \
        they arise naturally from separation of variables with periodic boundary conditions. For a function on [0,L], \
        f(x) = a₀/2 + Σ(aₙcos(nπx/L) + bₙsin(nπx/L)). Coefficients are found by orthogonality: \
        aₙ = (2/L)∫f(x)cos(nπx/L)dx. Fourier series converge to the function (in L² sense) and provide \
        spectral decomposition, showing which frequencies are present."
    );

    messages.insert(
        PdeMessageKey::GreensFunctions,
        "**Green's Functions** G(x,x';t,t') represent the response at (x,t) to a unit impulse at (x',t'). \
        They convert PDEs into integral equations: u(x,t) = ∫G(x,x';t,0)f(x')dx' for initial value f. \
        Green's functions satisfy the PDE with a delta function source and appropriate boundary conditions. \
        They embody the superposition principle: the solution for any source is the integral of point source \
        solutions. Finding Green's functions is often difficult but provides complete solution formulas."
    );

    // Boundary conditions
    messages.insert(
        PdeMessageKey::DirichletCondition,
        "**Dirichlet Boundary Conditions** specify the value of the solution on the boundary: u|∂Ω = g. \
        Physically, this fixes temperature (heat equation), displacement (wave equation), or potential \
        (Laplace equation) at boundaries. For uniqueness, Dirichlet conditions completely determine \
        elliptic and parabolic solutions. In separation of variables, they determine the eigenvalues \
        and eigenfunctions. Example: u(0,t) = 0, u(L,t) = 0 for a string fixed at both ends."
    );

    messages.insert(
        PdeMessageKey::NeumannCondition,
        "**Neumann Boundary Conditions** specify the normal derivative on the boundary: ∂u/∂n|∂Ω = h. \
        This prescribes flux: heat flow (heat equation), velocity (wave equation), or electric field \
        (Laplace equation). Pure Neumann problems for Laplace's equation are only solvable if ∫h = 0 \
        (conservation). Solutions are unique up to a constant. Example: ∂u/∂x(0,t) = 0 represents \
        an insulated boundary (no heat flow)."
    );

    messages.insert(
        PdeMessageKey::RobinCondition,
        "**Robin Boundary Conditions** (mixed/third type) combine Dirichlet and Neumann: αu + β∂u/∂n = γ. \
        They model realistic boundaries like convective heat transfer: -k∂u/∂n = h(u - u∞), where heat \
        flux is proportional to temperature difference. Robin conditions often arise from coupling PDEs \
        across interfaces. They ensure unique solutions for elliptic problems when α and β have appropriate \
        signs. Eigenvalue problems with Robin conditions have discrete spectra."
    );

    messages.insert(
        PdeMessageKey::PeriodicCondition,
        "**Periodic Boundary Conditions** require u(x + L) = u(x) and ∂u/∂x(x + L) = ∂u/∂x(x), \
        making the solution periodic with period L. They model systems on circles, tori, or with \
        translational symmetry. Eigenfunctions are complex exponentials e^(2πinx/L) or sines/cosines. \
        Periodic conditions lead naturally to Fourier series representations. Applications include \
        crystal lattices, circular membranes, and periodic wave guides."
    );

    // Initial conditions
    messages.insert(
        PdeMessageKey::InitialValueProblem,
        "An **Initial Value Problem (IVP)** specifies the solution and possibly its time derivatives at t = 0. \
        For parabolic equations (heat), only u(x,0) = f(x) is needed. For hyperbolic equations (wave), \
        both position u(x,0) = f(x) and velocity ∂u/∂t(x,0) = g(x) are required. The initial data \
        determines the future evolution uniquely (for well-posed problems). Smoothness of initial data \
        affects solution regularity: discontinuous data may remain discontinuous (hyperbolic) or smooth \
        out (parabolic)."
    );

    messages.insert(
        PdeMessageKey::InitialBoundaryValueProblem,
        "An **Initial-Boundary Value Problem (IBVP)** combines initial conditions with boundary conditions, \
        typical for evolution equations on bounded domains. The heat equation on [0,L] × [0,∞) needs: \
        initial temperature u(x,0) = f(x), and boundary conditions like u(0,t) = u(L,t) = 0. \
        Well-posedness requires compatibility: initial and boundary data must agree at corners (x,t) = (0,0), (L,0). \
        Solutions are often found by separation of variables, yielding Fourier series representations."
    );

    // Numerical methods
    messages.insert(
        PdeMessageKey::FiniteDifference,
        "**Finite Difference Methods** approximate derivatives using nearby function values: \
        ∂u/∂x ≈ (u(x+h) - u(x-h))/(2h). PDEs become systems of algebraic equations on a grid. \
        Explicit methods (forward Euler) are simple but may require small time steps for stability. \
        Implicit methods (backward Euler, Crank-Nicolson) are stable but require solving linear systems. \
        Accuracy depends on grid spacing (typically O(h²)). Boundary conditions are incorporated through \
        ghost points or modified stencils."
    );

    messages.insert(
        PdeMessageKey::FiniteElement,
        "**Finite Element Methods (FEM)** approximate solutions as linear combinations of basis functions \
        (often piecewise polynomials on triangular/tetrahedral meshes). The PDE is reformulated weakly: \
        multiply by test functions and integrate by parts. This naturally handles complex geometries and \
        boundary conditions. Galerkin method uses same basis for trial and test functions. FEM provides \
        rigorous error estimates and adaptive refinement. It's the standard for elliptic problems in \
        engineering (structural mechanics, electromagnetics)."
    );

    messages.insert(
        PdeMessageKey::SpectralMethods,
        "**Spectral Methods** represent solutions using global basis functions (Fourier series, Chebyshev \
        polynomials). They achieve exponential convergence for smooth solutions (spectral accuracy). \
        Derivatives are computed in spectral space (multiplication by ik for Fourier). Nonlinear terms \
        use transforms: FFT to physical space, multiply, FFT back. Spectral methods excel for periodic \
        domains and smooth solutions but struggle with discontinuities (Gibbs phenomenon). They're used \
        in turbulence simulation and numerical weather prediction."
    );

    messages
});

/// Get a PDE educational message by key
pub fn get_pde_message(key: PdeMessageKey) -> Option<&'static str> {
    PDE_MESSAGE_REGISTRY.get(&key).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pde_message_registry() {
        let test_keys = vec![
            PdeMessageKey::WhatIsPde,
            PdeMessageKey::HeatEquation,
            PdeMessageKey::WaveEquation,
            PdeMessageKey::LaplaceEquation,
            PdeMessageKey::SeparationOfVariables,
            PdeMessageKey::DirichletCondition,
            PdeMessageKey::FiniteDifference,
        ];

        for key in test_keys {
            let message = get_pde_message(key);
            assert!(message.is_some(), "Missing message for key {:?}", key);
            assert!(
                !message.unwrap().is_empty(),
                "Empty message for key {:?}",
                key
            );
        }
    }

    #[test]
    fn test_message_content_quality() {
        let message = get_pde_message(PdeMessageKey::HeatEquation).unwrap();
        assert!(message.len() > 100, "Message too short");
        assert!(message.contains("Heat Equation"), "Missing key term");
        assert!(message.contains("∂u/∂t"), "Missing equation");
    }

    #[test]
    fn test_all_messages_present() {
        assert_eq!(PDE_MESSAGE_REGISTRY.len(), 23, "Expected 23 PDE messages");
    }
}

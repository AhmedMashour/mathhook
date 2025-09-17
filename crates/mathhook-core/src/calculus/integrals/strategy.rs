//! Integration strategy dispatcher
//!
//! Orchestrates all integration techniques in optimal order (fast to slow).
//!
//! # Strategy Layers
//!
//! 1. **Table lookup** - O(1) exact pattern matching for common integrals
//! 2. **Rational functions** - Partial fraction decomposition for P(x)/Q(x)
//! 3. **Function registry** - Known antiderivatives (sin, cos, exp, ln, etc.)
//! 4. **Integration by parts** - Product rule in reverse using LIATE heuristic
//! 5. **Substitution** - Chain rule in reverse (u-substitution)
//! 6. **Trigonometric** - Trig identities and power reduction formulas
//! 7. **Risch algorithm** - Decision procedure for elementary functions
//! 8. **Basic rules** - Power rule, constants, sums, constant multiples
//! 9. **Symbolic fallback** - Return unevaluated integral expression
//!
//! # Strategy Tracking
//!
//! To prevent infinite recursion, we track which strategies are currently active
//! in the call stack. A strategy cannot recursively call itself.
//!
//! # Recursion Depth Limit
//!
//! Maximum integration depth is 10 to prevent infinite recursion in pathological cases.
use crate::calculus::integrals::{
    basic::BasicIntegrals, by_parts::IntegrationByParts, function_integrals::FunctionIntegrals,
    rational, risch, substitution, table, trigonometric,
};
use crate::core::{Expression, Number, Symbol};
use std::collections::HashSet;
/// Maximum integration recursion depth
///
/// This prevents infinite recursion in cases where integration strategies
/// recursively call each other without terminating.
const MAX_DEPTH: usize = 10;
/// Integration strategy identifier
///
/// Each integration technique is assigned a unique identifier to track
/// which strategies are currently active in the call stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegrationStrategy {
    TableLookup,
    RationalFunction,
    FunctionRegistry,
    IntegrationByParts,
    Substitution,
    Trigonometric,
    Risch,
    BasicRules,
}
/// Strategy execution context
///
/// Tracks which integration strategies are currently active to prevent
/// infinite recursion and improve strategy selection.
#[derive(Debug, Clone)]
pub struct StrategyContext {
    active_strategies: HashSet<IntegrationStrategy>,
    depth: usize,
}
impl StrategyContext {
    /// Create a new strategy context with no active strategies
    pub fn new() -> Self {
        Self {
            active_strategies: HashSet::new(),
            depth: 0,
        }
    }
    /// Check if a strategy is currently active (would cause recursion)
    pub fn is_active(&self, strategy: IntegrationStrategy) -> bool {
        self.active_strategies.contains(&strategy)
    }
    /// Execute a strategy, marking it as active during execution
    pub fn with_strategy<F>(&self, strategy: IntegrationStrategy, f: F) -> Option<Expression>
    where
        F: FnOnce(&Self) -> Option<Expression>,
    {
        if self.is_active(strategy) {
            return None;
        }
        let mut child_context = self.clone();
        child_context.active_strategies.insert(strategy);
        child_context.depth += 1;
        f(&child_context)
    }
    /// Get current recursion depth
    pub fn depth(&self) -> usize {
        self.depth
    }
}
impl Default for StrategyContext {
    fn default() -> Self {
        Self::new()
    }
}
/// Main integration strategy dispatcher
///
/// Tries strategies in order from fast to slow, returning first success.
///
/// # Recursion Depth Limit
///
/// Returns symbolic integral if depth >= MAX_DEPTH (10) to prevent infinite recursion.
pub fn integrate_with_strategy(expr: &Expression, var: Symbol, depth: usize) -> Expression {
    if depth >= MAX_DEPTH {
        return Expression::integral(expr.clone(), var);
    }
    let context = StrategyContext {
        active_strategies: HashSet::new(),
        depth,
    };
    integrate_with_context(expr, var, &context)
}
/// Integration with explicit strategy context
///
/// Used by recursive calls to track which strategies are currently active.
fn integrate_with_context(expr: &Expression, var: Symbol, ctx: &StrategyContext) -> Expression {
    if let Some(result) = try_table_lookup_with_context(expr, &var, ctx) {
        return result;
    }
    if is_rational_function(expr, &var) {
        if let Some(result) = try_rational_function(expr, &var) {
            return result;
        }
    }
    if let Some(result) = try_registry_integration_with_context(expr, &var, ctx) {
        return result;
    }
    if let Some(result) = ctx.with_strategy(IntegrationStrategy::IntegrationByParts, |child_ctx| {
        try_by_parts_with_context(expr, &var, child_ctx, child_ctx.depth())
    }) {
        return result;
    }
    if let Some(result) = ctx.with_strategy(IntegrationStrategy::Substitution, |child_ctx| {
        try_substitution_with_context(expr, &var, child_ctx)
    }) {
        return result;
    }
    if let Some(result) = ctx.with_strategy(IntegrationStrategy::Trigonometric, |child_ctx| {
        try_trigonometric_with_context(expr, &var, child_ctx)
    }) {
        return result;
    }
    if let Some(result) = ctx.with_strategy(IntegrationStrategy::Risch, |child_ctx| {
        try_risch_with_context(expr, &var, child_ctx)
    }) {
        return result;
    }
    if let Some(result) = try_basic_rules_with_context(expr, &var, ctx) {
        return result;
    }
    Expression::integral(expr.clone(), var)
}
/// Table lookup with strategy context
fn try_table_lookup_with_context(
    expr: &Expression,
    var: &Symbol,
    _context: &StrategyContext,
) -> Option<Expression> {
    table::try_table_lookup(expr, var)
}
/// Rational function integration with strategy context
fn try_rational_function(expr: &Expression, var: &Symbol) -> Option<Expression> {
    rational::integrate_rational(expr, var)
}
/// Try function registry integration using known antiderivatives
///
/// Uses the function intelligence registry for elementary functions.
pub fn try_registry_integration(expr: &Expression, var: &Symbol) -> Option<Expression> {
    match expr {
        Expression::Function { name, args } => {
            let result = FunctionIntegrals::integrate(name, args, var.clone());
            if is_symbolic_integral(&result) {
                None
            } else {
                Some(result)
            }
        }
        _ => None,
    }
}
/// Function registry integration with strategy context
fn try_registry_integration_with_context(
    expr: &Expression,
    var: &Symbol,
    _context: &StrategyContext,
) -> Option<Expression> {
    try_registry_integration(expr, var)
}
/// Try integration by parts using the LIATE heuristic
///
/// Applies the product rule in reverse for expressions like x*exp(x).
pub fn try_by_parts(expr: &Expression, var: &Symbol, depth: usize) -> Option<Expression> {
    IntegrationByParts::integrate(expr, var.clone(), depth)
}
/// Integration by parts with strategy context
///
/// This prevents recursive application of integration by parts.
fn try_by_parts_with_context(
    expr: &Expression,
    var: &Symbol,
    context: &StrategyContext,
    depth: usize,
) -> Option<Expression> {
    IntegrationByParts::integrate_with_context(expr, var.clone(), context, depth)
}
/// U-substitution with strategy context
fn try_substitution_with_context(
    expr: &Expression,
    var: &Symbol,
    context: &StrategyContext,
) -> Option<Expression> {
    substitution::try_substitution(expr, var, context.depth())
}
/// Trigonometric integration with strategy context
fn try_trigonometric_with_context(
    expr: &Expression,
    var: &Symbol,
    _context: &StrategyContext,
) -> Option<Expression> {
    trigonometric::try_trigonometric_integration(expr, var)
}
/// Risch algorithm with strategy context
fn try_risch_with_context(
    expr: &Expression,
    var: &Symbol,
    _context: &StrategyContext,
) -> Option<Expression> {
    risch::try_risch_integration(expr, var)
}
/// Check if expression is a polynomial in the given variable
///
/// Polynomial: only var, constants, +, *, and non-negative integer powers.
pub fn is_polynomial(expr: &Expression, _var: &Symbol) -> bool {
    match expr {
        Expression::Number(_) | Expression::Constant(_) => true,
        Expression::Symbol(_sym) => true,
        Expression::Add(terms) => terms.iter().all(|t| is_polynomial(t, _var)),
        Expression::Mul(factors) => factors.iter().all(|f| is_polynomial(f, _var)),
        Expression::Pow(base, exp) => {
            if !is_polynomial(base, _var) {
                return false;
            }
            matches!(exp.as_ref(), Expression::Number(Number::Integer(n)) if * n >= 0)
        }
        _ => false,
    }
}
/// Check if expression is a rational function P(x)/Q(x)
///
/// Rational: ratio of two polynomials
fn is_rational_function(expr: &Expression, var: &Symbol) -> bool {
    let result = match expr {
        _ if is_polynomial(expr, var) => true,
        Expression::Pow(base, exp) => {
            if let Expression::Number(Number::Integer(_)) = exp.as_ref() {
                let poly_check = is_polynomial(base.as_ref(), var);
                poly_check
            } else {
                false
            }
        }
        Expression::Mul(factors) => {
            for factor in factors.iter() {
                if let Expression::Pow(base, exp) = factor {
                    if let Expression::Number(Number::Integer(n)) = exp.as_ref() {
                        if *n < 0 && !is_polynomial(base.as_ref(), var) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
            factors.iter().all(|f| match f {
                Expression::Pow(base, _) => is_polynomial(base.as_ref(), var),
                _ => is_polynomial(f, var),
            })
        }
        _ => false,
    };
    result
}
/// Basic integration rules with strategy context
///
/// CRITICAL: Calls helper functions that recursively call integrate_with_strategy.
/// Depth limit at entry point prevents infinite recursion.
fn try_basic_rules_with_context(
    expr: &Expression,
    var: &Symbol,
    context: &StrategyContext,
) -> Option<Expression> {
    match expr {
        Expression::Number(_) => Some(BasicIntegrals::handle_constant(expr, var.clone())),
        Expression::Symbol(sym) => Some(BasicIntegrals::handle_symbol(sym, var)),
        Expression::Add(terms) => Some(BasicIntegrals::handle_sum(terms, var, context.depth())),
        Expression::Mul(factors) => Some(BasicIntegrals::handle_product(
            factors,
            var.clone(),
            context.depth(),
        )),
        Expression::Pow(base, exp) => Some(BasicIntegrals::handle_power(base, exp, var.clone())),
        Expression::Calculus(data) => {
            Some(BasicIntegrals::handle_calculus(expr, data, var.clone()))
        }
        _ => None,
    }
}
/// Check if result is a symbolic integral (unevaluated)
fn is_symbolic_integral(expr: &Expression) -> bool {
    matches!(expr, Expression::Calculus(_))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;
    #[test]
    fn test_is_polynomial_constant() {
        let x = symbol!(x);
        assert!(is_polynomial(&Expression::integer(5), &x));
    }
    #[test]
    fn test_is_polynomial_variable() {
        let x = symbol!(x);
        assert!(is_polynomial(&Expression::symbol(x.clone()), &x));
    }
    #[test]
    fn test_is_polynomial_sum() {
        let x = symbol!(x);
        let poly = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(x.clone()),
            Expression::integer(1),
        ]);
        assert!(is_polynomial(&poly, &x));
    }
    #[test]
    fn test_is_polynomial_product() {
        let x = symbol!(x);
        let poly = Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]);
        assert!(is_polynomial(&poly, &x));
    }
    #[test]
    fn test_is_not_polynomial_negative_power() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));
        assert!(!is_polynomial(&expr, &x));
    }
    #[test]
    fn test_is_not_polynomial_function() {
        let x = symbol!(x);
        let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        assert!(!is_polynomial(&expr, &x));
    }
}

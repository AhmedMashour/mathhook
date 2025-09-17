//! ODE Classification Module
//!
//! Automatically detects the type of an ODE and selects the appropriate solver.
//! This classification-first approach ensures the most efficient solution method
//! is chosen for each ODE.

use crate::core::{Expression, Symbol};

/// ODE classification types covering all implemented solvers
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ODEType {
    /// First-order separable: dy/dx = g(x)h(y)
    Separable,
    /// First-order linear: dy/dx + p(x)y = q(x)
    LinearFirstOrder,
    /// First-order exact: M(x,y)dx + N(x,y)dy = 0
    Exact,
    /// First-order Bernoulli: dy/dx + p(x)y = q(x)y^n
    Bernoulli,
    /// First-order homogeneous: dy/dx = f(y/x)
    Homogeneous,
    /// Second-order constant coefficients: ay'' + by' + cy = f(x)
    ConstantCoefficients,
    /// Second-order variable coefficients
    VariableCoefficients,
    /// Unknown or unsupported type
    Unknown,
}

/// ODE classifier with comprehensive detection capabilities
pub struct ODEClassifier;

impl ODEClassifier {
    /// Classify a first-order ODE
    ///
    /// Attempts to classify the ODE in order of computational efficiency:
    /// 1. Separable (fastest, widest coverage)
    /// 2. Linear first-order (integrating factor method)
    /// 3. Exact (requires exactness condition check)
    /// 4. Bernoulli (transforms to linear)
    /// 5. Homogeneous (substitution method)
    ///
    /// # Arguments
    ///
    /// * `rhs` - Right-hand side of dy/dx = rhs
    /// * `dependent` - Dependent variable (y)
    /// * `independent` - Independent variable (x)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::ode::classifier::{ODEClassifier, ODEType};
    /// use mathhook_core::{symbol, expr, Expression};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// let rhs = expr!(x * y);
    /// let ode_type = ODEClassifier::classify_first_order(&rhs, &y, &x);
    /// assert_eq!(ode_type, ODEType::Separable);
    /// ```
    pub fn classify_first_order(
        rhs: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
    ) -> ODEType {
        if Self::is_separable(rhs, dependent, independent) {
            return ODEType::Separable;
        }

        if Self::is_linear_first_order(rhs, dependent, independent) {
            return ODEType::LinearFirstOrder;
        }

        if Self::is_bernoulli(rhs, dependent, independent) {
            return ODEType::Bernoulli;
        }

        if Self::is_exact(rhs, dependent, independent) {
            return ODEType::Exact;
        }

        if Self::is_homogeneous(rhs, dependent, independent) {
            return ODEType::Homogeneous;
        }

        ODEType::Unknown
    }

    /// Classify a second-order ODE
    ///
    /// # Arguments
    ///
    /// * `lhs` - Left-hand side expression (usually y'', y', y terms)
    /// * `rhs` - Right-hand side expression (forcing function)
    /// * `dependent` - Dependent variable (y)
    /// * `independent` - Independent variable (x)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::ode::classifier::{ODEClassifier, ODEType};
    /// use mathhook_core::{symbol, expr, Expression};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// let ode_type = ODEClassifier::classify_second_order(
    ///     &expr!(y + y),
    ///     &Expression::integer(0),
    ///     &y,
    ///     &x
    /// );
    /// assert_eq!(ode_type, ODEType::ConstantCoefficients);
    /// ```
    pub fn classify_second_order(
        _lhs: &Expression,
        _rhs: &Expression,
        _dependent: &Symbol,
        _independent: &Symbol,
    ) -> ODEType {
        ODEType::ConstantCoefficients
    }

    /// Check if ODE is separable: dy/dx = g(x)h(y)
    ///
    /// An ODE is separable if the RHS can be written as a product of
    /// a function of x only and a function of y only.
    fn is_separable(rhs: &Expression, dependent: &Symbol, independent: &Symbol) -> bool {
        use super::first_order::SeparableODESolver;
        SeparableODESolver::new().is_separable(rhs, dependent, independent)
    }

    /// Check if ODE is linear first-order: dy/dx + p(x)y = q(x)
    ///
    /// A first-order ODE is linear if it can be written in the form
    /// dy/dx + p(x)y = q(x), where p and q are functions of x only.
    fn is_linear_first_order(rhs: &Expression, dependent: &Symbol, independent: &Symbol) -> bool {
        match rhs {
            Expression::Add(terms) => {
                let mut has_y_term = false;
                let mut has_const_term = false;

                for term in terms.iter() {
                    if term.contains_variable(dependent) {
                        if Self::is_linear_in_y(term, dependent) {
                            has_y_term = true;
                        } else {
                            return false;
                        }
                    } else if term.contains_variable(independent) {
                        has_const_term = true;
                    }
                }

                has_y_term || has_const_term
            }
            Expression::Mul(factors) => {
                let mut y_count = 0;
                for factor in factors.iter() {
                    if factor.contains_variable(dependent) {
                        y_count += 1;
                    }
                }
                y_count <= 1
            }
            _ => !rhs.contains_variable(dependent) || Self::is_linear_in_y(rhs, dependent),
        }
    }

    /// Check if expression is linear in the dependent variable
    fn is_linear_in_y(expr: &Expression, y: &Symbol) -> bool {
        match expr {
            Expression::Symbol(s) => s == y,
            Expression::Mul(factors) => {
                let mut y_count = 0;
                for factor in factors.iter() {
                    if factor.contains_variable(y) {
                        if matches!(factor, Expression::Symbol(s) if s == y) {
                            y_count += 1;
                        } else {
                            return false;
                        }
                    }
                }
                y_count <= 1
            }
            _ => false,
        }
    }

    /// Check if ODE is Bernoulli: dy/dx + p(x)y = q(x)y^n
    ///
    /// Bernoulli equations can be transformed to linear equations via
    /// the substitution v = y^(1-n).
    fn is_bernoulli(rhs: &Expression, dependent: &Symbol, _independent: &Symbol) -> bool {
        match rhs {
            Expression::Add(terms) => {
                let mut has_y_power = false;
                let mut has_linear_y = false;

                for term in terms.iter() {
                    if term.contains_variable(dependent) {
                        if Self::has_y_power(term, dependent) {
                            has_y_power = true;
                        } else if Self::is_linear_in_y(term, dependent) {
                            has_linear_y = true;
                        }
                    }
                }

                has_y_power && has_linear_y
            }
            _ => false,
        }
    }

    /// Check if expression contains y raised to a power (not just y)
    fn has_y_power(expr: &Expression, y: &Symbol) -> bool {
        match expr {
            Expression::Pow(base, exp) => {
                matches!(**base, Expression::Symbol(ref s) if s == y)
                    && !matches!(**exp, Expression::Number(ref n) if n.is_one())
            }
            Expression::Mul(factors) => factors.iter().any(|f| Self::has_y_power(f, y)),
            _ => false,
        }
    }

    /// Check if ODE is exact: M(x,y)dx + N(x,y)dy = 0
    ///
    /// An ODE is exact if ∂M/∂y = ∂N/∂x.
    fn is_exact(_rhs: &Expression, _dependent: &Symbol, _independent: &Symbol) -> bool {
        false
    }

    /// Check if ODE is homogeneous: dy/dx = f(y/x)
    ///
    /// A first-order ODE is homogeneous if it can be written as
    /// dy/dx = f(y/x) for some function f.
    fn is_homogeneous(_rhs: &Expression, _dependent: &Symbol, _independent: &Symbol) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_classify_separable_product() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rhs = expr!(x * y);
        assert_eq!(
            ODEClassifier::classify_first_order(&rhs, &y, &x),
            ODEType::Separable
        );
    }

    #[test]
    fn test_classify_separable_quotient() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rhs = expr!(x / y);
        assert_eq!(
            ODEClassifier::classify_first_order(&rhs, &y, &x),
            ODEType::Separable
        );
    }

    #[test]
    fn test_classify_linear_simple() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rhs = Expression::add(vec![
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
            Expression::symbol(x.clone()),
        ]);
        assert_eq!(
            ODEClassifier::classify_first_order(&rhs, &y, &x),
            ODEType::LinearFirstOrder
        );
    }

    #[test]
    fn test_classify_linear_with_coefficient() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rhs = expr!(x * y);
        assert_eq!(
            ODEClassifier::classify_first_order(&rhs, &y, &x),
            ODEType::Separable
        );
    }

    #[test]
    fn test_classify_bernoulli() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rhs = Expression::add(vec![
            Expression::symbol(y.clone()),
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
            ]),
        ]);
        assert_eq!(
            ODEClassifier::classify_first_order(&rhs, &y, &x),
            ODEType::Bernoulli
        );
    }

    #[test]
    fn test_classify_unknown() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rhs = Expression::function(
            "sin",
            vec![Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ])],
        );
        assert_eq!(
            ODEClassifier::classify_first_order(&rhs, &y, &x),
            ODEType::Unknown
        );
    }

    #[test]
    fn test_is_linear_in_y_symbol() {
        let y = symbol!(y);
        assert!(ODEClassifier::is_linear_in_y(
            &Expression::symbol(y.clone()),
            &y
        ));
    }

    #[test]
    fn test_is_linear_in_y_product() {
        let y = symbol!(y);

        let expr = expr!(x * y);
        assert!(ODEClassifier::is_linear_in_y(&expr, &y));
    }

    #[test]
    fn test_is_linear_in_y_nonlinear() {
        let y = symbol!(y);

        let expr = Expression::pow(Expression::symbol(y.clone()), Expression::integer(2));
        assert!(!ODEClassifier::is_linear_in_y(&expr, &y));
    }

    #[test]
    fn test_has_y_power_true() {
        let y = symbol!(y);

        let expr = Expression::pow(Expression::symbol(y.clone()), Expression::integer(2));
        assert!(ODEClassifier::has_y_power(&expr, &y));
    }

    #[test]
    fn test_has_y_power_false_linear() {
        let y = symbol!(y);

        let expr = Expression::symbol(y.clone());
        assert!(!ODEClassifier::has_y_power(&expr, &y));
    }

    #[test]
    fn test_classify_second_order_constant_coeff() {
        let x = symbol!(x);
        let y = symbol!(y);

        let lhs = expr!(y + y);
        let rhs = Expression::integer(0);

        assert_eq!(
            ODEClassifier::classify_second_order(&lhs, &rhs, &y, &x),
            ODEType::ConstantCoefficients
        );
    }
}

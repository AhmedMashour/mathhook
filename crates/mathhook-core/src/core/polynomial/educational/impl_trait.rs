//! Trait implementation for PolynomialEducational
//!
//! This file provides a single implementation of the PolynomialEducational trait
//! for Expression, with methods split across division, gcd, and factorization modules.

use super::PolynomialEducational;
use crate::core::{Expression, Symbol};
use crate::educational::step_by_step::StepByStepExplanation;

impl PolynomialEducational for Expression {
    fn explain_poly_division(&self, divisor: &Expression, var: &Symbol) -> StepByStepExplanation {
        super::division::explain_poly_division_impl(self, divisor, var)
    }

    fn explain_poly_gcd(&self, other: &Expression) -> StepByStepExplanation {
        super::gcd::explain_poly_gcd_impl(self, other)
    }

    fn explain_poly_factorization(&self, var: &Symbol) -> StepByStepExplanation {
        super::factorization::explain_poly_factorization_impl(self, var)
    }
}

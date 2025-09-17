//! Display and Debug implementations for PolyZp

use crate::core::polynomial::finite_field::poly::PolyZp;
use std::fmt;

impl fmt::Debug for PolyZp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PolyZp({:?}, mod {})",
            self.coefficients(),
            self.modulus()
        )
    }
}

impl fmt::Display for PolyZp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }

        let mut terms = Vec::new();
        for (i, &coeff) in self.coefficients().iter().enumerate().rev() {
            if coeff == 0 {
                continue;
            }

            let term = if i == 0 {
                format!("{}", coeff)
            } else if i == 1 {
                if coeff == 1 {
                    "x".to_owned()
                } else {
                    format!("{}*x", coeff)
                }
            } else if coeff == 1 {
                format!("x^{}", i)
            } else {
                format!("{}*x^{}", coeff, i)
            };

            terms.push(term);
        }

        write!(f, "{}", terms.join(" + "))
    }
}

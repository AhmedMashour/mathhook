//! Code generation for Expression constructors
//!
//! Converts parsed expressions into TokenStreams that call mathhook Expression constructors.
use proc_macro2::TokenStream;
use quote::quote;
/// Code generator for Expression constructor calls
pub struct CodeGenerator;
impl CodeGenerator {
    /// Generate code for integer literal
    pub fn generate_integer(value: i64) -> TokenStream {
        quote! {
            mathhook_core::Expression::integer(# value)
        }
    }
    /// Generate code for float literal
    pub fn generate_float(value: f64) -> TokenStream {
        quote! {
            mathhook_core::Expression::float(# value)
        }
    }
    /// Generate code for symbol
    pub fn generate_symbol(name: &str) -> TokenStream {
        match name {
            "pi" => {
                quote! {
                    mathhook_core::Expression::pi()
                }
            }
            "e" => {
                quote! {
                    mathhook_core::Expression::e()
                }
            }
            "i" => {
                quote! {
                    mathhook_core::Expression::i()
                }
            }
            _ => {
                quote! {
                    mathhook_core::Expression::symbol(mathhook_core::Symbol::scalar(#
                    name))
                }
            }
        }
    }
    /// Generate code for addition: a + b
    pub fn generate_add(left: &TokenStream, right: &TokenStream) -> TokenStream {
        quote! {
            mathhook_core::Expression::add(vec![# left, # right])
        }
    }
    /// Generate code for subtraction: a - b
    ///
    /// Implemented as a + (-1 * b) for consistency with mathematical conventions
    pub fn generate_sub(left: &TokenStream, right: &TokenStream) -> TokenStream {
        let neg_right = Self::generate_neg(right);
        quote! {
            mathhook_core::Expression::add(vec![# left, # neg_right])
        }
    }
    /// Generate code for multiplication: a * b
    pub fn generate_mul(left: &TokenStream, right: &TokenStream) -> TokenStream {
        quote! {
            mathhook_core::Expression::mul(vec![# left, # right])
        }
    }
    /// Generate code for division: a / b
    ///
    /// Implemented as a * b^(-1) for consistency with mathematical conventions
    pub fn generate_div(left: &TokenStream, right: &TokenStream) -> TokenStream {
        let inv_right = quote! {
            mathhook_core::Expression::pow(# right, mathhook_core::Expression::integer(-
            1))
        };
        quote! {
            mathhook_core::Expression::mul(vec![# left, # inv_right])
        }
    }
    /// Generate code for power: a**b or a.pow(b)
    pub fn generate_pow(base: &TokenStream, exp: &TokenStream) -> TokenStream {
        quote! {
            mathhook_core::Expression::pow(# base, # exp)
        }
    }
    /// Generate code for negation: -x
    ///
    /// Implemented as -1 * x
    pub fn generate_neg(expr: &TokenStream) -> TokenStream {
        quote! {
            mathhook_core::Expression::mul(vec![mathhook_core::Expression::integer(- 1),
            # expr])
        }
    }
    /// Generate code for function call: f(x, y, ...)
    pub fn generate_function(name: &str, args: &Vec<TokenStream>) -> TokenStream {
        quote! {
            mathhook_core::Expression::function(# name, vec![# (# args),*])
        }
    }
    /// Generate code for equality: a == b
    pub fn generate_relation_equal(left: &TokenStream, right: &TokenStream) -> TokenStream {
        quote! {
            mathhook_core::Expression::relation(# left, # right,
            mathhook_core::core::expression::RelationType::Equal)
        }
    }
    /// Generate code for less than: a < b
    pub fn generate_relation_less(left: &TokenStream, right: &TokenStream) -> TokenStream {
        quote! {
            mathhook_core::Expression::relation(# left, # right,
            mathhook_core::core::expression::RelationType::Less)
        }
    }
    /// Generate code for greater than: a > b
    pub fn generate_relation_greater(left: &TokenStream, right: &TokenStream) -> TokenStream {
        quote! {
            mathhook_core::Expression::relation(# left, # right,
            mathhook_core::core::expression::RelationType::Greater)
        }
    }
    /// Generate code for less than or equal: a <= b
    pub fn generate_relation_less_equal(left: &TokenStream, right: &TokenStream) -> TokenStream {
        quote! {
            mathhook_core::Expression::relation(# left, # right,
            mathhook_core::core::expression::RelationType::LessEqual)
        }
    }
    /// Generate code for greater than or equal: a >= b
    pub fn generate_relation_greater_equal(left: &TokenStream, right: &TokenStream) -> TokenStream {
        quote! {
            mathhook_core::Expression::relation(# left, # right,
            mathhook_core::core::expression::RelationType::GreaterEqual)
        }
    }
    /// Generate code for absolute value: x.abs()
    pub fn generate_method_abs(receiver: &TokenStream) -> TokenStream {
        quote! {
            mathhook_core::Expression::function("abs", vec![# receiver])
        }
    }
    /// Generate code for square root: x.sqrt()
    pub fn generate_method_sqrt(receiver: &TokenStream) -> TokenStream {
        quote! {
            mathhook_core::Expression::function("sqrt", vec![# receiver])
        }
    }
    /// Generate code for simplify method: x.simplify()
    pub fn generate_method_simplify(receiver: &TokenStream) -> TokenStream {
        quote! {
            (# receiver).simplify()
        }
    }
}

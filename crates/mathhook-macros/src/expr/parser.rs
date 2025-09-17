//! Expression parser for procedural expr!() macro
//!
//! Parses syn::Expr into an intermediate representation that can be converted
//! to mathhook Expression constructor calls.
//!
//! "**" power operator support via ^ (BitXor) preprocessing.
use super::codegen::CodeGenerator;
use super::errors;
use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{BinOp, Expr, ExprBinary, ExprLit, ExprParen, ExprUnary, Lit, UnOp};
/// Parser for mathematical expressions
pub struct ExprParser;
impl ExprParser {
    /// Parse a syn::Expr into TokenStream of Expression constructors
    pub fn parse(expr: &Expr) -> syn::Result<TokenStream> {
        Self::parse_expr(expr)
    }
    /// Recursively parse expression tree
    fn parse_expr(expr: &Expr) -> syn::Result<TokenStream> {
        match expr {
            Expr::Lit(ExprLit { lit, .. }) => Self::parse_literal(lit),
            Expr::Path(path) => {
                if path.path.segments.len() == 1 {
                    let ident = &path.path.segments[0].ident;
                    let name = ident.to_string();
                    Ok(CodeGenerator::generate_symbol(&name))
                } else {
                    Err(errors::unsupported_expression(
                        "qualified path",
                        path.span(),
                    ))
                }
            }
            Expr::Binary(binary) => Self::parse_binary(binary),
            Expr::Unary(unary) => Self::parse_unary(unary),
            Expr::Paren(ExprParen { expr, .. }) => Self::parse_expr(expr),
            Expr::Call(call) => {
                if let Expr::Path(path) = call.func.as_ref() {
                    if path.path.segments.len() == 1 {
                        let func_name = path.path.segments[0].ident.to_string();
                        let mut args = Vec::new();
                        for arg in &call.args {
                            args.push(Self::parse_expr(arg)?);
                        }
                        return Ok(CodeGenerator::generate_function(&func_name, &args));
                    }
                }
                Err(errors::unsupported_expression(
                    "complex function call",
                    call.span(),
                ))
            }
            Expr::MethodCall(method) => Self::parse_method_call(method),
            _ => Err(errors::unsupported_expression(
                "this expression type",
                expr.span(),
            )),
        }
    }
    /// Parse literal values
    fn parse_literal(lit: &Lit) -> syn::Result<TokenStream> {
        match lit {
            Lit::Int(int_lit) => {
                let value = int_lit
                    .base10_parse::<i64>()
                    .map_err(|_| syn::Error::new(int_lit.span(), "Integer literal overflow"))?;
                Ok(CodeGenerator::generate_integer(value))
            }
            Lit::Float(float_lit) => {
                let value = float_lit
                    .base10_parse::<f64>()
                    .map_err(|_| syn::Error::new(float_lit.span(), "Invalid float literal"))?;
                Ok(CodeGenerator::generate_float(value))
            }
            _ => Err(errors::unsupported_expression(
                "this literal type",
                lit.span(),
            )),
        }
    }
    /// Parse binary operations with correct precedence
    ///
    /// Precedence (high to low):
    /// 1. Power (**) - right-associative
    /// 2. Multiplication, Division (*, /) - left-associative
    /// 3. Addition, Subtraction (+, -) - left-associative
    /// 4. Comparisons (==, <, >, <=, >=) - non-associative
    ///
    /// Note: ** is preprocessed to ^ (BitXor) for parsing.
    fn parse_binary(binary: &ExprBinary) -> syn::Result<TokenStream> {
        match &binary.op {
            BinOp::BitXor(_) => {
                let left = Self::parse_expr(&binary.left)?;
                let right = Self::parse_expr(&binary.right)?;
                Ok(CodeGenerator::generate_pow(&left, &right))
            }
            BinOp::Mul(_) => {
                let left = Self::parse_expr(&binary.left)?;
                let right = Self::parse_expr(&binary.right)?;
                Ok(CodeGenerator::generate_mul(&left, &right))
            }
            BinOp::Div(_) => {
                let left = Self::parse_expr(&binary.left)?;
                let right = Self::parse_expr(&binary.right)?;
                Ok(CodeGenerator::generate_div(&left, &right))
            }
            BinOp::Add(_) => {
                let left = Self::parse_expr(&binary.left)?;
                let right = Self::parse_expr(&binary.right)?;
                Ok(CodeGenerator::generate_add(&left, &right))
            }
            BinOp::Sub(_) => {
                let left = Self::parse_expr(&binary.left)?;
                let right = Self::parse_expr(&binary.right)?;
                Ok(CodeGenerator::generate_sub(&left, &right))
            }
            BinOp::Eq(_) => {
                let left = Self::parse_expr(&binary.left)?;
                let right = Self::parse_expr(&binary.right)?;
                Ok(CodeGenerator::generate_relation_equal(&left, &right))
            }
            BinOp::Lt(_) => {
                let left = Self::parse_expr(&binary.left)?;
                let right = Self::parse_expr(&binary.right)?;
                Ok(CodeGenerator::generate_relation_less(&left, &right))
            }
            BinOp::Gt(_) => {
                let left = Self::parse_expr(&binary.left)?;
                let right = Self::parse_expr(&binary.right)?;
                Ok(CodeGenerator::generate_relation_greater(&left, &right))
            }
            BinOp::Le(_) => {
                let left = Self::parse_expr(&binary.left)?;
                let right = Self::parse_expr(&binary.right)?;
                Ok(CodeGenerator::generate_relation_less_equal(&left, &right))
            }
            BinOp::Ge(_) => {
                let left = Self::parse_expr(&binary.left)?;
                let right = Self::parse_expr(&binary.right)?;
                Ok(CodeGenerator::generate_relation_greater_equal(
                    &left, &right,
                ))
            }
            _ => Err(errors::unsupported_operator("this operator", binary.span())),
        }
    }
    /// Parse unary operations
    fn parse_unary(unary: &ExprUnary) -> syn::Result<TokenStream> {
        match unary.op {
            UnOp::Neg(_) => {
                let expr = Self::parse_expr(&unary.expr)?;
                Ok(CodeGenerator::generate_neg(&expr))
            }
            UnOp::Deref(_) => Err(errors::unsupported_unary_operator("*", unary.span())),
            _ => Err(errors::unsupported_unary_operator(
                "this unary operator",
                unary.span(),
            )),
        }
    }
    /// Parse method calls
    fn parse_method_call(method: &syn::ExprMethodCall) -> syn::Result<TokenStream> {
        let method_name = method.method.to_string();
        let receiver = Self::parse_expr(&method.receiver)?;
        match method_name.as_str() {
            "pow" if method.args.len() == 1 => {
                let arg = Self::parse_expr(&method.args[0])?;
                Ok(CodeGenerator::generate_pow(&receiver, &arg))
            }
            "abs" if method.args.is_empty() => Ok(CodeGenerator::generate_method_abs(&receiver)),
            "sqrt" if method.args.is_empty() => Ok(CodeGenerator::generate_method_sqrt(&receiver)),
            "simplify" if method.args.is_empty() => {
                Ok(CodeGenerator::generate_method_simplify(&receiver))
            }
            _ => Err(errors::unsupported_method_call(
                &method_name,
                method.method.span(),
            )),
        }
    }
}

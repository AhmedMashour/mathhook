use super::{LaTeXContext, LaTeXFormatter, MAX_RECURSION_DEPTH, MAX_TERMS_PER_OPERATION};
use crate::core::expression::smart_display::SmartDisplayFormatter;
use crate::core::expression::{CalculusData, LimitDirection, Matrix, RelationType};
use crate::core::{Expression, MathConstant, Number};
use crate::formatter::FormattingError;

pub(super) fn to_latex_with_depth_impl(
    expr: &Expression,
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    if depth > MAX_RECURSION_DEPTH {
        return Err(FormattingError::RecursionLimitExceeded {
            depth,
            limit: MAX_RECURSION_DEPTH,
        });
    }

    Ok(match expr {
        Expression::Number(num) => format_number(num),
        Expression::Symbol(s) => s.name().to_string(),
        Expression::Add(terms) => format_addition(terms, context, depth)?,
        Expression::Mul(factors) => format_multiplication(factors, context, depth)?,
        Expression::Pow(base, exp) => format_power(base, exp, context, depth)?,
        Expression::Function { name, args } => {
            expr.function_to_latex_with_depth(name, args, context, depth + 1)?
        }
        Expression::Constant(c) => format_constant(c),
        Expression::Complex(complex_data) => format!(
            "{} + {}i",
            complex_data.real.to_latex_with_depth(context, depth + 1)?,
            complex_data.imag.to_latex_with_depth(context, depth + 1)?
        ),
        Expression::Matrix(matrix) => format_matrix(matrix, context, depth)?,
        Expression::Relation(relation_data) => format_relation(relation_data, context, depth)?,
        Expression::Piecewise(piecewise_data) => format_piecewise(piecewise_data, context, depth)?,
        Expression::Set(elements) => format_set(elements, context, depth)?,
        Expression::Interval(interval_data) => format_interval(interval_data, context, depth)?,
        Expression::Calculus(calculus_data) => format_calculus(calculus_data, context, depth)?,
        Expression::MethodCall(method_data) => format!(
            "{}.{}({})",
            method_data.object.to_latex_with_depth(context, depth + 1)?,
            method_data.method_name,
            method_data
                .args
                .iter()
                .map(|arg| arg.to_latex_with_depth(context, depth + 1))
                .collect::<Result<Vec<_>, _>>()?
                .join(", ")
        ),
    })
}

/// Format number in LaTeX notation
fn format_number(num: &Number) -> String {
    match num {
        Number::Integer(n) => n.to_string(),
        Number::BigInteger(n) => n.to_string(),
        Number::Rational(r) => {
            if r.denom() == &num_bigint::BigInt::from(1) {
                r.numer().to_string()
            } else {
                format!("\\frac{{{}}}{{{}}}", r.numer(), r.denom())
            }
        }
        Number::Float(f) => f.to_string(),
    }
}

/// Format mathematical constant
fn format_constant(c: &MathConstant) -> String {
    match c {
        MathConstant::Pi => "\\pi".to_string(),
        MathConstant::E => "e".to_string(),
        MathConstant::I => "i".to_string(),
        MathConstant::Infinity => "\\infty".to_string(),
        MathConstant::NegativeInfinity => "-\\infty".to_string(),
        MathConstant::Undefined => "\\text{undefined}".to_string(),
        MathConstant::GoldenRatio => "\\phi".to_string(),
        MathConstant::EulerGamma => "\\gamma".to_string(),
        MathConstant::TribonacciConstant => "\\alpha_3".to_string(),
    }
}

/// Format addition with smart subtraction detection
fn format_addition(
    terms: &[Expression],
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    if terms.len() > MAX_TERMS_PER_OPERATION {
        return Err(FormattingError::TooManyTerms {
            count: terms.len(),
            limit: MAX_TERMS_PER_OPERATION,
        });
    }

    let mut term_strs = Vec::with_capacity(terms.len());
    for (i, term) in terms.iter().enumerate() {
        if i == 0 {
            term_strs.push(term.to_latex_with_depth(context, depth + 1)?);
        } else {
            if SmartDisplayFormatter::is_negated_expression(term) {
                if let Some(positive_part) = SmartDisplayFormatter::extract_negated_expression(term)
                {
                    term_strs.push(format!(
                        " - {}",
                        positive_part.to_latex_with_depth(context, depth + 1)?
                    ));
                } else {
                    term_strs.push(format!(
                        " + {}",
                        term.to_latex_with_depth(context, depth + 1)?
                    ));
                }
            } else {
                term_strs.push(format!(
                    " + {}",
                    term.to_latex_with_depth(context, depth + 1)?
                ));
            }
        }
    }
    Ok(if context.needs_parentheses {
        format!("\\left({}\\right)", term_strs.join(""))
    } else {
        term_strs.join("")
    })
}

/// Format multiplication with smart division detection
fn format_multiplication(
    factors: &[Expression],
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    if factors.len() > MAX_TERMS_PER_OPERATION {
        return Err(FormattingError::TooManyTerms {
            count: factors.len(),
            limit: MAX_TERMS_PER_OPERATION,
        });
    }

    if let Some((dividend, divisor)) = SmartDisplayFormatter::extract_division_parts(factors) {
        return Ok(format!(
            "\\frac{{{}}}{{{}}}",
            dividend.to_latex_with_depth(context, depth + 1)?,
            divisor.to_latex_with_depth(context, depth + 1)?
        ));
    }

    let mut factor_strs = Vec::with_capacity(factors.len());
    for f in factors.iter() {
        let needs_parens = matches!(f, Expression::Add(_));
        if needs_parens {
            factor_strs.push(format!(
                "\\left({}\\right)",
                f.to_latex_with_depth(context, depth + 1)?
            ));
        } else {
            factor_strs.push(f.to_latex_with_depth(context, depth + 1)?);
        }
    }

    if factors.len() == 2 {
        let first = factors[0].to_latex_with_depth(context, depth + 1)?;
        let second = factors[1].to_latex_with_depth(context, depth + 1)?;

        if let (Expression::Number(_), Expression::Constant(_)) = (&factors[0], &factors[1]) {
            Ok(format!("{}{}", first, second))
        } else {
            Ok(format!("{} \\cdot {}", first, second))
        }
    } else {
        Ok(factor_strs.join(" \\cdot "))
    }
}

/// Format power with smart handling of square roots and function powers
fn format_power(
    base: &Box<Expression>,
    exp: &Box<Expression>,
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    if let Expression::Number(Number::Rational(r)) = exp.as_ref() {
        if r.numer() == &num_bigint::BigInt::from(1) && r.denom() == &num_bigint::BigInt::from(2) {
            return Ok(format!(
                "\\sqrt{{{}}}",
                base.to_latex_with_depth(context, depth + 1)?
            ));
        }
    }

    if let Expression::Function { name, args } = base.as_ref() {
        println!("print function: {}", name);
        return Ok(format!(
            "\\{}^{{{}}}({})",
            name,
            exp.to_latex_with_depth(context, depth + 1)?,
            args[0].to_latex_with_depth(context, depth + 1)?
        ));
    }

    let base_str = match base.as_ref() {
        Expression::Add(_) | Expression::Mul(_) => {
            format!(
                "\\left({}\\right)",
                base.to_latex_with_depth(context, depth + 1)?
            )
        }
        _ => base.to_latex_with_depth(context, depth + 1)?,
    };

    let exp_str = exp.to_latex_with_depth(context, depth + 1)?;

    let clean_exp_str = if exp_str.starts_with("\\{") && exp_str.ends_with("\\}") {
        exp_str[2..exp_str.len() - 2].to_string()
    } else {
        exp_str
    };

    Ok(
        if clean_exp_str.len() == 1 || (clean_exp_str.len() == 2 && clean_exp_str.starts_with('-'))
        {
            if clean_exp_str.starts_with('-') {
                format!("{}^{{{}}}", base_str, clean_exp_str)
            } else {
                format!("{}^{}", base_str, clean_exp_str)
            }
        } else {
            format!("{}^{{{}}}", base_str, clean_exp_str)
        },
    )
}

/// Format matrix in LaTeX pmatrix environment
fn format_matrix(
    matrix: &Matrix,
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    let (rows, cols) = matrix.dimensions();
    let mut row_strs = Vec::with_capacity(rows);

    for i in 0..rows {
        let mut col_strs = Vec::with_capacity(cols);
        for j in 0..cols {
            let elem = matrix.get_element(i, j);
            col_strs.push(elem.to_latex_with_depth(context, depth + 1)?);
        }
        row_strs.push(col_strs.join(" & "));
    }

    Ok(format!(
        "\\begin{{pmatrix}} {} \\end{{pmatrix}}",
        row_strs.join(" \\\\ ")
    ))
}

/// Format relational expression
fn format_relation(
    relation_data: &crate::core::expression::RelationData,
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    let left_latex = relation_data.left.to_latex_with_depth(context, depth + 1)?;
    let right_latex = relation_data
        .right
        .to_latex_with_depth(context, depth + 1)?;
    let operator = match relation_data.relation_type {
        RelationType::Equal => "=",
        RelationType::NotEqual => "\\neq",
        RelationType::Less => "<",
        RelationType::LessEqual => "\\leq",
        RelationType::Greater => ">",
        RelationType::GreaterEqual => "\\geq",
        RelationType::Approximate => "\\approx",
        RelationType::Similar => "\\sim",
        RelationType::Proportional => "\\propto",
    };
    Ok(format!("{} {} {}", left_latex, operator, right_latex))
}

/// Format piecewise function
fn format_piecewise(
    piecewise_data: &crate::core::expression::PiecewiseData,
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    let mut cases = Vec::new();

    for (condition, value) in &piecewise_data.pieces {
        let condition_latex = condition.to_latex_with_depth(context, depth + 1)?;
        let value_latex = value.to_latex_with_depth(context, depth + 1)?;
        cases.push(format!(
            "{} & \\text{{if }} {}",
            value_latex, condition_latex
        ));
    }

    if let Some(default_value) = &piecewise_data.default {
        let default_latex = default_value.to_latex_with_depth(context, depth + 1)?;
        cases.push(format!("{} & \\text{{otherwise}}", default_latex));
    }

    Ok(format!(
        "\\begin{{cases}} {} \\end{{cases}}",
        cases.join(" \\\\\\\\ ")
    ))
}

/// Format set notation
fn format_set(
    elements: &[Expression],
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    if elements.is_empty() {
        Ok("\\{\\}".to_string())
    } else {
        let mut element_strs = Vec::with_capacity(elements.len());
        for elem in elements.iter() {
            element_strs.push(elem.to_latex_with_depth(context, depth + 1)?);
        }
        Ok(format!("\\{{{}\\}}", element_strs.join(", ")))
    }
}

/// Format interval notation
fn format_interval(
    interval_data: &crate::core::expression::IntervalData,
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    let start_bracket = if interval_data.start_inclusive {
        "["
    } else {
        "("
    };
    let end_bracket = if interval_data.end_inclusive {
        "]"
    } else {
        ")"
    };
    let start_latex = interval_data
        .start
        .to_latex_with_depth(context, depth + 1)?;
    let end_latex = interval_data.end.to_latex_with_depth(context, depth + 1)?;
    Ok(format!(
        "{}{}, {}{}",
        start_bracket, start_latex, end_latex, end_bracket
    ))
}

/// Format calculus expressions (derivatives, integrals, limits, sums, products)
fn format_calculus(
    calculus_data: &CalculusData,
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    Ok(match calculus_data {
        CalculusData::Derivative {
            expression,
            variable,
            order,
        } => {
            if *order == 1 {
                format!(
                    "\\frac{{d}}{{d{}}} {}",
                    variable.name(),
                    expression.to_latex_with_depth(context, depth + 1)?
                )
            } else {
                format!(
                    "\\frac{{d^{}}}{{d{}^{}}} {}",
                    order,
                    variable.name(),
                    order,
                    expression.to_latex_with_depth(context, depth + 1)?
                )
            }
        }
        CalculusData::Integral {
            integrand,
            variable,
            bounds,
        } => match bounds {
            None => format!(
                "\\int {} d{}",
                integrand.to_latex_with_depth(context, depth + 1)?,
                variable.name()
            ),
            Some((start, end)) => format!(
                "\\int_{}^{} {} d{}",
                start.to_latex_with_depth(context, depth + 1)?,
                end.to_latex_with_depth(context, depth + 1)?,
                integrand.to_latex_with_depth(context, depth + 1)?,
                variable.name()
            ),
        },
        CalculusData::Limit {
            expression,
            variable,
            direction,
            ..
        } => {
            format!(
                "\\lim_{{{}\\to{}}} {}",
                variable.name(),
                match direction {
                    LimitDirection::Left => "0^-",
                    LimitDirection::Right => "0^+",
                    LimitDirection::Both => "0",
                },
                expression.to_latex_with_depth(context, depth + 1)?
            )
        }
        CalculusData::Sum {
            expression,
            variable,
            start,
            end,
        } => {
            format!(
                "\\sum_{{{}={}}}^{} {}",
                variable.name(),
                start.to_latex_with_depth(context, depth + 1)?,
                end.to_latex_with_depth(context, depth + 1)?,
                expression.to_latex_with_depth(context, depth + 1)?
            )
        }
        CalculusData::Product {
            expression,
            variable,
            start,
            end,
        } => {
            format!(
                "\\prod_{{{}={}}}^{{{}}} {}",
                variable.name(),
                start.to_latex_with_depth(context, depth + 1)?,
                end.to_latex_with_depth(context, depth + 1)?,
                expression.to_latex_with_depth(context, depth + 1)?
            )
        }
    })
}

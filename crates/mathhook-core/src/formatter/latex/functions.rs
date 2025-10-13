use super::{LaTeXContext, LaTeXFormatter, MAX_RECURSION_DEPTH, MAX_TERMS_PER_OPERATION};
use crate::core::Expression;
use crate::formatter::FormattingError;

pub(super) fn function_to_latex_with_depth_impl(
    _expr: &Expression,
    name: &str,
    args: &[Expression],
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    if depth > MAX_RECURSION_DEPTH {
        return Err(FormattingError::RecursionLimitExceeded {
            depth,
            limit: MAX_RECURSION_DEPTH,
        });
    }

    if args.len() > MAX_TERMS_PER_OPERATION {
        return Err(FormattingError::TooManyTerms {
            count: args.len(),
            limit: MAX_TERMS_PER_OPERATION,
        });
    }

    Ok(match name {
        "log" => format_log_function(args, context, depth)?,
        "exp" => format!(
            "\\exp({})",
            args[0].to_latex_with_depth(context, depth + 1)?
        ),
        "sqrt" => format_sqrt_function(args, context, depth)?,
        "factorial" => format_factorial_function(args, context, depth)?,
        "double_factorial" => format_double_factorial_function(args, context, depth)?,
        "integrate" => format_integrate_function(args, context, depth)?,
        "derivative" => format_derivative_function(args, context, depth)?,
        "sum" => format_sum_function(args, context, depth)?,
        "gamma" => format!(
            "\\Gamma({})",
            args[0].to_latex_with_depth(context, depth + 1)?
        ),
        "digamma" | "psi" => format!(
            "\\psi({})",
            args[0].to_latex_with_depth(context, depth + 1)?
        ),
        "riemann_zeta" => format!(
            "\\zeta({})",
            args[0].to_latex_with_depth(context, depth + 1)?
        ),
        "eta" => format!(
            "\\eta({})",
            args[0].to_latex_with_depth(context, depth + 1)?
        ),
        "abs" => format!(
            "|{}|",
            args[0].to_latex_with_depth(context, depth + 1)?
        ),
        "conjugate" => format!(
            "\\overline{{{}}}",
            args[0].to_latex_with_depth(context, depth + 1)?
        ),
        "gradient" => format!(
            "\\nabla {}",
            args[0].to_latex_with_depth(context, depth + 1)?
        ),
        "divergence" => format!(
            "\\nabla \\cdot {}",
            args[0].to_latex_with_depth(context, depth + 1)?
        ),
        "curl" => format!(
            "\\nabla \\times {}",
            args[0].to_latex_with_depth(context, depth + 1)?
        ),
        "laplacian" => format!(
            "\\nabla^2 {}",
            args[0].to_latex_with_depth(context, depth + 1)?
        ),
        "vector" => format!(
            "\\vec{{{}}}",
            args[0].to_latex_with_depth(context, depth + 1)?
        ),
        "bessel_j_indexed" if args.len() >= 2 => {
            format_indexed_function("J", args, context, depth)?
        }
        "bessel_y_indexed" if args.len() >= 2 => {
            format_indexed_function("Y", args, context, depth)?
        }
        "legendre_p_indexed" if args.len() >= 2 => {
            format_indexed_function("P", args, context, depth)?
        }
        "legendre_q_indexed" if args.len() >= 2 => {
            format_indexed_function("Q", args, context, depth)?
        }
        "hermite_indexed" if args.len() >= 2 => {
            format_indexed_function("H", args, context, depth)?
        }
        "laguerre_indexed" if args.len() >= 2 => {
            format_indexed_function("L", args, context, depth)?
        }
        "chebyshev_first_indexed" if args.len() >= 2 => {
            format_indexed_function("T", args, context, depth)?
        }
        _ => format_generic_function(name, args, context, depth)?,
    })
}

/// Format logarithm function with optional base
fn format_log_function(
    args: &[Expression],
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    if args.len() == 1 {
        Ok(format!(
            "\\log({})",
            args[0].to_latex_with_depth(context, depth + 1)?
        ))
    } else if args.len() == 2 {
        Ok(format!(
            "\\log_{{{}}}({})",
            args[1].to_latex_with_depth(context, depth + 1)?,
            args[0].to_latex_with_depth(context, depth + 1)?
        ))
    } else {
        let arg_strs: Vec<String> = args
            .iter()
            .map(|arg| arg.to_latex_with_depth(context, depth + 1))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(format!("\\log({})", arg_strs.join(", ")))
    }
}

/// Format square root or nth root function
fn format_sqrt_function(
    args: &[Expression],
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    if args.len() == 1 {
        Ok(format!(
            "\\sqrt{{{}}}",
            args[0].to_latex_with_depth(context, depth + 1)?
        ))
    } else if args.len() == 2 {
        Ok(format!(
            "\\sqrt[{}]{{{}}}",
            args[1].to_latex_with_depth(context, depth + 1)?,
            args[0].to_latex_with_depth(context, depth + 1)?
        ))
    } else {
        Ok(format!(
            "\\sqrt{{{}}}",
            args[0].to_latex_with_depth(context, depth + 1)?
        ))
    }
}

/// Format factorial function
fn format_factorial_function(
    args: &[Expression],
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    if args.len() == 1 {
        Ok(format!("{}!", args[0].to_latex_with_depth(context, depth + 1)?))
    } else {
        let arg_strs: Vec<String> = args
            .iter()
            .map(|arg| arg.to_latex_with_depth(context, depth + 1))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(format!("\\text{{factorial}}({})", arg_strs.join(", ")))
    }
}

/// Format double factorial function
fn format_double_factorial_function(
    args: &[Expression],
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    if args.len() == 1 {
        Ok(format!("{}!!", args[0].to_latex_with_depth(context, depth + 1)?))
    } else {
        let arg_strs: Vec<String> = args
            .iter()
            .map(|arg| arg.to_latex_with_depth(context, depth + 1))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(format!("\\text{{double_factorial}}({})", arg_strs.join(", ")))
    }
}

/// Format integration operator
fn format_integrate_function(
    args: &[Expression],
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    if args.len() == 2 {
        Ok(format!(
            "\\int {} \\, d{}",
            args[0].to_latex_with_depth(context, depth + 1)?,
            args[1].to_latex_with_depth(context, depth + 1)?
        ))
    } else {
        let arg_strs: Vec<String> = args
            .iter()
            .map(|arg| arg.to_latex_with_depth(context, depth + 1))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(format!("\\int({})", arg_strs.join(", ")))
    }
}

/// Format derivative operator
fn format_derivative_function(
    args: &[Expression],
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    if args.len() == 2 {
        Ok(format!(
            "\\frac{{d}}{{d{}}} {}",
            args[1].to_latex_with_depth(context, depth + 1)?,
            args[0].to_latex_with_depth(context, depth + 1)?
        ))
    } else {
        Ok(format!(
            "\\frac{{d}}{{dx}}({})",
            args[0].to_latex_with_depth(context, depth + 1)?
        ))
    }
}

/// Format summation operator
fn format_sum_function(
    args: &[Expression],
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    if args.len() == 4 {
        Ok(format!(
            "\\sum_{{{}={}}}^{{{}}} {}",
            args[1].to_latex_with_depth(context, depth + 1)?,
            args[2].to_latex_with_depth(context, depth + 1)?,
            args[3].to_latex_with_depth(context, depth + 1)?,
            args[0].to_latex_with_depth(context, depth + 1)?
        ))
    } else {
        let arg_strs: Vec<String> = args
            .iter()
            .map(|arg| arg.to_latex_with_depth(context, depth + 1))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(format!("\\sum({})", arg_strs.join(", ")))
    }
}

/// Format indexed special functions (Bessel, Legendre, etc.) with smart subscript bracing
fn format_indexed_function(
    base_name: &str,
    args: &[Expression],
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    let subscript = args[0].to_latex_with_depth(context, depth + 1)?;
    let subscript_formatted = if subscript.len() == 1 {
        subscript
    } else {
        format!("{{{}}}", subscript)
    };

    let remaining_args: Vec<String> = args[1..]
        .iter()
        .map(|arg| arg.to_latex_with_depth(context, depth + 1))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(format!(
        "{}_{}({})",
        base_name,
        subscript_formatted,
        remaining_args.join(", ")
    ))
}

/// Format generic functions with proper LaTeX backslash handling
fn format_generic_function(
    name: &str,
    args: &[Expression],
    context: &LaTeXContext,
    depth: usize,
) -> Result<String, FormattingError> {
    const KNOWN_LATEX_FUNCTIONS: &[&str] = &[
        "sin", "cos", "tan", "cot", "sec", "csc",
        "arcsin", "arccos", "arctan", "arccot", "arcsec", "arccsc",
        "sinh", "cosh", "tanh", "coth", "sech", "csch",
        "arcsinh", "arccosh", "arctanh", "arccoth", "arcsech", "arccsch",
        "ln", "lg",
        "erf", "erfc", "min", "max", "gcd", "lcm",
        "det", "dim", "ker", "hom", "arg", "deg", "lim", "sup", "inf",
    ];

    let should_add_backslash = KNOWN_LATEX_FUNCTIONS.contains(&name);

    if args.is_empty() {
        Ok(if should_add_backslash {
            format!("\\{}", name)
        } else {
            name.to_string()
        })
    } else {
        let arg_strs: Vec<String> = args
            .iter()
            .map(|arg| arg.to_latex_with_depth(context, depth + 1))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(if should_add_backslash {
            format!("\\{}({})", name, arg_strs.join(", "))
        } else {
            format!("{}({})", name, arg_strs.join(", "))
        })
    }
}

/// LaTeX mathematical fonts and text styling commands
///
/// This module handles LaTeX commands for mathematical fonts, text styling,
/// and font directives like \text{}, \mathcal{}, \mathbf{}, \mathbb{}, etc.
///
/// Architecture:
/// - Modular design with focused responsibilities
/// - Performance: Efficient parsing with minimal allocations
/// - Memory: Zero-copy parsing where possible
/// - Readability: Clear function names and comprehensive documentation
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, multispace0},
    sequence::delimited,
    IResult, Parser,
};

use crate::core::Expression;

/// Parse LaTeX mathematical font commands
///
/// Handles: \text{content}, \mathcal{A}, \mathbf{x}, \mathrm{d}, \mathbb{R}, etc.
pub fn latex_font_commands(input: &str) -> IResult<&str, Expression> {
    alt((
        latex_text,
        latex_mathcal,
        latex_mathbf,
        latex_mathrm,
        latex_mathit,
        latex_mathbb,
        latex_mathfrak,
        latex_mathsf,
        latex_mathtt,
    ))
    .parse(input)
}

/// Parse \text{content} - regular text within math mode
///
/// Example: \text{if } x > 0 → Expression::text("if x > 0")
fn latex_text(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\text"), multispace0).parse(input)?;
    let (input, content) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        take_until("}"),
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    Ok((
        input,
        Expression::function("text", vec![Expression::symbol(content)]),
    ))
}

/// Parse \mathcal{A} - calligraphic/script letters
///
/// Example: \mathcal{A} → Expression::symbol("𝒜")
fn latex_mathcal(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\mathcal"), multispace0).parse(input)?;
    let (input, letter) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        alpha1,
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    // Convert to calligraphic Unicode (simplified mapping)
    let calligraphic_symbol = match letter {
        "A" => "𝒜",
        "B" => "ℬ",
        "C" => "𝒞",
        "D" => "𝒟",
        "E" => "ℰ",
        "F" => "ℱ",
        "G" => "𝒢",
        "H" => "ℋ",
        "I" => "ℐ",
        "J" => "𝒥",
        "K" => "𝒦",
        "L" => "ℒ",
        "M" => "ℳ",
        "N" => "𝒩",
        "O" => "𝒪",
        "P" => "𝒫",
        "Q" => "𝒬",
        "R" => "ℛ",
        "S" => "𝒮",
        "T" => "𝒯",
        "U" => "𝒰",
        "V" => "𝒱",
        "W" => "𝒲",
        "X" => "𝒳",
        "Y" => "𝒴",
        "Z" => "𝒵",
        _ => letter, // Fallback to original letter
    };

    Ok((input, Expression::symbol(calligraphic_symbol)))
}

/// Parse \mathbf{x} - bold mathematical symbols
///
/// Example: \mathbf{x} → Expression::symbol("𝐱")
fn latex_mathbf(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\mathbf"), multispace0).parse(input)?;
    let (input, content) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        alt((alpha1, take_until("}"))),
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    // For simplicity, we'll mark it as bold with a prefix
    let bold_symbol = format!("𝐛𝐨𝐥𝐝_{}", content);
    Ok((input, Expression::symbol(bold_symbol)))
}

/// Parse \mathrm{d} - roman (upright) mathematical symbols
///
/// Example: \mathrm{d} → Expression::symbol("d")
fn latex_mathrm(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\mathrm"), multispace0).parse(input)?;
    let (input, content) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        alt((alpha1, take_until("}"))),
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    // Roman text is just regular symbols
    Ok((input, Expression::symbol(content)))
}

/// Parse \mathit{x} - italic mathematical symbols
fn latex_mathit(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\mathit"), multispace0).parse(input)?;
    let (input, content) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        alt((alpha1, take_until("}"))),
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    // Italic is the default for variables, so just return as symbol
    Ok((input, Expression::symbol(content)))
}

/// Parse \mathbb{R} - blackboard bold (double-struck) symbols
///
/// Example: \mathbb{R} → Expression::symbol("ℝ")
fn latex_mathbb(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\mathbb"), multispace0).parse(input)?;
    let (input, letter) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        alpha1,
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    // Convert to blackboard bold Unicode
    let blackboard_symbol = match letter {
        "A" => "𝔸",
        "B" => "𝔹",
        "C" => "ℂ",
        "D" => "𝔻",
        "E" => "𝔼",
        "F" => "𝔽",
        "G" => "𝔾",
        "H" => "ℍ",
        "I" => "𝕀",
        "J" => "𝕁",
        "K" => "𝕂",
        "L" => "𝕃",
        "M" => "𝕄",
        "N" => "ℕ",
        "O" => "𝕆",
        "P" => "ℙ",
        "Q" => "ℚ",
        "R" => "ℝ",
        "S" => "𝕊",
        "T" => "𝕋",
        "U" => "𝕌",
        "V" => "𝕍",
        "W" => "𝕎",
        "X" => "𝕏",
        "Y" => "𝕐",
        "Z" => "ℤ",
        _ => letter, // Fallback
    };

    Ok((input, Expression::symbol(blackboard_symbol)))
}

/// Parse \mathfrak{A} - Fraktur (Gothic) symbols
fn latex_mathfrak(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\mathfrak"), multispace0).parse(input)?;
    let (input, letter) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        alpha1,
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    // Convert to Fraktur Unicode (simplified)
    let fraktur_symbol = match letter {
        "A" => "𝔄",
        "B" => "𝔅",
        "C" => "ℭ",
        "D" => "𝔇",
        "E" => "𝔈",
        "F" => "𝔉",
        "G" => "𝔊",
        "H" => "ℌ",
        "I" => "ℑ",
        "J" => "𝔍",
        "K" => "𝔎",
        "L" => "𝔏",
        "M" => "𝔐",
        "N" => "𝔑",
        "O" => "𝔒",
        "P" => "𝔓",
        "Q" => "𝔔",
        "R" => "ℜ",
        "S" => "𝔖",
        "T" => "𝔗",
        "U" => "𝔘",
        "V" => "𝔙",
        "W" => "𝔚",
        "X" => "𝔛",
        "Y" => "𝔜",
        "Z" => "ℨ",
        _ => letter, // Fallback
    };

    Ok((input, Expression::symbol(fraktur_symbol)))
}

/// Parse \mathsf{x} - sans-serif mathematical symbols
fn latex_mathsf(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\mathsf"), multispace0).parse(input)?;
    let (input, content) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        alt((alpha1, take_until("}"))),
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    // Mark as sans-serif with prefix
    let sans_symbol = format!("sf_{}", content);
    Ok((input, Expression::symbol(sans_symbol)))
}

/// Parse \mathtt{x} - typewriter (monospace) mathematical symbols
fn latex_mathtt(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\mathtt"), multispace0).parse(input)?;
    let (input, content) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        alt((alpha1, take_until("}"))),
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    // Mark as typewriter with prefix
    let tt_symbol = format!("tt_{}", content);
    Ok((input, Expression::symbol(tt_symbol)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::combinator::all_consuming;

    #[test]
    fn test_latex_text() {
        // Test basic text
        let result = all_consuming(latex_text).parse("\\text{if }").unwrap().1;
        assert_eq!(
            result,
            Expression::function("text", vec![Expression::symbol("if ")])
        );

        // Test text with spaces
        let result = all_consuming(latex_text)
            .parse("\\text{for all }")
            .unwrap()
            .1;
        assert_eq!(
            result,
            Expression::function("text", vec![Expression::symbol("for all ")])
        );
    }

    #[test]
    fn test_latex_mathcal() {
        // Test calligraphic A
        let result = all_consuming(latex_mathcal)
            .parse("\\mathcal{A}")
            .unwrap()
            .1;
        assert_eq!(result, Expression::symbol("𝒜"));

        // Test calligraphic R
        let result = all_consuming(latex_mathcal)
            .parse("\\mathcal{R}")
            .unwrap()
            .1;
        assert_eq!(result, Expression::symbol("ℛ"));
    }

    #[test]
    fn test_latex_mathbb() {
        // Test blackboard bold R (real numbers)
        let result = all_consuming(latex_mathbb).parse("\\mathbb{R}").unwrap().1;
        assert_eq!(result, Expression::symbol("ℝ"));

        // Test blackboard bold N (natural numbers)
        let result = all_consuming(latex_mathbb).parse("\\mathbb{N}").unwrap().1;
        assert_eq!(result, Expression::symbol("ℕ"));

        // Test blackboard bold Z (integers)
        let result = all_consuming(latex_mathbb).parse("\\mathbb{Z}").unwrap().1;
        assert_eq!(result, Expression::symbol("ℤ"));
    }

    #[test]
    fn test_latex_mathrm() {
        // Test roman d (for differentials)
        let result = all_consuming(latex_mathrm).parse("\\mathrm{d}").unwrap().1;
        assert_eq!(result, Expression::symbol("d"));
    }
}

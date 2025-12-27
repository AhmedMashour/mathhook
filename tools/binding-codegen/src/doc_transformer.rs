//! Documentation transformation utilities for converting Rust doc comments
//! to Python docstrings (Google-style) and JSDoc format.

use crate::scanner::clean_doc_for_binding;

/// Parsed Rust documentation structure
#[derive(Debug, Default)]
pub struct ParsedRustDoc {
    /// Main description (before any section headers)
    pub description: Vec<String>,
    /// Arguments section content (from # Arguments)
    pub arguments: Vec<(String, String)>,
    /// Returns section content (from # Returns)
    pub returns: Option<String>,
    /// Errors section content (from # Errors)
    pub errors: Vec<String>,
    /// Examples section content (from # Examples)
    pub examples: Vec<String>,
    /// Panics section content (from # Panics)
    pub panics: Vec<String>,
    /// Safety section content (from # Safety)
    pub safety: Vec<String>,
}

impl ParsedRustDoc {
    /// Parse a Rust doc comment into structured sections
    pub fn parse(doc: &str) -> Self {
        let mut result = ParsedRustDoc::default();
        let mut current_section = Section::Description;
        let mut current_lines: Vec<String> = Vec::new();

        for line in doc.lines() {
            let trimmed = line.trim();

            if let Some(new_section) = Self::parse_section_header(trimmed) {
                Self::save_section(&mut result, current_section, &current_lines);
                current_lines.clear();
                current_section = new_section;
                continue;
            }

            current_lines.push(line.to_string());
        }

        Self::save_section(&mut result, current_section, &current_lines);

        result
    }

    fn parse_section_header(line: &str) -> Option<Section> {
        if line.starts_with("# Arguments") || line.starts_with("# Parameters") {
            Some(Section::Arguments)
        } else if line.starts_with("# Returns") || line.starts_with("# Return") {
            Some(Section::Returns)
        } else if line.starts_with("# Errors") || line.starts_with("# Error") {
            Some(Section::Errors)
        } else if line.starts_with("# Examples") || line.starts_with("# Example") {
            Some(Section::Examples)
        } else if line.starts_with("# Panics") || line.starts_with("# Panic") {
            Some(Section::Panics)
        } else if line.starts_with("# Safety") {
            Some(Section::Safety)
        } else {
            None
        }
    }

    fn save_section(result: &mut ParsedRustDoc, section: Section, lines: &[String]) {
        let cleaned: Vec<String> = lines
            .iter()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        match section {
            Section::Description => {
                result.description = cleaned;
            }
            Section::Arguments => {
                result.arguments = Self::parse_argument_list(&cleaned);
            }
            Section::Returns => {
                result.returns = if cleaned.is_empty() {
                    None
                } else {
                    Some(cleaned.join(" "))
                };
            }
            Section::Errors => {
                result.errors = cleaned;
            }
            Section::Examples => {
                result.examples = lines.to_vec();
            }
            Section::Panics => {
                result.panics = cleaned;
            }
            Section::Safety => {
                result.safety = cleaned;
            }
        }
    }

    /// Parse argument list from lines like:
    /// * `variable` - The variable to differentiate with respect to
    fn parse_argument_list(lines: &[String]) -> Vec<(String, String)> {
        let mut args = Vec::new();

        for line in lines {
            if let Some(parsed) = Self::parse_argument_line(line) {
                args.push(parsed);
            }
        }

        args
    }

    fn parse_argument_line(line: &str) -> Option<(String, String)> {
        let line = line.trim();

        if line.starts_with('*') || line.starts_with('-') {
            let rest = line.trim_start_matches('*').trim_start_matches('-').trim();

            if let Some(stripped) = rest.strip_prefix('`') {
                if let Some(end_tick) = stripped.find('`') {
                    let name = stripped[..end_tick].to_string();
                    let remaining = stripped[end_tick + 1..].trim();

                    let description = remaining
                        .trim_start_matches('-')
                        .trim_start_matches(':')
                        .trim()
                        .to_string();

                    return Some((name, description));
                }
            }

            if let Some(dash_pos) = rest.find(" - ") {
                let name = rest[..dash_pos].trim().to_string();
                let description = rest[dash_pos + 3..].trim().to_string();
                return Some((name, description));
            }
        }

        None
    }

    pub fn is_empty(&self) -> bool {
        self.description.is_empty()
            && self.arguments.is_empty()
            && self.returns.is_none()
            && self.errors.is_empty()
            && self.examples.is_empty()
            && self.panics.is_empty()
            && self.safety.is_empty()
    }
}

#[derive(Clone, Copy)]
enum Section {
    Description,
    Arguments,
    Returns,
    Errors,
    Examples,
    Panics,
    Safety,
}

/// Transform Rust documentation to Python Google-style docstring format.
/// Outputs multiple #[doc = "..."] attributes for PyO3.
/// Automatically strips @no-binding directives from the output.
pub fn to_python_docstring(doc: &str, indent: usize) -> String {
    let cleaned_doc = clean_doc_for_binding(doc);
    let parsed = ParsedRustDoc::parse(&cleaned_doc);
    if parsed.is_empty() {
        return String::new();
    }

    let indent_str = " ".repeat(indent);
    let mut doc_lines: Vec<String> = Vec::new();

    for line in &parsed.description {
        doc_lines.push(line.clone());
    }

    if !parsed.arguments.is_empty() {
        doc_lines.push(String::new());
        doc_lines.push("Args:".to_string());
        for (name, desc) in &parsed.arguments {
            doc_lines.push(format!("    {}: {}", name, desc));
        }
    }

    if let Some(ref returns) = parsed.returns {
        doc_lines.push(String::new());
        doc_lines.push("Returns:".to_string());
        doc_lines.push(format!("    {}", returns));
    }

    if !parsed.errors.is_empty() {
        doc_lines.push(String::new());
        doc_lines.push("Raises:".to_string());
        for error in &parsed.errors {
            doc_lines.push(format!("    {}", error));
        }
    }

    if !parsed.examples.is_empty() {
        doc_lines.push(String::new());
        doc_lines.push("Example:".to_string());
        let mut in_code_block = false;
        for line in &parsed.examples {
            let trimmed = line.trim();
            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
                doc_lines.push(format!("    >>> {}", trimmed));
            }
        }
    }

    doc_lines
        .iter()
        .map(|line| {
            let escaped = escape_doc_string(line);
            format!("{}#[doc = \"{}\"]", indent_str, escaped)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Transform Rust documentation to JSDoc format
/// Automatically strips @no-binding directives from the output.
pub fn to_jsdoc(doc: &str, indent: usize) -> String {
    let cleaned_doc = clean_doc_for_binding(doc);
    let parsed = ParsedRustDoc::parse(&cleaned_doc);
    let indent_str = " ".repeat(indent);

    let mut lines = Vec::new();
    lines.push("/**".to_string());

    for line in &parsed.description {
        lines.push(format!(" * {}", line));
    }

    if !parsed.arguments.is_empty() {
        lines.push(" *".to_string());
        for (name, desc) in &parsed.arguments {
            lines.push(format!(" * @param {} - {}", name, desc));
        }
    }

    if let Some(ref returns) = parsed.returns {
        lines.push(format!(" * @returns {}", returns));
    }

    if !parsed.errors.is_empty() {
        for error in &parsed.errors {
            lines.push(format!(" * @throws {}", error));
        }
    }

    if !parsed.examples.is_empty() {
        lines.push(" *".to_string());
        lines.push(" * @example".to_string());
        let mut in_code_block = false;
        for line in &parsed.examples {
            let trimmed = line.trim();
            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
                lines.push(format!(" * {}", trimmed));
            }
        }
    }

    lines.push(" */".to_string());

    lines
        .iter()
        .map(|l| format!("{}{}", indent_str, l))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Generate a simple one-line doc comment for Rust (/// comment)
pub fn to_rust_doc_line(doc: &str) -> Option<String> {
    let parsed = ParsedRustDoc::parse(doc);
    parsed.description.first().cloned()
}

/// Transform Rust documentation to NAPI-RS compatible format.
/// NAPI-RS reads Rust doc comments and generates JSDoc in .d.ts.
/// Outputs multiple #[doc = "..."] attributes.
/// Automatically strips @no-binding directives from the output.
pub fn to_napi_doc(doc: &str, indent: usize) -> String {
    let cleaned_doc = clean_doc_for_binding(doc);
    let parsed = ParsedRustDoc::parse(&cleaned_doc);

    if parsed.is_empty() {
        return String::new();
    }

    let indent_str = " ".repeat(indent);
    let mut doc_lines: Vec<String> = Vec::new();

    for line in &parsed.description {
        doc_lines.push(line.clone());
    }

    if !parsed.arguments.is_empty() {
        doc_lines.push(String::new());
        doc_lines.push("# Arguments".to_string());
        doc_lines.push(String::new());
        for (name, desc) in &parsed.arguments {
            doc_lines.push(format!("* `{}` - {}", name, desc));
        }
    }

    if let Some(ref returns) = parsed.returns {
        doc_lines.push(String::new());
        doc_lines.push("# Returns".to_string());
        doc_lines.push(String::new());
        doc_lines.push(returns.clone());
    }

    if !parsed.errors.is_empty() {
        doc_lines.push(String::new());
        doc_lines.push("# Errors".to_string());
        doc_lines.push(String::new());
        for error in &parsed.errors {
            doc_lines.push(format!("* {}", error));
        }
    }

    doc_lines
        .iter()
        .map(|line| {
            let escaped = escape_doc_string(line);
            format!("{}#[doc = \"{}\"]", indent_str, escaped)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Escape string for use inside #[doc = "..."]
fn escape_doc_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_doc() {
        let doc = "Computes the derivative of an expression.";
        let parsed = ParsedRustDoc::parse(doc);
        assert_eq!(parsed.description.len(), 1);
        assert_eq!(
            parsed.description[0],
            "Computes the derivative of an expression."
        );
    }

    #[test]
    fn test_parse_with_arguments() {
        let doc = r#"Computes the derivative of an expression.

# Arguments

* `variable` - The variable to differentiate with respect to
* `order` - The order of differentiation"#;

        let parsed = ParsedRustDoc::parse(doc);
        assert_eq!(parsed.description.len(), 1);
        assert_eq!(parsed.arguments.len(), 2);
        assert_eq!(parsed.arguments[0].0, "variable");
        assert_eq!(
            parsed.arguments[0].1,
            "The variable to differentiate with respect to"
        );
        assert_eq!(parsed.arguments[1].0, "order");
    }

    #[test]
    fn test_parse_with_returns() {
        let doc = r#"Computes something.

# Returns

The computed result as an Expression."#;

        let parsed = ParsedRustDoc::parse(doc);
        assert_eq!(
            parsed.returns,
            Some("The computed result as an Expression.".to_string())
        );
    }

    #[test]
    fn test_to_python_docstring_simple() {
        let doc = "Computes the derivative.";
        let result = to_python_docstring(doc, 0);
        assert_eq!(result, "#[doc = \"Computes the derivative.\"]");
    }

    #[test]
    fn test_to_python_docstring_with_args() {
        let doc = r#"Computes the derivative of an expression.

# Arguments

* `variable` - The variable to differentiate with respect to"#;

        let result = to_python_docstring(doc, 0);
        assert!(result.contains("#[doc = \"Args:\"]"));
        assert!(result.contains("#[doc = \"    variable: The variable"));
    }

    #[test]
    fn test_to_python_docstring_escapes_quotes() {
        let doc = r#"Returns "hello" world."#;
        let result = to_python_docstring(doc, 0);
        assert_eq!(result, r#"#[doc = "Returns \"hello\" world."]"#);
    }

    #[test]
    fn test_to_python_docstring_escapes_backslashes() {
        let doc = r#"Use \n for newline."#;
        let result = to_python_docstring(doc, 0);
        assert_eq!(result, r#"#[doc = "Use \\n for newline."]"#);
    }

    #[test]
    fn test_to_python_docstring_multiline() {
        let doc = r#"First line.
Second line."#;
        let result = to_python_docstring(doc, 0);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "#[doc = \"First line.\"]");
        assert_eq!(lines[1], "#[doc = \"Second line.\"]");
    }

    #[test]
    fn test_to_python_docstring_strips_no_binding() {
        let doc = "Some documentation.\n\n@no-binding\n\nMore docs.";
        let result = to_python_docstring(doc, 0);
        assert!(!result.contains("@no-binding"));
        assert!(result.contains("Some documentation."));
        assert!(result.contains("More docs."));
    }

    #[test]
    fn test_to_jsdoc_simple() {
        let doc = "Computes the derivative.";
        let result = to_jsdoc(doc, 0);
        assert!(result.contains("/**"));
        assert!(result.contains(" * Computes the derivative."));
        assert!(result.contains(" */"));
    }

    #[test]
    fn test_to_jsdoc_with_params() {
        let doc = r#"Computes the derivative of an expression.

# Arguments

* `variable` - The variable to differentiate with respect to

# Returns

The derivative expression."#;

        let result = to_jsdoc(doc, 0);
        assert!(result.contains("@param variable - The variable"));
        assert!(result.contains("@returns The derivative expression."));
    }

    #[test]
    fn test_to_jsdoc_strips_no_binding() {
        let doc = "@no-binding - internal only\nSome description.";
        let result = to_jsdoc(doc, 0);
        assert!(!result.contains("@no-binding"));
        assert!(result.contains("Some description."));
    }
}

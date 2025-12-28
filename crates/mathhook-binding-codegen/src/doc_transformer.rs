//! Documentation transformation utilities for converting Rust doc comments
//! to Python docstrings (Google-style) and JSDoc format.

use crate::scanner::clean_doc_for_binding;

#[derive(Debug, Default)]
pub struct ParsedRustDoc {
    pub description: Vec<String>,
    pub arguments: Vec<(String, String)>,
    pub returns: Option<String>,
    pub errors: Vec<String>,
    pub examples: Vec<String>,
    pub panics: Vec<String>,
    pub safety: Vec<String>,
}

impl ParsedRustDoc {
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
            Section::Description => result.description = cleaned,
            Section::Arguments => result.arguments = Self::parse_argument_list(&cleaned),
            Section::Returns => {
                result.returns = if cleaned.is_empty() {
                    None
                } else {
                    Some(cleaned.join(" "))
                };
            }
            Section::Errors => result.errors = cleaned,
            Section::Examples => result.examples = lines.to_vec(),
            Section::Panics => result.panics = cleaned,
            Section::Safety => result.safety = cleaned,
        }
    }

    fn parse_argument_list(lines: &[String]) -> Vec<(String, String)> {
        lines
            .iter()
            .filter_map(|l| Self::parse_argument_line(l))
            .collect()
    }

    fn parse_argument_line(line: &str) -> Option<(String, String)> {
        let line = line.trim();
        if !line.starts_with('*') && !line.starts_with('-') {
            return None;
        }
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
///
/// Note: Rust examples are excluded from output. Language-specific example
/// transformation would require manual translation of Rust code to Python.
pub fn to_python_docstring(doc: &str, indent: usize) -> String {
    let cleaned_doc = clean_doc_for_binding(doc);
    let parsed = ParsedRustDoc::parse(&cleaned_doc);
    if parsed.is_empty() {
        return String::new();
    }

    let indent_str = " ".repeat(indent);
    let mut doc_lines: Vec<String> = Vec::new();

    doc_lines.extend(parsed.description.iter().cloned());

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

    doc_lines
        .iter()
        .map(|line| format!("{}#[doc = \"{}\"]", indent_str, escape_doc_string(line)))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Transform Rust documentation to JSDoc format.
/// Automatically strips @no-binding directives from the output.
///
/// Note: Rust examples are excluded from output. Language-specific example
/// transformation would require manual translation of Rust code to JavaScript.
pub fn to_jsdoc(doc: &str, indent: usize) -> String {
    let cleaned_doc = clean_doc_for_binding(doc);
    let parsed = ParsedRustDoc::parse(&cleaned_doc);
    let indent_str = " ".repeat(indent);

    let mut lines = vec!["/**".to_string()];
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

    for error in &parsed.errors {
        lines.push(format!(" * @throws {}", error));
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
    ParsedRustDoc::parse(doc).description.first().cloned()
}

/// Transform Rust documentation to NAPI-RS compatible format.
/// NAPI-RS reads Rust doc comments and generates JSDoc in .d.ts.
/// Outputs multiple #[doc = "..."] attributes.
/// Automatically strips @no-binding directives from the output.
///
/// Note: Rust examples are excluded from output. Language-specific example
/// transformation would require manual translation of Rust code to JavaScript/TypeScript.
pub fn to_napi_doc(doc: &str, indent: usize) -> String {
    let cleaned_doc = clean_doc_for_binding(doc);
    let parsed = ParsedRustDoc::parse(&cleaned_doc);

    if parsed.is_empty() {
        return String::new();
    }

    let indent_str = " ".repeat(indent);
    let mut doc_lines: Vec<String> = Vec::new();

    doc_lines.extend(parsed.description.iter().cloned());

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
        .map(|line| format!("{}#[doc = \"{}\"]", indent_str, escape_doc_string(line)))
        .collect::<Vec<_>>()
        .join("\n")
}

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
        let doc = "Desc.\n\n# Arguments\n\n* `variable` - Var desc\n* `order` - Order desc";
        let parsed = ParsedRustDoc::parse(doc);
        assert_eq!(parsed.arguments.len(), 2);
        assert_eq!(parsed.arguments[0].0, "variable");
        assert_eq!(parsed.arguments[0].1, "Var desc");
    }

    #[test]
    fn test_parse_with_returns() {
        let doc = "Desc.\n\n# Returns\n\nThe result.";
        let parsed = ParsedRustDoc::parse(doc);
        assert_eq!(parsed.returns, Some("The result.".to_string()));
    }

    #[test]
    fn test_to_python_docstring() {
        assert_eq!(
            to_python_docstring("Computes.", 0),
            "#[doc = \"Computes.\"]"
        );

        let doc = "Desc.\n\n# Arguments\n\n* `x` - X desc";
        let result = to_python_docstring(doc, 0);
        assert!(result.contains("#[doc = \"Args:\"]"));
        assert!(result.contains("x: X desc"));
    }

    #[test]
    fn test_to_python_docstring_escaping() {
        assert_eq!(
            to_python_docstring(r#"Returns "hello"."#, 0),
            r#"#[doc = "Returns \"hello\"."]"#
        );
        assert_eq!(
            to_python_docstring(r#"Use \n for newline."#, 0),
            r#"#[doc = "Use \\n for newline."]"#
        );
    }

    #[test]
    fn test_to_python_docstring_strips_no_binding() {
        let doc = "Docs.\n\n@no-binding\n\nMore.";
        let result = to_python_docstring(doc, 0);
        assert!(!result.contains("@no-binding"));
        assert!(result.contains("Docs."));
    }

    #[test]
    fn test_examples_excluded() {
        let doc = "Desc.\n\n# Examples\n\n```rust\nlet x = symbol!(x);\n```";
        let py = to_python_docstring(doc, 0);
        let js = to_jsdoc(doc, 0);
        let napi = to_napi_doc(doc, 0);
        assert!(!py.contains("Examples") && !py.contains("symbol!"));
        assert!(!js.contains("@example") && !js.contains("symbol!"));
        assert!(!napi.contains("Examples") && !napi.contains("symbol!"));
    }

    #[test]
    fn test_to_jsdoc() {
        let doc = "Computes.";
        let result = to_jsdoc(doc, 0);
        assert!(
            result.contains("/**") && result.contains(" * Computes.") && result.contains(" */")
        );

        let doc = "Desc.\n\n# Arguments\n\n* `x` - X\n\n# Returns\n\nResult.";
        let result = to_jsdoc(doc, 0);
        assert!(result.contains("@param x - X"));
        assert!(result.contains("@returns Result."));
    }

    #[test]
    fn test_to_jsdoc_strips_no_binding() {
        let doc = "@no-binding - internal\nDesc.";
        let result = to_jsdoc(doc, 0);
        assert!(!result.contains("@no-binding"));
        assert!(result.contains("Desc."));
    }
}

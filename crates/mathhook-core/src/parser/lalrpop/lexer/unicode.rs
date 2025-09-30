/// Unicode mathematical symbols and character handling
///
/// Provides efficient lookup and categorization of Unicode mathematical symbols
/// for high-performance lexical analysis.
use std::collections::HashMap;

/// Unicode mathematical symbols mapping
pub struct UnicodeSymbols {
    /// Symbol to constant mapping
    symbol_map: HashMap<char, &'static str>,
}

impl UnicodeSymbols {
    /// Create new Unicode symbols handler
    pub fn new() -> Self {
        let mut symbol_map = HashMap::new();

        // Mathematical constants
        symbol_map.insert('π', "pi");
        symbol_map.insert('∞', "infinity");
        symbol_map.insert('e', "e");
        symbol_map.insert('i', "i");
        symbol_map.insert('φ', "phi");
        symbol_map.insert('γ', "gamma");

        // Mathematical operators
        symbol_map.insert('±', "plusminus");
        symbol_map.insert('∓', "minusplus");
        symbol_map.insert('·', "cdot");
        symbol_map.insert('×', "times");
        symbol_map.insert('÷', "div");

        // Greek letters (commonly used in mathematics)
        symbol_map.insert('α', "alpha");
        symbol_map.insert('β', "beta");
        symbol_map.insert('δ', "delta");
        symbol_map.insert('ε', "epsilon");
        symbol_map.insert('θ', "theta");
        symbol_map.insert('λ', "lambda");
        symbol_map.insert('μ', "mu");
        symbol_map.insert('σ', "sigma");
        symbol_map.insert('ω', "omega");

        // Set theory symbols
        symbol_map.insert('∅', "emptyset");
        symbol_map.insert('∈', "in");
        symbol_map.insert('∉', "notin");
        symbol_map.insert('⊂', "subset");
        symbol_map.insert('⊃', "supset");
        symbol_map.insert('∪', "union");
        symbol_map.insert('∩', "intersection");

        Self { symbol_map }
    }

    /// Check if character is a mathematical symbol
    pub fn is_math_symbol(&self, ch: char) -> bool {
        self.symbol_map.contains_key(&ch)
    }

    /// Get symbol name for character
    pub fn get_symbol_name(&self, ch: char) -> Option<&'static str> {
        self.symbol_map.get(&ch).copied()
    }

    /// Check if character is a mathematical constant
    pub fn is_constant(&self, ch: char) -> bool {
        matches!(ch, 'π' | '∞' | 'e' | 'i' | 'φ' | 'γ')
    }

    /// Check if character is a mathematical operator
    pub fn is_operator(&self, ch: char) -> bool {
        matches!(ch, '±' | '∓' | '·' | '×' | '÷')
    }

    /// Check if character is a Greek letter
    pub fn is_greek_letter(&self, ch: char) -> bool {
        matches!(
            ch,
            'α' | 'β' | 'γ' | 'δ' | 'ε' | 'θ' | 'λ' | 'μ' | 'π' | 'σ' | 'φ' | 'ω'
        )
    }
}

impl Default for UnicodeSymbols {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if character requires multi-byte UTF-8 encoding
pub fn is_multibyte_char(ch: char) -> bool {
    ch.len_utf8() > 1
}

/// Get UTF-8 byte length of character
pub fn char_byte_len(ch: char) -> usize {
    ch.len_utf8()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_symbols() {
        let symbols = UnicodeSymbols::new();

        assert!(symbols.is_math_symbol('π'));
        assert!(symbols.is_math_symbol('∞'));
        assert!(symbols.is_constant('π'));
        assert!(symbols.is_operator('±'));
        assert!(symbols.is_greek_letter('α'));

        assert_eq!(symbols.get_symbol_name('π'), Some("pi"));
        assert_eq!(symbols.get_symbol_name('∞'), Some("infinity"));
    }

    #[test]
    fn test_multibyte_chars() {
        assert!(is_multibyte_char('π'));
        assert!(is_multibyte_char('∞'));
        assert!(!is_multibyte_char('a'));

        assert_eq!(char_byte_len('π'), 2);
        assert_eq!(char_byte_len('a'), 1);
    }
}

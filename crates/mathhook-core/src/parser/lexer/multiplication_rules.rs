//! Precomputed implicit multiplication rules matrix
//!
//! This module contains the ultra-fast precomputed matrix for determining
//! when implicit multiplication should be inserted between token types.

use super::token_maps::TokenType;

/// Precomputed implicit multiplication rules matrix for fast O(1) lookups
///
/// This matrix encodes all the rules for when implicit multiplication should
/// be inserted between different token types. Using a matrix allows for
/// constant-time lookups without any conditional logic.
///
/// Matrix indices correspond to TokenType enum values:
/// - 0: Number, 1: Identifier, 2: Constant, 3: GreekSymbol, 4: Function
/// - 5: LeftParen, 6: RightParen, 7: Operator, 8: LaTeXCommand, 9: Other
pub const IMPLICIT_MUL_MATRIX: [[bool; 10]; 10] = [
    // From\To:  Num  Id   Con  Grk  Fun  LP   RP   Op   LaT  Oth
    /* Number */
    [
        false, true, true, true, true, true, false, false, true, false,
    ],
    /* Identifier */
    [
        false, true, true, true, true, true, false, false, true, false,
    ],
    /* Constant */
    [
        false, true, true, true, true, true, false, false, true, false,
    ],
    /* GreekSymbol */
    [
        false, true, true, true, true, true, false, false, true, false,
    ],
    /* Function */
    [
        false, false, false, false, false, false, false, false, false, false,
    ],
    /* LeftParen */
    [
        false, false, false, false, false, false, false, false, false, false,
    ],
    /* RightParen */
    [
        false, true, true, true, true, true, false, false, true, false,
    ],
    /* Operator */
    [
        false, false, false, false, false, false, false, false, false, false,
    ],
    /* LaTeXCommand*/
    [
        false, true, true, true, true, true, false, false, true, false,
    ],
    /* Other */
    [
        false, false, false, false, false, false, false, false, false, false,
    ],
];

/// Ultra-fast multiplication check using precomputed matrix
#[inline]
pub fn should_insert_multiplication_fast(left: TokenType, right: TokenType) -> bool {
    IMPLICIT_MUL_MATRIX[left as usize][right as usize]
}

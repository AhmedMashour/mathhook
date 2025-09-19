//! Relation types for equations and inequalities

use serde::{Deserialize, Serialize};

/// Relation types for equations and inequalities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationType {
    /// Equal (=)
    Equal,
    /// Not equal (≠)
    NotEqual,
    /// Less than (<)
    Less,
    /// Greater than (>)
    Greater,
    /// Less than or equal (≤)
    LessEqual,
    /// Greater than or equal (≥)
    GreaterEqual,
    /// Approximately equal (≈)
    Approximately,
}

/// Direction for limits
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LimitDirection {
    /// Approach from both sides
    Both,
    /// Approach from the left
    Left,
    /// Approach from the right
    Right,
}

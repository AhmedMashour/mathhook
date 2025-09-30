/// Visitor patterns for Expression traversal
///
/// Provides efficient traversal patterns for Expression trees,
/// useful for analysis, transformation, and optimization.
use crate::core::Expression;

/// Visitor trait for Expression traversal
pub trait ExpressionVisitor {
    type Output;

    /// Visit an Expression node
    fn visit(&mut self, expr: &Expression) -> Self::Output;

    /// Visit children of an Expression
    fn visit_children(&mut self, expr: &Expression) {
        match expr {
            Expression::Add(terms) | Expression::Mul(terms) => {
                for term in terms.iter() {
                    self.visit(term);
                }
            }
            Expression::Pow(base, exp) => {
                self.visit(base);
                self.visit(exp);
            }
            Expression::Function { args, .. } => {
                for arg in args.iter() {
                    self.visit(arg);
                }
            }
            // Add other cases as needed
            _ => {}
        }
    }
}

/// Depth-first expression traversal
pub struct DepthFirstVisitor<F> {
    visit_fn: F,
}

impl<F> DepthFirstVisitor<F>
where
    F: FnMut(&Expression),
{
    /// Create a new depth-first visitor
    pub fn new(visit_fn: F) -> Self {
        Self { visit_fn }
    }
}

impl<F> ExpressionVisitor for DepthFirstVisitor<F>
where
    F: FnMut(&Expression),
{
    type Output = ();

    fn visit(&mut self, expr: &Expression) -> Self::Output {
        self.visit_children(expr);
        (self.visit_fn)(expr);
    }
}

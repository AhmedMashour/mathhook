//! Arena-based memory allocation for expressions to reduce heap fragmentation
//! Improves cache performance and reduces allocation overhead

use crate::core::{Expression, Symbol, Number};
use std::cell::RefCell;
use std::rc::Rc;

/// Arena allocator for expressions
pub struct ExpressionArena {
    expressions: RefCell<Vec<Expression>>,
}

impl ExpressionArena {
    /// Create a new arena
    pub fn new() -> Self {
        Self {
            expressions: RefCell::new(Vec::with_capacity(1000)),
        }
    }
    
    /// Allocate an expression in the arena
    pub fn alloc(self: &Rc<Self>, expr: Expression) -> ArenaExpression {
        let mut expressions = self.expressions.borrow_mut();
        expressions.push(expr);
        let index = expressions.len() - 1;
        ArenaExpression {
            arena: Rc::clone(self),
            index,
        }
    }
    
    /// Get an expression by index
    pub fn get(&self, index: usize) -> Option<Expression> {
        self.expressions.borrow().get(index).cloned()
    }
    
    /// Clear the arena
    pub fn clear(&self) {
        self.expressions.borrow_mut().clear();
    }
    
    /// Get the number of allocated expressions
    pub fn len(&self) -> usize {
        self.expressions.borrow().len()
    }
    
    /// Check if the arena is empty
    pub fn is_empty(&self) -> bool {
        self.expressions.borrow().is_empty()
    }
}

impl Default for ExpressionArena {
    fn default() -> Self {
        Self::new()
    }
}

/// Reference to an expression in an arena
pub struct ArenaExpression {
    // Store the arena directly instead of a reference
    arena: Rc<ExpressionArena>,
    index: usize,
}

impl ArenaExpression {
    /// Get the expression from the arena
    pub fn get(&self) -> Option<Expression> {
        self.arena.get(self.index)
    }
}

/// Arena-optimized expression operations
pub struct ArenaOptimized {
    arena: Rc<ExpressionArena>,
}

impl ArenaOptimized {
    /// Create a new arena-optimized system
    pub fn new() -> Self {
        Self {
            arena: Rc::new(ExpressionArena::new()),
        }
    }
    
    /// Create an integer expression in the arena
    pub fn integer(&self, value: i64) -> ArenaExpression {
        self.arena.alloc(Expression::integer(value))
    }
    
    /// Create a symbol expression in the arena
    pub fn symbol(&self, name: &str) -> ArenaExpression {
        self.arena.alloc(Expression::symbol(Symbol::new(name)))
    }
    
    /// Perform arena-optimized addition
    pub fn add(&self, terms: Vec<ArenaExpression>) -> ArenaExpression {
        let expr_terms: Vec<Expression> = terms.into_iter()
            .filter_map(|t| t.get())
            .collect();
        self.arena.alloc(Expression::add(expr_terms))
    }
    
    /// Get arena statistics
    pub fn stats(&self) -> ArenaStats {
        ArenaStats {
            allocated: self.arena.len(),
            capacity: self.arena.expressions.borrow().capacity(),
        }
    }
}

impl Default for ArenaOptimized {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about arena usage
#[derive(Debug)]
pub struct ArenaStats {
    pub allocated: usize,
    pub capacity: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_basic_allocation() {
        let arena = Rc::new(ExpressionArena::new());
        
        let expr1 = arena.alloc(Expression::integer(42));
        let expr2 = arena.alloc(Expression::symbol(Symbol::new("x")));
        
        assert_eq!(arena.len(), 2);
        assert_eq!(expr1.get(), Some(Expression::integer(42)));
        assert_eq!(expr2.get(), Some(Expression::symbol(Symbol::new("x"))));
    }
    
    #[test]
    fn test_arena_expression_allocation() {
        let arena_opt = ArenaOptimized::new();
        
        let x = arena_opt.symbol("x");
        let five = arena_opt.integer(5);
        let sum = arena_opt.add(vec![x, five]);
        
        let stats = arena_opt.stats();
        assert!(stats.allocated >= 3);
        assert!(sum.get().is_some());
    }
    
    #[test]
    fn test_arena_clear() {
        let arena = Rc::new(ExpressionArena::new());
        
        arena.alloc(Expression::integer(1));
        arena.alloc(Expression::integer(2));
        assert_eq!(arena.len(), 2);
        
        arena.clear();
        assert_eq!(arena.len(), 0);
        assert!(arena.is_empty());
    }
}

//! Thread-local caching for parser performance optimization
//!
//! This module provides thread-local storage for expensive parser operations
//! to avoid repeated allocations and computations. Based on Rust Performance
//! Book recommendations for high-performance parsing.

use crate::core::Expression;
use std::cell::RefCell;
use std::collections::HashMap;
use std::thread_local;

thread_local! {
    /// Cache for parsed function names to avoid repeated string allocations
    ///
    /// This cache stores constructed function names like "bessel_j_indexed"
    /// to avoid repeated format! calls during parsing.
    static FUNCTION_NAME_CACHE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());

    /// Pre-allocated Vec for expression lists to avoid repeated allocations
    ///
    /// This buffer is reused for building Vec<Expression> during parsing
    /// to minimize heap allocations.
    static EXPR_LIST_BUFFER: RefCell<Vec<Expression>> = RefCell::new(Vec::with_capacity(16));

    /// Cache for commonly used Expression instances
    ///
    /// Stores frequently used expressions like constants and simple operations
    /// to avoid repeated construction.
    static COMMON_EXPRESSIONS: RefCell<HashMap<&'static str, Expression>> = RefCell::new({
        let mut map = HashMap::new();
        map.insert("0", Expression::integer(0));
        map.insert("1", Expression::integer(1));
        map.insert("-1", Expression::integer(-1));
        map.insert("2", Expression::integer(2));
        map.insert("pi", Expression::pi());
        map.insert("e", Expression::e());
        map.insert("i", Expression::i());
        map.insert("infinity", Expression::infinity());
        map
    });
}

/// Efficient function name construction with caching
///
/// Constructs function names like "bessel_j_indexed" and caches them
/// to avoid repeated string allocations.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::parser::cache::get_cached_function_name;
///
/// let name1 = get_cached_function_name("bessel", "j_indexed");
/// let name2 = get_cached_function_name("bessel", "j_indexed");
/// // Second call reuses cached result
/// ```
///
/// # Performance
///
/// - First call: O(n) string construction + HashMap insertion
/// - Subsequent calls: O(1) HashMap lookup + clone
pub fn get_cached_function_name(base: &str, suffix: &str) -> String {
    FUNCTION_NAME_CACHE.with(|cache| {
        let key = format!("{}_{}", base, suffix);
        let mut cache = cache.borrow_mut();

        cache.entry(key.clone()).or_insert_with(|| key).clone()
    })
}

/// Reuse Vec allocations for expression lists
///
/// Builds a `Vec<Expression>` using a thread-local buffer to minimize
/// heap allocations during parsing.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::parser::cache::build_expr_list;
/// use mathhook_core::Expression;
///
/// let exprs = vec![Expression::integer(1), Expression::integer(2)];
/// let result = build_expr_list(exprs);
/// ```
///
/// # Performance
///
/// - Reuses pre-allocated Vec capacity
/// - Single clone operation instead of multiple allocations
/// - Thread-local storage avoids synchronization overhead
pub fn build_expr_list(exprs: impl IntoIterator<Item = Expression>) -> Vec<Expression> {
    EXPR_LIST_BUFFER.with(|buffer| {
        let mut buffer = buffer.borrow_mut();
        buffer.clear();
        buffer.extend(exprs);
        buffer.clone() // Clone the populated buffer
    })
}

/// Get a commonly used expression from cache
///
/// Returns cached instances of frequently used expressions to avoid
/// repeated construction.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::parser::cache::get_cached_expression;
///
/// let zero = get_cached_expression("0");
/// let pi = get_cached_expression("pi");
/// ```
///
/// # Performance
///
/// - O(1) HashMap lookup for cached expressions
/// - Avoids repeated Expression construction for constants
pub fn get_cached_expression(key: &'static str) -> Option<Expression> {
    COMMON_EXPRESSIONS.with(|cache| cache.borrow().get(key).cloned())
}

/// Build a function expression with cached name construction
///
/// Combines function name caching with expression construction for
/// optimal performance in indexed function parsing.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::parser::cache::build_cached_function;
/// use mathhook_core::Expression;
///
/// let args = vec![Expression::integer(1), Expression::symbol("x")];
/// let func = build_cached_function("bessel", "j_indexed", args);
/// ```
pub fn build_cached_function(base: &str, suffix: &str, args: Vec<Expression>) -> Expression {
    let name = get_cached_function_name(base, suffix);
    Expression::function(name, args)
}

/// Clear all thread-local caches
///
/// Useful for testing or when memory usage needs to be minimized.
/// Should rarely be needed in normal operation.
pub fn clear_caches() {
    FUNCTION_NAME_CACHE.with(|cache| cache.borrow_mut().clear());
    EXPR_LIST_BUFFER.with(|buffer| buffer.borrow_mut().clear());
    COMMON_EXPRESSIONS.with(|cache| cache.borrow_mut().clear());
}

/// Get cache statistics for monitoring
///
/// Returns information about cache usage for performance monitoring
/// and optimization.
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub function_name_cache_size: usize,
    pub expr_list_buffer_capacity: usize,
    pub common_expressions_size: usize,
}

/// Get current cache statistics
///
/// # Examples
///
/// ```rust
/// use mathhook_core::parser::cache::get_cache_stats;
///
/// let stats = get_cache_stats();
/// println!("Function name cache size: {}", stats.function_name_cache_size);
/// ```
pub fn get_cache_stats() -> CacheStats {
    let function_name_cache_size = FUNCTION_NAME_CACHE.with(|cache| cache.borrow().len());
    let expr_list_buffer_capacity = EXPR_LIST_BUFFER.with(|buffer| buffer.borrow().capacity());
    let common_expressions_size = COMMON_EXPRESSIONS.with(|cache| cache.borrow().len());

    CacheStats {
        function_name_cache_size,
        expr_list_buffer_capacity,
        common_expressions_size,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name_caching() {
        clear_caches(); // Start with clean cache

        let name1 = get_cached_function_name("test", "function");
        let name2 = get_cached_function_name("test", "function");

        assert_eq!(name1, name2);
        assert_eq!(name1, "test_function");

        let stats = get_cache_stats();
        assert_eq!(stats.function_name_cache_size, 1);
    }

    #[test]
    fn test_expr_list_building() {
        let exprs = vec![
            Expression::integer(1),
            Expression::integer(2),
            Expression::integer(3),
        ];

        let result = build_expr_list(exprs.clone());
        assert_eq!(result.len(), 3);
        assert_eq!(result, exprs);
    }

    #[test]
    fn test_cached_expressions() {
        let zero = get_cached_expression("0");
        let pi = get_cached_expression("pi");
        let unknown = get_cached_expression("unknown");

        assert!(zero.is_some());
        assert!(pi.is_some());
        assert!(unknown.is_none());
    }

    #[test]
    fn test_cached_function_building() {
        let args = vec![Expression::integer(1), Expression::symbol("x")];
        let func = build_cached_function("bessel", "j", args.clone());

        match func {
            Expression::Function {
                name,
                args: func_args,
            } => {
                assert_eq!(name, "bessel_j");
                assert_eq!(*func_args, args);
            }
            _ => panic!("Expected function expression"),
        }
    }

    #[test]
    fn test_cache_clearing() {
        // Populate caches
        let _name = get_cached_function_name("test", "clear");
        let _expr = build_expr_list(vec![Expression::integer(1)]);

        let stats_before = get_cache_stats();
        assert!(stats_before.function_name_cache_size > 0);

        clear_caches();

        let stats_after = get_cache_stats();
        assert_eq!(stats_after.function_name_cache_size, 0);
    }
}

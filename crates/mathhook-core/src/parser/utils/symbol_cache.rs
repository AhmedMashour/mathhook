use crate::core::Symbol;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static SYMBOL_CACHE: RefCell<HashMap<String, Symbol>> = RefCell::new(HashMap::new());
}

/// Get or create a symbol using the thread-local symbol cache
///
/// This provides better performance than a global mutex-protected cache
/// while still ensuring symbol consistency within a parsing context.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::parser::utils::symbol_cache::get_or_create_symbol;
///
/// let x1 = get_or_create_symbol("x");
/// let x2 = get_or_create_symbol("x");
/// assert_eq!(x1.name(), x2.name());
/// ```
pub fn get_or_create_symbol(name: &str) -> Symbol {
    SYMBOL_CACHE.with(|cache| {
        let mut cache_map = cache.borrow_mut();
        if let Some(symbol) = cache_map.get(name) {
            symbol.clone()
        } else {
            let symbol = Symbol::new(name);
            cache_map.insert(name.to_string(), symbol.clone());
            symbol
        }
    })
}

/// Clear all symbols from the thread-local cache
pub fn clear_symbol_cache() {
    SYMBOL_CACHE.with(|cache| {
        cache.borrow_mut().clear();
    });
}

/// Get all symbol names from the thread-local cache
/// # Examples
/// ```rust
/// use mathhook_core::parser::utils::symbol_cache::get_symbol_names;
/// let names = get_symbol_names();
/// assert!(names.contains(&"x".to_string()));
/// ```
pub fn get_symbol_names() -> Vec<String> {
    SYMBOL_CACHE.with(|cache| cache.borrow().keys().cloned().collect())
}

/// Check if a symbol exists in the thread-local cache
/// # Examples
/// ```rust
/// use mathhook_core::parser::utils::symbol_cache::has_symbol;
/// assert!(has_symbol("x"));
/// assert!(!has_symbol("y"));
/// ```
pub fn has_symbol(name: &str) -> bool {
    SYMBOL_CACHE.with(|cache| cache.borrow().contains_key(name))
}

/// Get the number of symbols in the thread-local cache
/// # Examples
/// ```rust
/// use mathhook_core::parser::utils::symbol_cache::symbol_count;
/// assert_eq!(symbol_count(), 0);
/// ```
pub fn symbol_count() -> usize {
    SYMBOL_CACHE.with(|cache| cache.borrow().len())
}

/// Execute a closure with a clean symbol cache, restoring the previous state afterward
///
/// This is useful for parsing in isolated contexts.
/// # Examples
/// ```rust
/// use mathhook_core::parser::utils::symbol_cache::with_clean_cache;
/// with_clean_cache(|| {
///     assert_eq!(symbol_count(), 0);
/// });
/// ```
pub fn with_clean_cache<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    // Save current cache
    let saved_cache = SYMBOL_CACHE.with(|cache| cache.borrow().clone());

    // Clear cache for clean parsing
    clear_symbol_cache();

    // Execute the closure
    let result = f();

    // Restore previous cache
    SYMBOL_CACHE.with(|cache| {
        *cache.borrow_mut() = saved_cache;
    });

    result
}

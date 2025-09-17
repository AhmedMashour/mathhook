//! Polynomial Computation Cache
//!
//! Thread-local LRU cache for expensive polynomial computations.
//! Uses side-table design to preserve 32-byte Expression size constraint.
//!
//! The cache implements a proper LRU (Least Recently Used) eviction policy
//! by tracking access order for each cached entry. When the cache reaches
//! capacity, the least recently accessed entries are evicted first.

use std::cell::RefCell;
use std::collections::HashMap;

use super::poly::IntPoly;
use crate::core::Symbol;

/// Access counter for LRU tracking
/// Higher values indicate more recent access
static GLOBAL_ACCESS_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn next_access_time() -> u64 {
    GLOBAL_ACCESS_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// Entry wrapper that tracks last access time for LRU eviction
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    value: T,
    last_access: u64,
}

impl<T> CacheEntry<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            last_access: next_access_time(),
        }
    }

    fn touch(&mut self) {
        self.last_access = next_access_time();
    }
}

/// Thread-local polynomial computation cache
///
/// Caches expensive computations like degree, classification, and content
/// using expression pointer hash as key. Implements true LRU eviction
/// by tracking access times for each entry.
///
/// # Design
///
/// Uses pointer-based hashing since `Expression` doesn't implement `Hash`.
/// The cache is thread-local to avoid synchronization overhead.
pub struct PolynomialCache {
    /// Degree cache: expression_hash -> (variable_name -> degree)
    degree_cache: HashMap<u64, CacheEntry<HashMap<String, i64>>>,
    /// Classification cache: expression_hash -> classification
    classification_cache: HashMap<u64, CacheEntry<CachedClassification>>,
    /// Leading coefficient cache: expression_hash -> (variable_name -> coeff_hash)
    leading_coeff_cache: HashMap<u64, CacheEntry<HashMap<String, u64>>>,
    /// Content cache: expression_hash -> (variable_name -> content_hash)
    content_cache: HashMap<u64, CacheEntry<HashMap<String, u64>>>,
    /// IntPoly cache: expression_hash -> (IntPoly, Symbol)
    /// Caches Expression → IntPoly conversions to eliminate repeated bridging
    intpoly_cache: HashMap<u64, CacheEntry<(IntPoly, Symbol)>>,
    /// Maximum cache entries per cache type
    max_entries: usize,
    /// Cache hit counter for statistics
    hits: u64,
    /// Cache miss counter for statistics
    misses: u64,
    /// IntPoly-specific hit counter
    intpoly_hits: u64,
    /// IntPoly-specific miss counter
    intpoly_misses: u64,
}

/// Cached classification result
#[derive(Debug, Clone)]
pub enum CachedClassification {
    Integer,
    Rational,
    Univariate {
        var: String,
        degree: i64,
    },
    Multivariate {
        vars: Vec<String>,
        total_degree: i64,
    },
    RationalFunction,
    Transcendental,
    Symbolic,
}

impl PolynomialCache {
    /// Create a new cache with default capacity (1024 entries per cache type)
    pub fn new() -> Self {
        Self {
            degree_cache: HashMap::new(),
            classification_cache: HashMap::new(),
            leading_coeff_cache: HashMap::new(),
            content_cache: HashMap::new(),
            intpoly_cache: HashMap::new(),
            max_entries: 1024,
            hits: 0,
            misses: 0,
            intpoly_hits: 0,
            intpoly_misses: 0,
        }
    }

    /// Create a new cache with custom capacity
    pub fn with_capacity(max_entries: usize) -> Self {
        Self {
            degree_cache: HashMap::new(),
            classification_cache: HashMap::new(),
            leading_coeff_cache: HashMap::new(),
            content_cache: HashMap::new(),
            intpoly_cache: HashMap::new(),
            max_entries,
            hits: 0,
            misses: 0,
            intpoly_hits: 0,
            intpoly_misses: 0,
        }
    }

    /// Get cached degree for expression and variable
    pub fn get_degree(&mut self, expr_hash: u64, var: &str) -> Option<i64> {
        if let Some(entry) = self.degree_cache.get_mut(&expr_hash) {
            entry.touch();
            if let Some(&degree) = entry.value.get(var) {
                self.hits += 1;
                return Some(degree);
            }
        }
        self.misses += 1;
        None
    }

    /// Cache degree for expression and variable
    pub fn set_degree(&mut self, expr_hash: u64, var: &str, degree: i64) {
        self.maybe_evict_lru(&CacheType::Degree);
        self.degree_cache
            .entry(expr_hash)
            .or_insert_with(|| CacheEntry::new(HashMap::new()))
            .value
            .insert(var.to_owned(), degree);
    }

    /// Get cached classification for expression
    pub fn get_classification(&mut self, expr_hash: u64) -> Option<CachedClassification> {
        if let Some(entry) = self.classification_cache.get_mut(&expr_hash) {
            entry.touch();
            self.hits += 1;
            return Some(entry.value.clone());
        }
        self.misses += 1;
        None
    }

    /// Cache classification for expression
    pub fn set_classification(&mut self, expr_hash: u64, classification: CachedClassification) {
        self.maybe_evict_lru(&CacheType::Classification);
        self.classification_cache
            .insert(expr_hash, CacheEntry::new(classification));
    }

    /// Get cached leading coefficient hash for expression and variable
    pub fn get_leading_coeff(&mut self, expr_hash: u64, var: &str) -> Option<u64> {
        if let Some(entry) = self.leading_coeff_cache.get_mut(&expr_hash) {
            entry.touch();
            if let Some(&coeff_hash) = entry.value.get(var) {
                self.hits += 1;
                return Some(coeff_hash);
            }
        }
        self.misses += 1;
        None
    }

    /// Cache leading coefficient hash for expression and variable
    pub fn set_leading_coeff(&mut self, expr_hash: u64, var: &str, coeff_hash: u64) {
        self.maybe_evict_lru(&CacheType::LeadingCoeff);
        self.leading_coeff_cache
            .entry(expr_hash)
            .or_insert_with(|| CacheEntry::new(HashMap::new()))
            .value
            .insert(var.to_owned(), coeff_hash);
    }

    /// Get cached content hash for expression and variable
    pub fn get_content(&mut self, expr_hash: u64, var: &str) -> Option<u64> {
        if let Some(entry) = self.content_cache.get_mut(&expr_hash) {
            entry.touch();
            if let Some(&content_hash) = entry.value.get(var) {
                self.hits += 1;
                return Some(content_hash);
            }
        }
        self.misses += 1;
        None
    }

    /// Cache content hash for expression and variable
    pub fn set_content(&mut self, expr_hash: u64, var: &str, content_hash: u64) {
        self.maybe_evict_lru(&CacheType::Content);
        self.content_cache
            .entry(expr_hash)
            .or_insert_with(|| CacheEntry::new(HashMap::new()))
            .value
            .insert(var.to_owned(), content_hash);
    }

    /// Get cached IntPoly for expression
    ///
    /// Returns the cached IntPoly and variable if available.
    /// This eliminates repeated Expression → IntPoly conversions.
    pub fn get_intpoly(&mut self, expr_hash: u64) -> Option<(IntPoly, Symbol)> {
        if let Some(entry) = self.intpoly_cache.get_mut(&expr_hash) {
            entry.touch();
            self.intpoly_hits += 1;
            return Some(entry.value.clone());
        }
        self.intpoly_misses += 1;
        None
    }

    /// Cache IntPoly for expression
    ///
    /// Caches the IntPoly representation along with its variable.
    pub fn set_intpoly(&mut self, expr_hash: u64, poly: IntPoly, var: Symbol) {
        self.maybe_evict_lru(&CacheType::IntPoly);
        self.intpoly_cache
            .insert(expr_hash, CacheEntry::new((poly, var)));
    }

    /// Clear all caches
    pub fn clear(&mut self) {
        self.degree_cache.clear();
        self.classification_cache.clear();
        self.leading_coeff_cache.clear();
        self.content_cache.clear();
        self.intpoly_cache.clear();
        self.hits = 0;
        self.misses = 0;
        self.intpoly_hits = 0;
        self.intpoly_misses = 0;
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let total_hits = self.hits + self.intpoly_hits;
        let total_misses = self.misses + self.intpoly_misses;
        CacheStats {
            degree_entries: self.degree_cache.len(),
            classification_entries: self.classification_cache.len(),
            leading_coeff_entries: self.leading_coeff_cache.len(),
            content_entries: self.content_cache.len(),
            intpoly_entries: self.intpoly_cache.len(),
            hits: total_hits,
            misses: total_misses,
            intpoly_hits: self.intpoly_hits,
            intpoly_misses: self.intpoly_misses,
            hit_rate: if total_hits + total_misses > 0 {
                total_hits as f64 / (total_hits + total_misses) as f64
            } else {
                0.0
            },
            intpoly_hit_rate: if self.intpoly_hits + self.intpoly_misses > 0 {
                self.intpoly_hits as f64 / (self.intpoly_hits + self.intpoly_misses) as f64
            } else {
                0.0
            },
        }
    }

    /// Evict least recently used entries from a specific cache
    fn maybe_evict_lru(&mut self, cache_type: &CacheType) {
        match cache_type {
            CacheType::Degree => {
                if self.degree_cache.len() >= self.max_entries {
                    self.evict_lru_from_degree_cache();
                }
            }
            CacheType::Classification => {
                if self.classification_cache.len() >= self.max_entries {
                    self.evict_lru_from_classification_cache();
                }
            }
            CacheType::LeadingCoeff => {
                if self.leading_coeff_cache.len() >= self.max_entries {
                    self.evict_lru_from_leading_coeff_cache();
                }
            }
            CacheType::Content => {
                if self.content_cache.len() >= self.max_entries {
                    self.evict_lru_from_content_cache();
                }
            }
            CacheType::IntPoly => {
                if self.intpoly_cache.len() >= self.max_entries {
                    self.evict_lru_from_intpoly_cache();
                }
            }
        }
    }

    fn evict_lru_from_degree_cache(&mut self) {
        let to_remove = self.max_entries / 4;
        let mut entries: Vec<_> = self
            .degree_cache
            .iter()
            .map(|(k, v)| (*k, v.last_access))
            .collect();
        entries.sort_by_key(|(_, access)| *access);

        for (key, _) in entries.into_iter().take(to_remove) {
            self.degree_cache.remove(&key);
        }
    }

    fn evict_lru_from_classification_cache(&mut self) {
        let to_remove = self.max_entries / 4;
        let mut entries: Vec<_> = self
            .classification_cache
            .iter()
            .map(|(k, v)| (*k, v.last_access))
            .collect();
        entries.sort_by_key(|(_, access)| *access);

        for (key, _) in entries.into_iter().take(to_remove) {
            self.classification_cache.remove(&key);
        }
    }

    fn evict_lru_from_leading_coeff_cache(&mut self) {
        let to_remove = self.max_entries / 4;
        let mut entries: Vec<_> = self
            .leading_coeff_cache
            .iter()
            .map(|(k, v)| (*k, v.last_access))
            .collect();
        entries.sort_by_key(|(_, access)| *access);

        for (key, _) in entries.into_iter().take(to_remove) {
            self.leading_coeff_cache.remove(&key);
        }
    }

    fn evict_lru_from_content_cache(&mut self) {
        let to_remove = self.max_entries / 4;
        let mut entries: Vec<_> = self
            .content_cache
            .iter()
            .map(|(k, v)| (*k, v.last_access))
            .collect();
        entries.sort_by_key(|(_, access)| *access);

        for (key, _) in entries.into_iter().take(to_remove) {
            self.content_cache.remove(&key);
        }
    }

    fn evict_lru_from_intpoly_cache(&mut self) {
        let to_remove = self.max_entries / 4;
        let mut entries: Vec<_> = self
            .intpoly_cache
            .iter()
            .map(|(k, v)| (*k, v.last_access))
            .collect();
        entries.sort_by_key(|(_, access)| *access);

        for (key, _) in entries.into_iter().take(to_remove) {
            self.intpoly_cache.remove(&key);
        }
    }
}

/// Internal enum to identify cache types for eviction
enum CacheType {
    Degree,
    Classification,
    LeadingCoeff,
    Content,
    IntPoly,
}

/// Cache statistics for monitoring
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub degree_entries: usize,
    pub classification_entries: usize,
    pub leading_coeff_entries: usize,
    pub content_entries: usize,
    pub intpoly_entries: usize,
    pub hits: u64,
    pub misses: u64,
    pub intpoly_hits: u64,
    pub intpoly_misses: u64,
    pub hit_rate: f64,
    pub intpoly_hit_rate: f64,
}

impl Default for PolynomialCache {
    fn default() -> Self {
        Self::new()
    }
}

// Thread-local cache instance
thread_local! {
    static CACHE: RefCell<PolynomialCache> = RefCell::new(PolynomialCache::new());
}

/// Access the thread-local polynomial cache
pub fn with_cache<F, R>(f: F) -> R
where
    F: FnOnce(&mut PolynomialCache) -> R,
{
    CACHE.with(|cache| f(&mut cache.borrow_mut()))
}

/// Clear the thread-local polynomial cache
pub fn clear_cache() {
    with_cache(|cache| cache.clear());
}

/// Get statistics from the thread-local polynomial cache
pub fn cache_stats() -> CacheStats {
    with_cache(|cache| cache.stats())
}

/// Get or compute IntPoly representation with caching
///
/// This is the main entry point for eliminating internal bridging.
/// It computes Expression → IntPoly once and caches the result.
/// Subsequent calls for the same expression hit the cache.
///
/// # Arguments
/// * `expr` - The Expression to convert
/// * `hash` - Pre-computed structural hash of the expression
/// * `compute_fn` - Function to compute IntPoly if not cached
///
/// # Returns
/// `Some((IntPoly, Symbol))` if conversion succeeds, `None` otherwise
pub fn get_or_compute_intpoly<F>(expr_hash: u64, compute_fn: F) -> Option<(IntPoly, Symbol)>
where
    F: FnOnce() -> Option<(IntPoly, Symbol)>,
{
    with_cache(|cache| {
        if let Some(cached) = cache.get_intpoly(expr_hash) {
            return Some(cached);
        }

        if let Some((poly, var)) = compute_fn() {
            cache.set_intpoly(expr_hash, poly.clone(), var.clone());
            Some((poly, var))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_degree() {
        let mut cache = PolynomialCache::new();

        cache.set_degree(12345, "x", 5);
        assert_eq!(cache.get_degree(12345, "x"), Some(5));
        assert_eq!(cache.get_degree(12345, "y"), None);
        assert_eq!(cache.get_degree(99999, "x"), None);
    }

    #[test]
    fn test_cache_classification() {
        let mut cache = PolynomialCache::new();

        cache.set_classification(
            12345,
            CachedClassification::Univariate {
                var: "x".to_string(),
                degree: 3,
            },
        );

        let result = cache.get_classification(12345);
        assert!(matches!(
            result,
            Some(CachedClassification::Univariate { .. })
        ));
    }

    #[test]
    fn test_thread_local_cache() {
        with_cache(|cache| {
            cache.set_degree(111, "x", 2);
        });

        let degree = with_cache(|cache| cache.get_degree(111, "x"));
        assert_eq!(degree, Some(2));
    }

    #[test]
    fn test_cache_lru_eviction() {
        let mut cache = PolynomialCache::with_capacity(10);

        for i in 0..15 {
            cache.set_degree(i, "x", i as i64);
        }

        let stats = cache.stats();
        assert!(
            stats.degree_entries <= 10,
            "Cache should have evicted entries"
        );
    }

    #[test]
    fn test_cache_hit_tracking() {
        let mut cache = PolynomialCache::new();

        cache.set_degree(123, "x", 5);

        let _ = cache.get_degree(123, "x");
        let _ = cache.get_degree(123, "y");
        let _ = cache.get_degree(999, "x");

        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 2);
    }

    #[test]
    fn test_cache_leading_coeff() {
        let mut cache = PolynomialCache::new();

        cache.set_leading_coeff(12345, "x", 999);
        assert_eq!(cache.get_leading_coeff(12345, "x"), Some(999));
        assert_eq!(cache.get_leading_coeff(12345, "y"), None);
    }

    #[test]
    fn test_cache_content() {
        let mut cache = PolynomialCache::new();

        cache.set_content(12345, "x", 777);
        assert_eq!(cache.get_content(12345, "x"), Some(777));
        assert_eq!(cache.get_content(12345, "y"), None);
    }

    #[test]
    fn test_cache_stats_helper() {
        clear_cache();
        with_cache(|cache| {
            cache.set_degree(1, "x", 1);
            cache.set_classification(2, CachedClassification::Integer);
        });

        let stats = cache_stats();
        assert_eq!(stats.degree_entries, 1);
        assert_eq!(stats.classification_entries, 1);
    }

    #[test]
    fn test_intpoly_cache() {
        use crate::symbol;

        let mut cache = PolynomialCache::new();
        let x = symbol!(x);
        let poly = IntPoly::from_coeffs(vec![1, 2, 3]);

        cache.set_intpoly(12345, poly.clone(), x.clone());
        let cached = cache.get_intpoly(12345);
        assert!(cached.is_some());
        let (p, v) = cached.unwrap();
        assert_eq!(p, poly);
        assert_eq!(v, x);

        assert!(cache.get_intpoly(99999).is_none());
    }

    #[test]
    fn test_get_or_compute_intpoly() {
        use crate::symbol;

        clear_cache();
        let x = symbol!(x);
        let poly = IntPoly::from_coeffs(vec![1, 2, 3]);
        let hash = 54321u64;

        let mut call_count = 0;

        let result1 = get_or_compute_intpoly(hash, || {
            call_count += 1;
            Some((poly.clone(), x.clone()))
        });
        assert!(result1.is_some());
        assert_eq!(call_count, 1);

        let result2 = get_or_compute_intpoly(hash, || {
            call_count += 1;
            Some((poly.clone(), x.clone()))
        });
        assert!(result2.is_some());
        assert_eq!(call_count, 1);

        let stats = cache_stats();
        assert!(stats.intpoly_hits >= 1);
    }
}

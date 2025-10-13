//! Performance optimization strategy for MathHook library usage
//!
//! This module defines the smart integration of SIMD, memoization, and concurrency
//! optimized for Python/Node.js bindings and interactive usage patterns.

use crate::core::Expression;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

/// Binding context for performance optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingContext {
    /// Native Rust application (maximum performance)
    Native,
    /// Python binding (GIL constraints)
    Python,
    /// Node.js binding (single-threaded model)
    NodeJs,
    /// WebAssembly binding (memory constraints)
    WebAssembly,
    /// Custom binding (user-defined strategy)
    Custom,
}

/// Performance configuration for different usage contexts
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Enable SIMD operations for bulk numeric operations
    pub simd_enabled: bool,
    /// Minimum size threshold for SIMD activation
    pub simd_threshold: usize,
    /// Enable memoization for expensive operations
    pub memoization_enabled: bool,
    /// Maximum cache size (number of entries)
    pub cache_size_limit: usize,
    /// Enable parallel processing for large operations
    pub parallel_enabled: bool,
    /// Minimum size threshold for parallel processing
    pub parallel_threshold: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            simd_enabled: true,
            simd_threshold: 50, // Based on benchmark results
            memoization_enabled: true,
            cache_size_limit: 10000,  // ~10MB for typical expressions
            parallel_enabled: false,  // Conservative default for bindings
            parallel_threshold: 1000, // Only for very large operations
        }
    }
}

impl PerformanceConfig {
    /// Configuration optimized for Python bindings (GIL-aware)
    pub fn python_optimized() -> Self {
        Self {
            simd_enabled: true,
            simd_threshold: 50,
            memoization_enabled: true,
            cache_size_limit: 50000, // Larger cache for interactive sessions
            parallel_enabled: false, // Avoid GIL contention
            parallel_threshold: usize::MAX, // Effectively disabled
        }
    }

    /// Configuration optimized for Node.js bindings
    pub fn nodejs_optimized() -> Self {
        Self {
            simd_enabled: true,
            simd_threshold: 50,
            memoization_enabled: true,
            cache_size_limit: 20000, // Medium cache for server usage
            parallel_enabled: true,  // Node can benefit from threads
            parallel_threshold: 500, // Lower threshold for server workloads
        }
    }

    /// Configuration for high-performance native usage
    pub fn native_optimized() -> Self {
        Self {
            simd_enabled: true,
            simd_threshold: 20, // More aggressive SIMD
            memoization_enabled: true,
            cache_size_limit: 100000, // Large cache for native apps
            parallel_enabled: true,
            parallel_threshold: 100, // Aggressive parallelization
        }
    }

    /// Configuration for WebAssembly bindings (memory-constrained)
    pub fn wasm_optimized() -> Self {
        Self {
            simd_enabled: true,  // WASM supports SIMD
            simd_threshold: 100, // Higher threshold due to overhead
            memoization_enabled: true,
            cache_size_limit: 1000,  // Small cache due to memory limits
            parallel_enabled: false, // WASM is single-threaded
            parallel_threshold: usize::MAX,
        }
    }

    /// Factory method to create configuration based on binding context
    pub fn for_binding(context: BindingContext) -> Self {
        match context {
            BindingContext::Native => Self::native_optimized(),
            BindingContext::Python => Self::python_optimized(),
            BindingContext::NodeJs => Self::nodejs_optimized(),
            BindingContext::WebAssembly => Self::wasm_optimized(),
            BindingContext::Custom => Self::default(),
        }
    }
}

/// Smart performance optimizer that decides when to use different strategies
pub struct PerformanceOptimizer {
    pub config: PerformanceConfig,
    simplify_cache: Arc<RwLock<HashMap<u64, Expression>>>,
    derivative_cache: Arc<RwLock<HashMap<u64, Expression>>>,
}

impl PerformanceOptimizer {
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            config,
            simplify_cache: Arc::new(RwLock::new(HashMap::new())),
            derivative_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Determine if SIMD should be used for bulk operations
    pub fn should_use_simd(&self, operation_size: usize) -> bool {
        self.config.simd_enabled && operation_size >= self.config.simd_threshold
    }

    /// Determine if parallel processing should be used
    pub fn should_use_parallel(&self, operation_size: usize) -> bool {
        self.config.parallel_enabled && operation_size >= self.config.parallel_threshold
    }

    /// Check cache for memoized result
    pub fn get_cached_simplify(&self, expr_hash: u64) -> Option<Expression> {
        if !self.config.memoization_enabled {
            return None;
        }

        self.simplify_cache.read().ok()?.get(&expr_hash).cloned()
    }

    /// Cache a simplification result
    pub fn cache_simplify(&self, expr_hash: u64, result: Expression) {
        if !self.config.memoization_enabled {
            return;
        }

        if let Ok(mut cache) = self.simplify_cache.write() {
            // Implement LRU eviction if cache is full
            if cache.len() >= self.config.cache_size_limit {
                // Simple eviction: remove oldest entry
                if let Some(oldest_key) = cache.keys().next().copied() {
                    cache.remove(&oldest_key);
                }
            }
            cache.insert(expr_hash, result);
        }
    }

    /// Get cache statistics for monitoring
    pub fn cache_stats(&self) -> CacheStats {
        let simplify_size = self.simplify_cache.read().map(|c| c.len()).unwrap_or(0);
        let derivative_size = self.derivative_cache.read().map(|c| c.len()).unwrap_or(0);

        CacheStats {
            simplify_cache_size: simplify_size,
            derivative_cache_size: derivative_size,
            total_memory_estimate: (simplify_size + derivative_size) * 1024, // Rough estimate
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub simplify_cache_size: usize,
    pub derivative_cache_size: usize,
    pub total_memory_estimate: usize, // Bytes
}

/// Global performance optimizer instance
static GLOBAL_OPTIMIZER: OnceLock<PerformanceOptimizer> = OnceLock::new();

/// Initialize global performance optimizer
pub fn init_performance_optimizer(config: PerformanceConfig) {
    let _ = GLOBAL_OPTIMIZER.get_or_init(|| PerformanceOptimizer::new(config));
}

/// Get global performance optimizer
pub fn get_performance_optimizer() -> Option<&'static PerformanceOptimizer> {
    GLOBAL_OPTIMIZER.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_config_defaults() {
        let config = PerformanceConfig::default();
        assert!(config.simd_enabled);
        assert_eq!(config.simd_threshold, 50);
        assert!(config.memoization_enabled);
    }

    #[test]
    fn test_python_optimized_config() {
        let config = PerformanceConfig::python_optimized();
        assert!(config.simd_enabled);
        assert!(!config.parallel_enabled); // Should avoid GIL contention
        assert!(config.memoization_enabled);
        assert_eq!(config.cache_size_limit, 50000);
    }

    #[test]
    fn test_performance_optimizer_simd_threshold() {
        let config = PerformanceConfig::default();
        let optimizer = PerformanceOptimizer::new(config);

        assert!(!optimizer.should_use_simd(10)); // Below threshold
        assert!(optimizer.should_use_simd(100)); // Above threshold
    }

    #[test]
    fn test_memoization_cache() {
        let config = PerformanceConfig::default();
        let optimizer = PerformanceOptimizer::new(config);

        let expr = Expression::integer(42);
        let hash = 12345u64;

        // Initially empty
        assert!(optimizer.get_cached_simplify(hash).is_none());

        // Cache result
        optimizer.cache_simplify(hash, expr.clone());

        // Should retrieve cached result
        assert_eq!(optimizer.get_cached_simplify(hash), Some(expr));
    }
}

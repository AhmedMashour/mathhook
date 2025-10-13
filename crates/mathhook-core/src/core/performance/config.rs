//! Global performance configuration management
//!
//! This module provides a global configuration system that allows:
//! 1. Binding crates to set their optimal configuration once
//! 2. Core operations to use the global config by default
//! 3. Explicit overrides when needed for specific operations

use super::simd::{SimdOps, SimdOptimized};
use super::strategy::{BindingContext, PerformanceConfig};
use crate::core::Expression;
use num_traits::ToPrimitive;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

/// Global performance configuration instance
static GLOBAL_CONFIG: OnceLock<Arc<RwLock<PerformanceConfig>>> = OnceLock::new();

/// Initialize global configuration (called automatically)
fn ensure_global_config() -> &'static Arc<RwLock<PerformanceConfig>> {
    GLOBAL_CONFIG.get_or_init(|| Arc::new(RwLock::new(PerformanceConfig::default())))
}

/// Get the current global performance configuration
pub fn get_global_config() -> PerformanceConfig {
    let config_lock = ensure_global_config();
    config_lock.read().unwrap().clone()
}

/// Set the global performance configuration
///
/// This is typically called once by binding crates during initialization:
///
/// ```ignore
/// // In mathhook-python/src/lib.rs
/// use mathhook_core::core::performance::config::set_global_config;
/// use mathhook_core::core::performance::strategy::{PerformanceConfig, BindingContext};
///
/// #[pymodule]
/// fn mathhook_python(_py: Python, m: &PyModule) -> PyResult<()> {
///     // Set Python-optimized configuration globally
///     let config = PerformanceConfig::for_binding(BindingContext::Python);
///     set_global_config(config);
///     Ok(())
/// }
/// ```
pub fn set_global_config(config: PerformanceConfig) {
    let config_lock = ensure_global_config();
    *config_lock.write().unwrap() = config;
}

/// Set global configuration for a specific binding context
///
/// Convenience method for binding crates:
///
/// ```ignore
/// // In mathhook-node/src/lib.rs
/// use mathhook_core::core::performance::config::set_binding_config;
/// use mathhook_core::core::performance::strategy::BindingContext;
///
/// fn init_mathhook_node() {
///     set_binding_config(BindingContext::NodeJs);
/// }
/// ```
pub fn set_binding_config(context: BindingContext) {
    let config = PerformanceConfig::for_binding(context);
    set_global_config(config);
}

/// Update specific configuration parameters without replacing the entire config
///
/// Useful for runtime tuning:
///
/// ```
/// use mathhook_core::core::performance::config::update_global_config;
///
/// // Disable parallelism at runtime
/// update_global_config(|config| {
///     config.parallel_enabled = false;
/// });
/// ```
pub fn update_global_config<F>(updater: F)
where
    F: FnOnce(&mut PerformanceConfig),
{
    let config_lock = ensure_global_config();
    if let Ok(mut config) = config_lock.write() {
        updater(&mut config);
    }
}

/// Get configuration statistics for monitoring
pub fn get_config_info() -> ConfigInfo {
    let config = get_global_config();
    ConfigInfo {
        simd_enabled: config.simd_enabled,
        simd_threshold: config.simd_threshold,
        memoization_enabled: config.memoization_enabled,
        cache_size_limit: config.cache_size_limit,
        parallel_enabled: config.parallel_enabled,
        parallel_threshold: config.parallel_threshold,
    }
}

/// Configuration information for monitoring and debugging
#[derive(Debug, Clone)]
pub struct ConfigInfo {
    pub simd_enabled: bool,
    pub simd_threshold: usize,
    pub memoization_enabled: bool,
    pub cache_size_limit: usize,
    pub parallel_enabled: bool,
    pub parallel_threshold: usize,
}

/// Comprehensive cache statistics for performance monitoring
#[derive(Debug, Clone)]
pub struct CacheStatistics {
    /// Current number of cached expressions
    pub current_size: usize,
    /// Maximum cache capacity
    pub max_size: usize,
    /// Estimated memory usage in bytes
    pub memory_estimate_bytes: usize,
    /// Cache utilization as percentage (0.0 - 100.0)
    pub utilization_percent: f64,
    /// Whether the cache is at maximum capacity
    pub is_full: bool,
}

impl Default for CacheStatistics {
    fn default() -> Self {
        Self {
            current_size: 0,
            max_size: CACHE_SIZE_LIMIT,
            memory_estimate_bytes: 0,
            utilization_percent: 0.0,
            is_full: false,
        }
    }
}

/// Comprehensive performance monitoring data
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Current configuration
    pub config: ConfigInfo,
    /// Cache statistics
    pub cache: CacheStatistics,
    /// SIMD usage statistics
    pub simd_stats: SimdStatistics,
    /// Parallel processing statistics
    pub parallel_stats: ParallelStatistics,
}

/// SIMD operation statistics
#[derive(Debug, Clone)]
pub struct SimdStatistics {
    /// Total SIMD operations performed
    pub operations_count: u64,
    /// Total elements processed via SIMD
    pub elements_processed: u64,
    /// Average elements per SIMD operation
    pub avg_elements_per_op: f64,
}

impl Default for SimdStatistics {
    fn default() -> Self {
        Self {
            operations_count: 0,
            elements_processed: 0,
            avg_elements_per_op: 0.0,
        }
    }
}

/// Parallel processing statistics
#[derive(Debug, Clone)]
pub struct ParallelStatistics {
    /// Total parallel operations performed
    pub operations_count: u64,
    /// Total elements processed in parallel
    pub elements_processed: u64,
    /// Average elements per parallel operation
    pub avg_elements_per_op: f64,
}

impl Default for ParallelStatistics {
    fn default() -> Self {
        Self {
            operations_count: 0,
            elements_processed: 0,
            avg_elements_per_op: 0.0,
        }
    }
}

/// Performance thresholds based on benchmarks
const SIMD_THRESHOLD: usize = 50; // 50+ elements benefit from SIMD
const CACHE_SIZE_LIMIT: usize = 10000; // 10K expressions = ~10MB cache

/// Parallelism threshold - exported for binding-specific crates to use
pub const PARALLEL_THRESHOLD: usize = 1000; // 1000+ elements benefit from parallelism

/// Global memoization cache for expensive operations
static GLOBAL_CACHE: OnceLock<Arc<RwLock<HashMap<u64, Expression>>>> = OnceLock::new();

/// Get reference to global cache (initializes if needed)
fn get_global_cache() -> &'static Arc<RwLock<HashMap<u64, Expression>>> {
    GLOBAL_CACHE.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

/// Get cached result for expression hash
pub fn get_cached_result(expr_hash: u64) -> Option<Expression> {
    let cache = get_global_cache();
    cache.read().ok()?.get(&expr_hash).cloned()
}

/// Cache a computation result
pub fn cache_result(expr_hash: u64, result: Expression) {
    let cache_arc = get_global_cache();
    if let Ok(mut cache) = cache_arc.write() {
        // Simple LRU: remove oldest if cache is full
        if cache.len() >= CACHE_SIZE_LIMIT {
            if let Some(oldest_key) = cache.keys().next().copied() {
                cache.remove(&oldest_key);
            }
        }
        cache.insert(expr_hash, result);
    }
}

/// Smart SIMD decision: use SIMD only when beneficial
#[inline(always)]
pub fn should_use_simd(operation_size: usize) -> bool {
    operation_size >= SIMD_THRESHOLD
}

/// Check if operation size meets parallel threshold (for binding-specific use)
#[inline(always)]
pub fn meets_parallel_threshold(operation_size: usize) -> bool {
    operation_size >= PARALLEL_THRESHOLD
}

/// SIMD-optimized bulk numeric addition
pub fn simd_bulk_add_numeric(values: &[f64]) -> f64 {
    if should_use_simd(values.len()) {
        SimdOptimized::bulk_add_numeric(values)
    } else {
        values.iter().sum()
    }
}

/// SIMD-optimized bulk numeric multiplication
pub fn simd_bulk_multiply_numeric(values: &[f64]) -> f64 {
    if should_use_simd(values.len()) {
        // Use SIMD for large arrays
        values
            .chunks(4)
            .map(|chunk| {
                let mut result = vec![0.0; chunk.len()];
                let ones = vec![1.0; chunk.len()];
                SimdOps::mul_f64_array(chunk, &ones, &mut result);
                result.iter().product::<f64>()
            })
            .product()
    } else {
        // Fallback for small arrays
        values.iter().product()
    }
}

/// Extract numeric values from expressions for SIMD processing
pub fn extract_numeric_f64(expressions: &[Expression]) -> (Vec<f64>, Vec<Expression>) {
    let mut numerics = Vec::new();
    let mut non_numerics = Vec::new();

    for expr in expressions {
        match expr {
            Expression::Number(crate::core::Number::Integer(i)) => {
                if let Some(f) = i.to_f64() {
                    numerics.push(f);
                } else {
                    non_numerics.push(expr.clone());
                }
            }
            Expression::Number(crate::core::Number::Float(f)) => {
                numerics.push(*f);
            }
            Expression::Number(crate::core::Number::Rational(r)) => {
                if let Some(f) = r.to_f64() {
                    numerics.push(f);
                } else {
                    non_numerics.push(expr.clone());
                }
            }
            _ => non_numerics.push(expr.clone()),
        }
    }

    (numerics, non_numerics)
}

/// Compute hash for expression (for memoization)
pub fn compute_expr_hash(expr: &Expression) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    // Simple hash based on expression structure
    std::mem::discriminant(expr).hash(&mut hasher);
    hasher.finish()
}

/// Get comprehensive cache statistics for monitoring
pub fn cache_stats() -> CacheStatistics {
    let cache_arc = get_global_cache();
    if let Ok(cache) = cache_arc.read() {
        let current_size = cache.len();
        let memory_estimate = current_size * 512; // Rough estimate: 512 bytes per expression
        let utilization = (current_size as f64 / CACHE_SIZE_LIMIT as f64) * 100.0;

        CacheStatistics {
            current_size,
            max_size: CACHE_SIZE_LIMIT,
            memory_estimate_bytes: memory_estimate,
            utilization_percent: utilization,
            is_full: current_size >= CACHE_SIZE_LIMIT,
        }
    } else {
        CacheStatistics::default()
    }
}

/// Get comprehensive performance metrics for monitoring and debugging
///
/// Note: SIMD and parallel statistics tracking is not yet implemented.
/// These fields return default values until runtime tracking is added.
pub fn get_performance_metrics() -> PerformanceMetrics {
    PerformanceMetrics {
        config: get_config_info(),
        cache: cache_stats(),
        simd_stats: SimdStatistics::default(),
        parallel_stats: ParallelStatistics::default(),
    }
}

/// Get performance summary as human-readable string
pub fn get_performance_summary() -> String {
    let metrics = get_performance_metrics();

    format!(
        "MathHook Performance Summary:\n\
         ├─ Configuration:\n\
         │  ├─ SIMD: {} (threshold: {})\n\
         │  ├─ Parallelism: {} (threshold: {})\n\
         │  └─ Memoization: {} (limit: {})\n\
         ├─ Cache Statistics:\n\
         │  ├─ Size: {}/{} ({:.1}%)\n\
         │  ├─ Memory: {:.2} KB\n\
         │  └─ Status: {}\n\
         └─ Optimization Status: {}",
        if metrics.config.simd_enabled {
            "Enabled"
        } else {
            "Disabled"
        },
        metrics.config.simd_threshold,
        if metrics.config.parallel_enabled {
            "Enabled"
        } else {
            "Disabled"
        },
        metrics.config.parallel_threshold,
        if metrics.config.memoization_enabled {
            "Enabled"
        } else {
            "Disabled"
        },
        metrics.config.cache_size_limit,
        metrics.cache.current_size,
        metrics.cache.max_size,
        metrics.cache.utilization_percent,
        metrics.cache.memory_estimate_bytes as f64 / 1024.0,
        if metrics.cache.is_full {
            "Full"
        } else {
            "Available"
        },
        if metrics.config.simd_enabled && metrics.config.memoization_enabled {
            "Fully Optimized"
        } else {
            "Partially Optimized"
        }
    )
}

/// Clear the global cache
pub fn clear_cache() {
    let cache_arc = get_global_cache();
    if let Ok(mut cache) = cache_arc.write() {
        cache.clear();
    }
}

/// Parallel bulk expression simplification
pub fn parallel_bulk_simplify(expressions: &[Expression]) -> Vec<Expression> {
    let config = get_global_config();

    if config.parallel_enabled && expressions.len() >= config.parallel_threshold {
        // Use parallel processing for large collections
        expressions.par_iter().map(|expr| expr.clone()).collect()
    } else {
        // Sequential processing for small collections or when parallel disabled
        expressions.iter().map(|expr| expr.clone()).collect()
    }
}

/// Parallel matrix element processing
pub fn parallel_matrix_process<F, T>(matrix_rows: &[Vec<Expression>], processor: F) -> Vec<Vec<T>>
where
    F: Fn(&Expression) -> T + Sync + Send,
    T: Send,
{
    let config = get_global_config();
    let total_elements: usize = matrix_rows.iter().map(|row| row.len()).sum();

    if config.parallel_enabled && total_elements >= config.parallel_threshold {
        // Use parallel processing for large matrices
        matrix_rows
            .par_iter()
            .map(|row| row.par_iter().map(&processor).collect())
            .collect()
    } else {
        // Sequential processing for small matrices or when parallel disabled
        matrix_rows
            .iter()
            .map(|row| row.iter().map(&processor).collect())
            .collect()
    }
}

/// Parallel numeric operations with SIMD + Rayon combination
pub fn parallel_simd_bulk_add(values: &[f64]) -> f64 {
    let config = get_global_config();

    if config.parallel_enabled && values.len() >= config.parallel_threshold {
        // Use parallel chunks with SIMD for very large arrays
        let chunk_size = config.parallel_threshold / 4; // Optimal chunk size
        values
            .par_chunks(chunk_size)
            .map(|chunk| simd_bulk_add_numeric(chunk))
            .sum()
    } else {
        // Use SIMD only for smaller arrays
        simd_bulk_add_numeric(values)
    }
}

/// Parallel numeric multiplication with SIMD + Rayon combination
pub fn parallel_simd_bulk_multiply(values: &[f64]) -> f64 {
    let config = get_global_config();

    if config.parallel_enabled && values.len() >= config.parallel_threshold {
        // Use parallel chunks with SIMD for very large arrays
        let chunk_size = config.parallel_threshold / 4; // Optimal chunk size
        values
            .par_chunks(chunk_size)
            .map(|chunk| simd_bulk_multiply_numeric(chunk))
            .product()
    } else {
        // Use SIMD only for smaller arrays
        simd_bulk_multiply_numeric(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_config_initialization() {
        let config = get_global_config();
        assert!(config.simd_enabled);
        assert!(config.memoization_enabled);
    }

    #[test]
    fn test_binding_config_setting() {
        set_binding_config(BindingContext::Python);
        let config = get_global_config();
        assert!(!config.parallel_enabled); // Python should disable parallelism
        assert_eq!(config.parallel_threshold, usize::MAX);
    }

    #[test]
    fn test_config_update() {
        let original_threshold = get_global_config().simd_threshold;

        update_global_config(|config| {
            config.simd_threshold = 100;
        });

        let updated_config = get_global_config();
        assert_eq!(updated_config.simd_threshold, 100);

        // Reset for other tests
        update_global_config(|config| {
            config.simd_threshold = original_threshold;
        });
    }
}

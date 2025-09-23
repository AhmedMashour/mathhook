//! Performance optimization system for MathHook
//!
//! This module provides a comprehensive performance optimization system with:
//! - SIMD operations for bulk numeric computations
//! - Intelligent memoization with configurable caching
//! - Binding-specific performance strategies
//! - Global configuration management
//! - Smart automatic optimization detection

pub mod background_compute;
pub mod config;
pub mod gpu_acceleration;
pub mod persistent_cache;
pub mod profiler;
pub mod simd;
pub mod stable_operations;
pub mod strategy;

#[cfg(test)]
pub mod phase3_integration_test;

pub use background_compute::{
    clear_background_compute, get_background_compute_statistics, get_background_result,
    get_background_result_by_expression, predict_and_precompute, submit_background_task,
    BackgroundComputeStatistics, ComputePriority, ComputeResult, ComputeTask,
};
pub use config::{
    cache_result, cache_stats, clear_cache, compute_expr_hash, extract_numeric_f64,
    get_cached_result, get_global_config, get_performance_metrics, get_performance_summary,
    meets_parallel_threshold, parallel_bulk_simplify, parallel_matrix_process,
    parallel_simd_bulk_add, parallel_simd_bulk_multiply, set_binding_config, set_global_config,
    should_use_simd, simd_bulk_add_numeric, simd_bulk_multiply_numeric, update_global_config,
    CacheStatistics, ConfigInfo, ParallelStatistics, PerformanceMetrics, SimdStatistics,
};
pub use gpu_acceleration::{
    get_gpu_capabilities, get_gpu_statistics, gpu_or_cpu_bulk_add, gpu_or_cpu_matrix_multiply,
    is_gpu_available, GpuBackend, GpuCapabilities, GpuError, GpuOperation, GpuStatistics,
};
pub use persistent_cache::{
    clear_persistent_cache, get_persistent_cache_statistics, get_persistent_cached_result,
    save_persistent_cache, store_persistent_cached_result, PersistentCacheStatistics,
};
pub use profiler::{
    get_adaptive_thresholds, get_profiler_statistics, record_performance, AdaptiveThresholds,
    PerformanceMeasurement, ProfilerStatistics,
};
pub use simd::{SimdOps, SimdOptimized};
pub use stable_operations::{
    stable_bulk_addition, stable_bulk_multiplication, StableCache, StableMatrix, StableSIMD,
};
pub use strategy::{BindingContext, PerformanceConfig, PerformanceOptimizer};

/// Re-export commonly used performance functions for convenience
pub use config::get_global_config as get_config;
pub use strategy::{BindingContext as Context, PerformanceConfig as Config};

/// Smart performance utilities that automatically choose optimal strategies
pub mod smart {
    pub use super::config::{
        cache_result, compute_expr_hash, extract_numeric_f64, get_cached_result,
        meets_parallel_threshold, parallel_bulk_simplify, parallel_matrix_process,
        parallel_simd_bulk_add, parallel_simd_bulk_multiply, should_use_simd,
        simd_bulk_add_numeric, simd_bulk_multiply_numeric,
    };
}

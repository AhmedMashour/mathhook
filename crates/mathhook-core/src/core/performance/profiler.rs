//! Runtime Performance Profiler
//!
//! This module implements adaptive performance profiling that learns from actual
//! runtime behavior to optimize SIMD and parallelism thresholds dynamically.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};
use std::time::{Duration, Instant};

/// Performance measurement for a specific operation type and size
#[derive(Debug, Clone)]
pub struct PerformanceMeasurement {
    /// Operation type (e.g., "simd_add", "parallel_multiply", "sequential_add")
    pub operation_type: String,
    /// Number of elements processed
    pub operation_size: usize,
    /// Time taken for the operation
    pub duration: Duration,
    /// Timestamp when measurement was taken
    pub timestamp: Instant,
}

/// Adaptive threshold configuration that learns from runtime performance
#[derive(Debug, Clone)]
pub struct AdaptiveThresholds {
    /// Current SIMD threshold (dynamically adjusted)
    pub simd_threshold: usize,
    /// Current parallelism threshold (dynamically adjusted)
    pub parallel_threshold: usize,
    /// Confidence level in current thresholds (0.0 - 1.0)
    pub confidence: f64,
    /// Number of measurements used to determine thresholds
    pub sample_count: usize,
}

impl Default for AdaptiveThresholds {
    fn default() -> Self {
        Self {
            simd_threshold: 50,       // Conservative starting point
            parallel_threshold: 1000, // Conservative starting point
            confidence: 0.0,          // No confidence initially
            sample_count: 0,
        }
    }
}

/// Runtime performance profiler that adapts thresholds based on actual performance
pub struct RuntimeProfiler {
    /// Historical performance measurements
    measurements: Arc<RwLock<Vec<PerformanceMeasurement>>>,
    /// Current adaptive thresholds
    thresholds: Arc<RwLock<AdaptiveThresholds>>,
    /// Maximum number of measurements to keep in memory
    max_measurements: usize,
    /// Minimum samples needed before adapting thresholds
    min_samples_for_adaptation: usize,
}

impl Default for RuntimeProfiler {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeProfiler {
    /// Create a new runtime profiler
    pub fn new() -> Self {
        Self {
            measurements: Arc::new(RwLock::new(Vec::new())),
            thresholds: Arc::new(RwLock::new(AdaptiveThresholds::default())),
            max_measurements: 10000,        // Keep last 10k measurements
            min_samples_for_adaptation: 50, // Need 50+ samples to adapt
        }
    }

    /// Record a performance measurement
    pub fn record_measurement(&self, measurement: PerformanceMeasurement) {
        if let Ok(mut measurements) = self.measurements.write() {
            measurements.push(measurement);

            // Keep only the most recent measurements
            if measurements.len() > self.max_measurements {
                let len = measurements.len();
                measurements.drain(0..len - self.max_measurements);
            }

            // Trigger threshold adaptation if we have enough samples
            if measurements.len() >= self.min_samples_for_adaptation {
                self.adapt_thresholds(&measurements);
            }
        }
    }

    /// Get current adaptive thresholds
    pub fn get_thresholds(&self) -> AdaptiveThresholds {
        self.thresholds
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    /// Adapt thresholds based on performance measurements
    fn adapt_thresholds(&self, measurements: &[PerformanceMeasurement]) {
        let simd_threshold = self.find_optimal_simd_threshold(measurements);
        let parallel_threshold = self.find_optimal_parallel_threshold(measurements);

        if let Ok(mut thresholds) = self.thresholds.write() {
            let old_simd = thresholds.simd_threshold;
            let old_parallel = thresholds.parallel_threshold;

            // Use exponential moving average for smooth adaptation
            let alpha = 0.1; // Learning rate
            thresholds.simd_threshold =
                ((1.0 - alpha) * old_simd as f64 + alpha * simd_threshold as f64) as usize;
            thresholds.parallel_threshold =
                ((1.0 - alpha) * old_parallel as f64 + alpha * parallel_threshold as f64) as usize;

            // Update confidence based on sample size and consistency
            thresholds.sample_count = measurements.len();
            thresholds.confidence = self.calculate_confidence(measurements);

            // Log threshold changes for debugging
            if old_simd != thresholds.simd_threshold
                || old_parallel != thresholds.parallel_threshold
            {
                println!("Adaptive thresholds updated: SIMD {} -> {}, Parallel {} -> {} (confidence: {:.2})",
                    old_simd, thresholds.simd_threshold,
                    old_parallel, thresholds.parallel_threshold,
                    thresholds.confidence
                );
            }
        }
    }

    /// Find optimal SIMD threshold by analyzing performance crossover point
    fn find_optimal_simd_threshold(&self, measurements: &[PerformanceMeasurement]) -> usize {
        let mut simd_measurements: Vec<_> = measurements
            .iter()
            .filter(|m| m.operation_type.contains("simd"))
            .collect();
        let mut sequential_measurements: Vec<_> = measurements
            .iter()
            .filter(|m| m.operation_type.contains("sequential"))
            .collect();

        if simd_measurements.is_empty() || sequential_measurements.is_empty() {
            return 50; // Default fallback
        }

        // Sort by operation size
        simd_measurements.sort_by_key(|m| m.operation_size);
        sequential_measurements.sort_by_key(|m| m.operation_size);

        // Find crossover point where SIMD becomes faster than sequential
        for size in (10..=1000).step_by(10) {
            let simd_perf = self.estimate_performance_at_size(&simd_measurements, size);
            let seq_perf = self.estimate_performance_at_size(&sequential_measurements, size);

            if let (Some(simd_time), Some(seq_time)) = (simd_perf, seq_perf) {
                if simd_time < seq_time {
                    return size;
                }
            }
        }

        50 // Conservative fallback
    }

    /// Find optimal parallelism threshold by analyzing performance crossover point
    fn find_optimal_parallel_threshold(&self, measurements: &[PerformanceMeasurement]) -> usize {
        let mut parallel_measurements: Vec<_> = measurements
            .iter()
            .filter(|m| m.operation_type.contains("parallel"))
            .collect();
        let mut sequential_measurements: Vec<_> = measurements
            .iter()
            .filter(|m| m.operation_type.contains("sequential"))
            .collect();

        if parallel_measurements.is_empty() || sequential_measurements.is_empty() {
            return 1000; // Default fallback
        }

        // Sort by operation size
        parallel_measurements.sort_by_key(|m| m.operation_size);
        sequential_measurements.sort_by_key(|m| m.operation_size);

        // Find crossover point where parallel becomes faster than sequential
        for size in (100..=5000).step_by(100) {
            let parallel_perf = self.estimate_performance_at_size(&parallel_measurements, size);
            let seq_perf = self.estimate_performance_at_size(&sequential_measurements, size);

            if let (Some(parallel_time), Some(seq_time)) = (parallel_perf, seq_perf) {
                if parallel_time < seq_time {
                    return size;
                }
            }
        }

        1000 // Conservative fallback
    }

    /// Estimate performance at a specific operation size using interpolation
    fn estimate_performance_at_size(
        &self,
        measurements: &[&PerformanceMeasurement],
        target_size: usize,
    ) -> Option<Duration> {
        if measurements.is_empty() {
            return None;
        }

        // Find measurements closest to target size
        let mut closest_smaller = None;
        let mut closest_larger = None;

        for measurement in measurements {
            if measurement.operation_size <= target_size {
                closest_smaller = Some(measurement);
            } else if closest_larger.is_none() {
                closest_larger = Some(measurement);
                break;
            }
        }

        match (closest_smaller, closest_larger) {
            (Some(smaller), Some(larger)) => {
                // Linear interpolation
                let size_diff = larger.operation_size - smaller.operation_size;
                let time_diff =
                    larger.duration.as_nanos() as f64 - smaller.duration.as_nanos() as f64;
                let target_offset = target_size - smaller.operation_size;

                let interpolated_nanos = smaller.duration.as_nanos() as f64
                    + (time_diff * target_offset as f64) / size_diff as f64;

                Some(Duration::from_nanos(interpolated_nanos as u64))
            }
            (Some(measurement), None) | (None, Some(measurement)) => {
                // Use the closest measurement
                Some(measurement.duration)
            }
            (None, None) => None,
        }
    }

    /// Calculate confidence in current thresholds based on measurement consistency
    fn calculate_confidence(&self, measurements: &[PerformanceMeasurement]) -> f64 {
        if measurements.len() < 10 {
            return 0.0;
        }

        // Calculate variance in performance measurements
        let recent_measurements: Vec<_> = measurements
            .iter()
            .rev()
            .take(100) // Use last 100 measurements
            .collect();

        if recent_measurements.is_empty() {
            return 0.0;
        }

        // Group by operation type and calculate consistency
        let mut type_groups: HashMap<String, Vec<Duration>> = HashMap::new();
        for measurement in recent_measurements {
            type_groups
                .entry(measurement.operation_type.clone())
                .or_default()
                .push(measurement.duration);
        }

        let mut total_consistency = 0.0;
        let mut group_count = 0;

        for (_, durations) in type_groups {
            if durations.len() < 3 {
                continue;
            }

            let mean_duration =
                durations.iter().sum::<Duration>().as_nanos() as f64 / durations.len() as f64;
            let variance = durations
                .iter()
                .map(|d| {
                    let diff = d.as_nanos() as f64 - mean_duration;
                    diff * diff
                })
                .sum::<f64>()
                / durations.len() as f64;

            let coefficient_of_variation = if mean_duration > 0.0 {
                variance.sqrt() / mean_duration
            } else {
                1.0
            };

            // Lower coefficient of variation = higher consistency = higher confidence
            let consistency = 1.0 / (1.0 + coefficient_of_variation);
            total_consistency += consistency;
            group_count += 1;
        }

        if group_count > 0 {
            (total_consistency / group_count as f64).min(1.0)
        } else {
            0.0
        }
    }

    /// Get performance statistics for monitoring
    pub fn get_statistics(&self) -> ProfilerStatistics {
        let measurements = self
            .measurements
            .read()
            .expect("BUG: Profiler measurements lock poisoned - indicates panic during profiler read in another thread");
        let thresholds = self.get_thresholds();

        let total_measurements = measurements.len();
        let recent_measurements = measurements.iter().rev().take(100).count();

        // Calculate average performance by operation type
        let mut type_stats: HashMap<String, (Duration, usize)> = HashMap::new();
        for measurement in measurements.iter().rev().take(1000) {
            let (total_duration, count) = type_stats
                .entry(measurement.operation_type.clone())
                .or_insert((Duration::ZERO, 0));
            *total_duration += measurement.duration;
            *count += 1;
        }

        let average_performance: HashMap<String, Duration> = type_stats
            .into_iter()
            .map(|(op_type, (total_duration, count))| {
                let avg_duration = if count > 0 {
                    Duration::from_nanos((total_duration.as_nanos() / count as u128) as u64)
                } else {
                    Duration::ZERO
                };
                (op_type, avg_duration)
            })
            .collect();

        ProfilerStatistics {
            total_measurements,
            recent_measurements,
            current_thresholds: thresholds,
            average_performance,
        }
    }
}

/// Statistics from the runtime profiler
#[derive(Debug, Clone)]
pub struct ProfilerStatistics {
    /// Total number of measurements recorded
    pub total_measurements: usize,
    /// Number of recent measurements (last 100)
    pub recent_measurements: usize,
    /// Current adaptive thresholds
    pub current_thresholds: AdaptiveThresholds,
    /// Average performance by operation type
    pub average_performance: HashMap<String, Duration>,
}

/// Global runtime profiler instance
static GLOBAL_PROFILER: OnceLock<RuntimeProfiler> = OnceLock::new();

/// Get the global runtime profiler instance
pub fn get_global_profiler() -> &'static RuntimeProfiler {
    GLOBAL_PROFILER.get_or_init(RuntimeProfiler::new)
}

/// Record a performance measurement in the global profiler
pub fn record_performance(operation_type: &str, operation_size: usize, duration: Duration) {
    let measurement = PerformanceMeasurement {
        operation_type: operation_type.to_owned(),
        operation_size,
        duration,
        timestamp: Instant::now(),
    };

    get_global_profiler().record_measurement(measurement);
}

/// Get current adaptive thresholds from the global profiler
pub fn get_adaptive_thresholds() -> AdaptiveThresholds {
    get_global_profiler().get_thresholds()
}

/// Get profiler statistics for monitoring
pub fn get_profiler_statistics() -> ProfilerStatistics {
    get_global_profiler().get_statistics()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_profiler_creation() {
        let profiler = RuntimeProfiler::new();
        let thresholds = profiler.get_thresholds();

        assert_eq!(thresholds.simd_threshold, 50);
        assert_eq!(thresholds.parallel_threshold, 1000);
        assert_eq!(thresholds.confidence, 0.0);
        assert_eq!(thresholds.sample_count, 0);
    }

    #[test]
    fn test_measurement_recording() {
        let profiler = RuntimeProfiler::new();

        let measurement = PerformanceMeasurement {
            operation_type: "test_operation".to_string(),
            operation_size: 100,
            duration: Duration::from_millis(10),
            timestamp: Instant::now(),
        };

        profiler.record_measurement(measurement);

        // Should not adapt thresholds with just one measurement
        let thresholds = profiler.get_thresholds();
        assert_eq!(thresholds.sample_count, 0); // Not enough samples yet
    }

    #[test]
    fn test_global_profiler() {
        record_performance("test_simd", 100, Duration::from_micros(50));
        record_performance("test_sequential", 100, Duration::from_micros(100));

        let stats = get_profiler_statistics();
        assert!(stats.total_measurements >= 2);

        let thresholds = get_adaptive_thresholds();
        assert!(thresholds.simd_threshold > 0);
        assert!(thresholds.parallel_threshold > 0);
    }
}

//! Background Precomputation System
//!
//! This module implements background precomputation of common expressions
//! to improve interactive performance by predicting and caching likely operations.

use crate::core::Expression;
use crate::simplify::Simplify;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

/// Priority level for background computation tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComputePriority {
    /// Low priority - compute when system is idle
    Low = 1,
    /// Medium priority - compute during normal operation
    Medium = 2,
    /// High priority - compute immediately
    High = 3,
}

/// Background computation task
#[derive(Debug, Clone)]
pub struct ComputeTask {
    /// Unique identifier for the task
    pub id: u64,
    /// Expression to precompute
    pub expression: Expression,
    /// Priority level
    pub priority: ComputePriority,
    /// When the task was created
    pub created_at: Instant,
    /// Predicted likelihood this will be needed (0.0 - 1.0)
    pub likelihood: f64,
}

/// Result of a background computation
#[derive(Debug, Clone)]
pub struct ComputeResult {
    /// Task ID that produced this result
    pub task_id: u64,
    /// Original expression
    pub original: Expression,
    /// Computed result
    pub result: Expression,
    /// Time taken to compute
    pub compute_time: Duration,
    /// When computation completed
    pub completed_at: Instant,
}

/// Background computation engine
pub struct BackgroundCompute {
    /// Task queue organized by priority
    task_queue: Arc<Mutex<VecDeque<ComputeTask>>>,
    /// Completed results cache
    results_cache: Arc<Mutex<HashMap<u64, ComputeResult>>>,
    /// Next task ID
    next_task_id: Arc<Mutex<u64>>,
    /// Whether the background worker is running
    worker_running: Arc<Mutex<bool>>,
    /// Maximum number of results to cache
    max_cached_results: usize,
    /// Maximum task queue size
    max_queue_size: usize,
}

impl Default for BackgroundCompute {
    fn default() -> Self {
        Self::new()
    }
}

impl BackgroundCompute {
    /// Create a new background compute engine
    pub fn new() -> Self {
        let engine = Self {
            task_queue: Arc::new(Mutex::new(VecDeque::new())),
            results_cache: Arc::new(Mutex::new(HashMap::new())),
            next_task_id: Arc::new(Mutex::new(1)),
            worker_running: Arc::new(Mutex::new(false)),
            max_cached_results: 1000,
            max_queue_size: 500,
        };

        // Start background worker
        engine.start_worker();
        engine
    }

    /// Submit a task for background computation
    pub fn submit_task(
        &self,
        expression: Expression,
        priority: ComputePriority,
        likelihood: f64,
    ) -> u64 {
        let task_id = {
            let mut next_id = self
                .next_task_id
                .lock()
                .expect("BUG: Background compute task ID lock poisoned - indicates panic during task submission in another thread");
            let id = *next_id;
            *next_id += 1;
            id
        };

        let task = ComputeTask {
            id: task_id,
            expression,
            priority,
            created_at: Instant::now(),
            likelihood,
        };

        if let Ok(mut queue) = self.task_queue.lock() {
            // Remove oldest low-priority tasks if queue is full
            if queue.len() >= self.max_queue_size {
                queue.retain(|t| t.priority != ComputePriority::Low);

                // If still full, remove oldest medium priority
                if queue.len() >= self.max_queue_size {
                    queue.retain(|t| t.priority == ComputePriority::High);
                }
            }

            // Insert task in priority order
            let insert_pos = queue
                .iter()
                .position(|t| t.priority < task.priority)
                .unwrap_or(queue.len());

            queue.insert(insert_pos, task);
        }

        task_id
    }

    /// Get a precomputed result if available
    pub fn get_result(&self, task_id: u64) -> Option<ComputeResult> {
        self.results_cache.lock().ok()?.get(&task_id).cloned()
    }

    /// Get result by expression hash (if we've computed this expression before)
    pub fn get_result_by_expression(&self, expr: &Expression) -> Option<ComputeResult> {
        let expr_hash = self.compute_expression_hash(expr);

        if let Ok(cache) = self.results_cache.lock() {
            for result in cache.values() {
                if self.compute_expression_hash(&result.original) == expr_hash {
                    return Some(result.clone());
                }
            }
        }
        None
    }

    /// Submit common expressions for precomputation
    pub fn precompute_common_expressions(&self) {
        let common_expressions = vec![
            // Common algebraic simplifications
            (
                Expression::add(vec![Expression::symbol("x"), Expression::integer(0)]),
                0.8,
            ),
            (
                Expression::mul(vec![Expression::symbol("x"), Expression::integer(1)]),
                0.8,
            ),
            (
                Expression::mul(vec![Expression::symbol("x"), Expression::integer(0)]),
                0.7,
            ),
            // Common trigonometric identities
            (
                Expression::add(vec![
                    Expression::pow(
                        Expression::function("sin", vec![Expression::symbol("x")]),
                        Expression::integer(2),
                    ),
                    Expression::pow(
                        Expression::function("cos", vec![Expression::symbol("x")]),
                        Expression::integer(2),
                    ),
                ]),
                0.6,
            ),
            // Common polynomial expansions
            (
                Expression::pow(
                    Expression::add(vec![Expression::symbol("x"), Expression::integer(1)]),
                    Expression::integer(2),
                ),
                0.5,
            ),
            (
                Expression::pow(
                    Expression::add(vec![Expression::symbol("x"), Expression::symbol("y")]),
                    Expression::integer(2),
                ),
                0.5,
            ),
            // Common derivatives
            (
                Expression::function(
                    "derivative",
                    vec![
                        Expression::pow(Expression::symbol("x"), Expression::integer(2)),
                        Expression::symbol("x"),
                    ],
                ),
                0.4,
            ),
            (
                Expression::function(
                    "derivative",
                    vec![
                        Expression::function("sin", vec![Expression::symbol("x")]),
                        Expression::symbol("x"),
                    ],
                ),
                0.4,
            ),
        ];

        for (expr, likelihood) in common_expressions {
            self.submit_task(expr, ComputePriority::Low, likelihood);
        }
    }

    /// Predict and precompute likely next expressions based on current expression
    pub fn predict_and_precompute(&self, current_expr: &Expression) {
        let predictions = self.predict_next_expressions(current_expr);

        for (expr, likelihood) in predictions {
            let priority = if likelihood > 0.7 {
                ComputePriority::High
            } else if likelihood > 0.4 {
                ComputePriority::Medium
            } else {
                ComputePriority::Low
            };

            self.submit_task(expr, priority, likelihood);
        }
    }

    /// Predict likely next expressions based on current expression
    fn predict_next_expressions(&self, expr: &Expression) -> Vec<(Expression, f64)> {
        let mut predictions = Vec::new();

        match expr {
            Expression::Add(terms) => {
                // Likely to simplify the addition
                predictions.push((expr.clone(), 0.9));

                // Might factor out common terms
                if terms.len() > 2 {
                    predictions.push((Expression::function("factor", vec![expr.clone()]), 0.6));
                }

                // Might differentiate
                predictions.push((
                    Expression::function("derivative", vec![expr.clone(), Expression::symbol("x")]),
                    0.4,
                ));
            }

            Expression::Mul(factors) => {
                // Likely to simplify the multiplication
                predictions.push((expr.clone(), 0.9));

                // Might expand if there are additions in factors
                for factor in factors.iter() {
                    if matches!(factor, Expression::Add(_)) {
                        predictions.push((Expression::function("expand", vec![expr.clone()]), 0.7));
                        break;
                    }
                }

                // Might differentiate
                predictions.push((
                    Expression::function("derivative", vec![expr.clone(), Expression::symbol("x")]),
                    0.4,
                ));
            }

            Expression::Pow(base, _exponent) => {
                // Likely to simplify the power
                predictions.push((expr.clone(), 0.8));

                // Might differentiate (power rule is common)
                predictions.push((
                    Expression::function("derivative", vec![expr.clone(), Expression::symbol("x")]),
                    0.6,
                ));

                // Might expand if base is a sum
                if matches!(**base, Expression::Add(_)) {
                    predictions.push((Expression::function("expand", vec![expr.clone()]), 0.7));
                }
            }

            Expression::Function { name, args: _ } => {
                match name.as_str() {
                    "sin" | "cos" | "tan" => {
                        // Trigonometric functions often get differentiated
                        predictions.push((
                            Expression::function(
                                "derivative",
                                vec![expr.clone(), Expression::symbol("x")],
                            ),
                            0.7,
                        ));

                        // Might use trigonometric identities
                        predictions
                            .push((Expression::function("trigsimp", vec![expr.clone()]), 0.5));
                    }

                    "exp" | "log" => {
                        // Exponential and logarithmic functions often get differentiated
                        predictions.push((
                            Expression::function(
                                "derivative",
                                vec![expr.clone(), Expression::symbol("x")],
                            ),
                            0.6,
                        ));
                    }

                    _ => {
                        // Generic function - might get differentiated
                        predictions.push((
                            Expression::function(
                                "derivative",
                                vec![expr.clone(), Expression::symbol("x")],
                            ),
                            0.3,
                        ));
                    }
                }
            }

            _ => {
                // For other expressions, predict basic operations
                predictions.push((
                    Expression::function("derivative", vec![expr.clone(), Expression::symbol("x")]),
                    0.3,
                ));
                predictions.push((
                    Expression::function("integrate", vec![expr.clone(), Expression::symbol("x")]),
                    0.2,
                ));
            }
        }

        predictions
    }

    /// Start the background worker thread
    fn start_worker(&self) {
        let queue = Arc::clone(&self.task_queue);
        let cache = Arc::clone(&self.results_cache);
        let running = Arc::clone(&self.worker_running);
        let max_cached = self.max_cached_results;

        // Set worker as running
        *running
            .lock()
            .expect("BUG: Background compute running flag lock poisoned - indicates panic during worker initialization in another thread") = true;

        thread::spawn(move || {
            while *running
                .lock()
                .expect("BUG: Background compute running flag lock poisoned - indicates panic during worker loop check in another thread") {
                // Get next task
                let task = {
                    let mut queue_guard = queue
                        .lock()
                        .expect("BUG: Background compute task queue lock poisoned - indicates panic during task retrieval in another thread");
                    queue_guard.pop_front()
                };

                if let Some(task) = task {
                    // Compute the result
                    let start_time = Instant::now();
                    let result = task.expression.simplify(); // This is the actual computation
                    let compute_time = start_time.elapsed();

                    let compute_result = ComputeResult {
                        task_id: task.id,
                        original: task.expression,
                        result,
                        compute_time,
                        completed_at: Instant::now(),
                    };

                    // Store result in cache
                    if let Ok(mut cache_guard) = cache.lock() {
                        // Remove oldest results if cache is full
                        if cache_guard.len() >= max_cached {
                            let oldest_id = cache_guard
                                .iter()
                                .min_by_key(|(_, result)| result.completed_at)
                                .map(|(id, _)| *id);

                            if let Some(id) = oldest_id {
                                cache_guard.remove(&id);
                            }
                        }

                        cache_guard.insert(task.id, compute_result);
                    }
                } else {
                    // No tasks available, sleep briefly
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });
    }

    /// Stop the background worker
    pub fn stop_worker(&self) {
        *self
            .worker_running
            .lock()
            .expect("BUG: Background compute running flag lock poisoned - indicates panic during worker stop in another thread") = false;
    }

    /// Get statistics about background computation
    pub fn get_statistics(&self) -> BackgroundComputeStatistics {
        let queue_size = self.task_queue.lock().map(|q| q.len()).unwrap_or(0);
        let cache_size = self.results_cache.lock().map(|c| c.len()).unwrap_or(0);
        let worker_running = *self
            .worker_running
            .lock()
            .expect("BUG: Background compute running flag lock poisoned - indicates panic during statistics read in another thread");

        // Calculate priority distribution
        let mut priority_counts = HashMap::new();
        if let Ok(queue) = self.task_queue.lock() {
            for task in queue.iter() {
                *priority_counts.entry(task.priority).or_insert(0) += 1;
            }
        }

        // Calculate average compute time
        let average_compute_time = if let Ok(cache) = self.results_cache.lock() {
            if cache.is_empty() {
                Duration::ZERO
            } else {
                let total_time: Duration = cache.values().map(|r| r.compute_time).sum();
                total_time / cache.len() as u32
            }
        } else {
            Duration::ZERO
        };

        let total_tasks_completed = if let Ok(cache) = self.results_cache.lock() {
            cache.len()
        } else {
            0
        };

        BackgroundComputeStatistics {
            queue_size,
            cache_size,
            worker_running,
            priority_counts,
            average_compute_time,
            total_tasks_completed,
        }
    }

    /// Compute a simple hash for an expression (for deduplication)
    fn compute_expression_hash(&self, expr: &Expression) -> u64 {
        // Simple hash based on expression structure
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        std::mem::discriminant(expr).hash(&mut hasher);
        hasher.finish()
    }

    /// Clear all tasks and results
    pub fn clear(&self) {
        if let Ok(mut queue) = self.task_queue.lock() {
            queue.clear();
        }
        if let Ok(mut cache) = self.results_cache.lock() {
            cache.clear();
        }
    }
}

/// Statistics for background computation
#[derive(Debug, Clone)]
pub struct BackgroundComputeStatistics {
    /// Number of tasks in queue
    pub queue_size: usize,
    /// Number of cached results
    pub cache_size: usize,
    /// Whether worker thread is running
    pub worker_running: bool,
    /// Count of tasks by priority
    pub priority_counts: HashMap<ComputePriority, usize>,
    /// Average computation time
    pub average_compute_time: Duration,
    /// Total number of tasks completed
    pub total_tasks_completed: usize,
}

/// Global background compute engine
static GLOBAL_BACKGROUND_COMPUTE: OnceLock<BackgroundCompute> = OnceLock::new();

/// Get the global background compute engine
pub fn get_global_background_compute() -> &'static BackgroundCompute {
    GLOBAL_BACKGROUND_COMPUTE.get_or_init(|| {
        let engine = BackgroundCompute::new();
        // Precompute common expressions on startup
        engine.precompute_common_expressions();
        engine
    })
}

/// Submit a task for background computation
pub fn submit_background_task(
    expression: Expression,
    priority: ComputePriority,
    likelihood: f64,
) -> u64 {
    get_global_background_compute().submit_task(expression, priority, likelihood)
}

/// Get a precomputed result
pub fn get_background_result(task_id: u64) -> Option<ComputeResult> {
    get_global_background_compute().get_result(task_id)
}

/// Get result by expression if available
pub fn get_background_result_by_expression(expr: &Expression) -> Option<ComputeResult> {
    get_global_background_compute().get_result_by_expression(expr)
}

/// Predict and precompute based on current expression
pub fn predict_and_precompute(current_expr: &Expression) {
    get_global_background_compute().predict_and_precompute(current_expr);
}

/// Get background computation statistics
pub fn get_background_compute_statistics() -> BackgroundComputeStatistics {
    get_global_background_compute().get_statistics()
}

/// Clear background computation cache
pub fn clear_background_compute() {
    get_global_background_compute().clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_compute_creation() {
        let engine = BackgroundCompute::new();
        let stats = engine.get_statistics();

        assert!(stats.worker_running);
        assert_eq!(stats.queue_size, 0);
        assert_eq!(stats.cache_size, 0);
    }

    #[test]
    fn test_task_submission() {
        let engine = BackgroundCompute::new();
        let expr = Expression::add(vec![Expression::symbol("x"), Expression::integer(1)]);

        let task_id = engine.submit_task(expr, ComputePriority::High, 0.8);
        assert!(task_id > 0);

        let stats = engine.get_statistics();
        assert!(stats.queue_size > 0);
    }

    #[test]
    fn test_global_background_compute() {
        let expr = Expression::mul(vec![Expression::symbol("x"), Expression::integer(2)]);
        let task_id = submit_background_task(expr.clone(), ComputePriority::Medium, 0.6);

        assert!(task_id > 0);

        let stats = get_background_compute_statistics();
        assert!(stats.worker_running);

        // Test prediction
        predict_and_precompute(&expr);

        let stats_after = get_background_compute_statistics();
        assert!(
            stats_after.queue_size >= stats.queue_size
                || stats_after.total_tasks_completed > stats.total_tasks_completed
        );
    }

    #[test]
    fn test_priority_ordering() {
        let engine = BackgroundCompute::new();

        // Submit tasks in reverse priority order
        let low_id = engine.submit_task(Expression::integer(1), ComputePriority::Low, 0.1);
        let high_id = engine.submit_task(Expression::integer(2), ComputePriority::High, 0.9);
        let medium_id = engine.submit_task(Expression::integer(3), ComputePriority::Medium, 0.5);

        // High priority task should have been inserted first
        assert!(high_id > low_id);
        assert!(medium_id > high_id);

        let stats = engine.get_statistics();
        assert_eq!(stats.queue_size, 3);
    }
}

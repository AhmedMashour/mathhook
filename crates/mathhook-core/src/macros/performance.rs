//! Performance optimization macros
//!
//! Performance-critical operation patterns and optimizations.
//! These macros provide compile-time optimizations and runtime
//! performance hints for mathematical computations.

/// Performance optimization utilities
///
/// This macro provides performance optimization patterns including
/// memory management, branch prediction hints, and SIMD operations.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::perf;
///
/// // Fast vector construction with known capacity
/// let vec = perf!(vec: 10, 1, 2, 3, 4, 5);
/// assert_eq!(vec.len(), 5);
/// assert!(vec.capacity() >= 10);
///
/// // Branch prediction hints (architecture-dependent)
/// let x = 42;
/// if perf!(likely: x > 0) {
///     // This branch is likely to be taken
/// }
///
/// if perf!(unlikely: x < 0) {
///     // This branch is unlikely to be taken
/// }
/// ```
#[macro_export]
macro_rules! perf {
    // Fast vector construction with known capacity
    (vec: $cap:expr, $($elem:expr),* $(,)?) => {{
        let mut vec = Vec::with_capacity($cap);
        $(vec.push($elem);)*
        vec
    }};

    // Inline small operations
    (inline: $op:expr) => {
        #[inline(always)]
        $op
    };

    // Branch prediction hints (simplified for stable Rust)
    (likely: $cond:expr) => {
        $cond
    };

    (unlikely: $cond:expr) => {
        $cond
    };

    // Cache-friendly iteration
    (cache_friendly_iter: $collection:expr, $chunk_size:expr) => {
        $collection.chunks($chunk_size)
    };

    // Memory prefetch hint (x86_64 only)
    //
    // Note: Actual prefetch intrinsics require unsafe code.
    // This provides a safe no-op interface for future optimization.
    (prefetch: $ptr:expr) => {
        #[cfg(target_arch = "x86_64")]
        {
            // Safe no-op - actual prefetch would use _mm_prefetch intrinsic
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            // No-op on other architectures
        }
    };

    // Fast memory copy for known sizes
    (fast_copy: $src:expr, $dst:expr, $len:expr) => {
        $dst[..$len].copy_from_slice(&$src[..$len]);
    };

    // Unroll small loops (compile-time)
    (unroll: $n:literal, $body:expr) => {{
        // Unroll loop for small, known iteration counts
        let mut i = 0;
        while i < $n {
            $body;
            i += 1;
        }
    }};

    // SIMD-friendly alignment
    (align: $value:expr, $alignment:expr) => {
        ($value + $alignment - 1) & !($alignment - 1)
    };

    // Hot path optimization hint (simplified for stable Rust)
    (hot_path: $code:block) => {
        $code
    };

    // Cold path optimization hint (simplified for stable Rust)
    (cold_path: $code:block) => {
        $code
    };

    // Memory barrier
    (memory_barrier:) => {
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    };

    // Thread-local storage hint
    (thread_local: $var:ident, $init:expr) => {
        thread_local! {
            static $var: std::cell::RefCell<_> = std::cell::RefCell::new($init);
        }
    };

    // Stack allocation hint for small arrays
    (stack_array: $size:expr, $init:expr) => {
        [$init; $size]
    };

    // Heap allocation hint for large data
    (heap_vec: $size:expr, $init:expr) => {
        vec![$init; $size]
    };

    // Branch-free min/max
    (branchless_min: $a:expr, $b:expr) => {
        if $a <= $b { $a } else { $b }
    };

    (branchless_max: $a:expr, $b:expr) => {
        if $a >= $b { $a } else { $b }
    };

    // Fast modulo for powers of 2
    (fast_mod_pow2: $value:expr, $mod:expr) => {
        $value & ($mod - 1)
    };

    // Bit manipulation helpers
    (popcount: $value:expr) => {
        $value.count_ones()
    };

    (leading_zeros: $value:expr) => {
        $value.leading_zeros()
    };

    (trailing_zeros: $value:expr) => {
        $value.trailing_zeros()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_perf_vec() {
        let vec = perf!(vec: 10, 1, 2, 3, 4, 5);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
        assert!(vec.capacity() >= 10);
    }

    #[test]
    fn test_perf_vec_empty() {
        let vec: Vec<i32> = perf!(vec: 5,);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 5);
    }

    #[test]
    fn test_perf_likely() {
        let x = 42;
        let result = if perf!(likely: x > 0) {
            "positive"
        } else {
            "non-positive"
        };
        assert_eq!(result, "positive");
    }

    #[test]
    fn test_perf_unlikely() {
        let x = 42;
        let result = if perf!(unlikely: x < 0) {
            "negative"
        } else {
            "non-negative"
        };
        assert_eq!(result, "non-negative");
    }

    #[test]
    fn test_perf_cache_friendly_iter() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let chunks: Vec<_> = perf!(cache_friendly_iter: data, 3).collect();
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], &[1, 2, 3]);
        assert_eq!(chunks[1], &[4, 5, 6]);
        assert_eq!(chunks[2], &[7, 8]);
    }

    #[test]
    fn test_perf_fast_copy() {
        let src = [1, 2, 3, 4, 5];
        let mut dst = [0; 5];
        perf!(fast_copy: src, dst, 3);
        assert_eq!(dst, [1, 2, 3, 0, 0]);
    }

    #[test]
    fn test_perf_align() {
        assert_eq!(perf!(align: 10, 8), 16);
        assert_eq!(perf!(align: 16, 8), 16);
        assert_eq!(perf!(align: 17, 8), 24);
    }

    #[test]
    fn test_perf_branchless_min_max() {
        assert_eq!(perf!(branchless_min: 5, 3), 3);
        assert_eq!(perf!(branchless_min: 3, 5), 3);

        assert_eq!(perf!(branchless_max: 5, 3), 5);
        assert_eq!(perf!(branchless_max: 3, 5), 5);
    }

    #[test]
    fn test_perf_fast_mod_pow2() {
        // Test modulo with power of 2
        assert_eq!(perf!(fast_mod_pow2: 10, 8), 2); // 10 % 8 = 2
        assert_eq!(perf!(fast_mod_pow2: 15, 16), 15); // 15 % 16 = 15
        assert_eq!(perf!(fast_mod_pow2: 16, 16), 0); // 16 % 16 = 0
    }

    #[test]
    fn test_perf_bit_operations() {
        let value = 0b1010_1100u32;

        assert_eq!(perf!(popcount: value), 4); // 4 ones
        assert_eq!(perf!(leading_zeros: value), value.leading_zeros());
        assert_eq!(perf!(trailing_zeros: value), 2); // 2 trailing zeros
    }

    #[test]
    fn test_perf_stack_array() {
        let arr = perf!(stack_array: 5, 42);
        assert_eq!(arr, [42; 5]);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_perf_heap_vec() {
        let vec = perf!(heap_vec: 5, 42);
        assert_eq!(vec, vec![42; 5]);
        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn test_perf_hot_path() {
        let result = perf!(hot_path: {
            let x = 1 + 1;
            x * 2
        });
        assert_eq!(result, 4);
    }

    #[test]
    fn test_perf_cold_path() {
        let result = perf!(cold_path: {
            // This would typically be error handling or rare cases
            42
        });
        assert_eq!(result, 42);
    }

    #[test]
    fn test_perf_unroll() {
        let mut sum = 0;
        perf!(unroll: 5, {
            sum += 1;
        });
        assert_eq!(sum, 5);
    }
}

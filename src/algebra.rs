//! This module provides basic algebraic functions such as factorial and Fibonacci sequence 
//! computation. These functions are commonly used in various mathematical and algorithmic 
//! contexts.

/// Computes the factorial of a given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer whose factorial is to be computed. The function uses 
///   64-bit unsigned integers (`u64`), which means that the input must be a non-negative 
///   integer within the range of 0 to 20 (inclusive) to avoid overflow. The factorial of 
///   21 and above will result in an overflow as the result exceeds the maximum value 
///   that can be stored in a `u64`.
///
/// # Returns
///
/// * The factorial of the input number `n` as a `u64`. If `n` is 0 or 1, the function 
///   returns 1, as defined by the mathematical convention.
///
/// # Examples
///
/// ```rust 
/// assert_eq!(factorial(5), 120);
/// assert_eq!(factorial(0), 1);
/// ```
///
/// # Panics
///
/// The function does not explicitly panic but will return incorrect results if `n` is 
/// greater than 20 due to overflow.
///
/// # Performance
///
/// This implementation uses an iterative approach with Rust's built-in iterator and 
/// product method to compute the factorial, which is generally efficient in terms of 
/// both time and space complexity.
///
/// # Usage
///
/// The factorial function is often used in combinatorial mathematics, such as in the 
/// calculation of permutations and combinations, as well as in algorithms requiring 
/// factorial-based calculations.
///
/// # Limitations
///
/// The function is limited by the size of `u64`, which means it is only suitable for 
/// small values of `n`. For larger values, consider using a library that supports 
/// arbitrary-precision arithmetic.
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Computes the `n`-th Fibonacci number.
///
/// # Arguments
///
/// * `n` - The position in the Fibonacci sequence for which to compute the value. The 
///   function uses 64-bit unsigned integers (`u64`), and `n` must be a non-negative 
///   integer. The Fibonacci sequence starts with `fib(0) = 0` and `fib(1) = 1`.
///
/// # Returns
///
/// * The `n`-th Fibonacci number as a `u64`. The function returns `0` for `n = 0` and `1` 
///   for `n = 1`. For larger values of `n`, it recursively computes the sum of the two 
///   preceding numbers in the sequence.
///
/// # Examples
///
/// ```rust
/// assert_eq!(fibonacci(5), 5);
/// assert_eq!(fibonacci(10), 55);
/// ```
///
/// # Panics
///
/// The function may exhibit stack overflow for large values of `n` due to its recursive 
/// nature. Rust does not automatically optimize for tail-recursion, so large `n` can 
/// cause the stack to grow significantly, leading to a potential overflow.
///
/// # Performance
///
/// This implementation uses a simple recursive approach, which has exponential time 
/// complexity `O(2^n)`. For large values of `n`, this function is inefficient, and a 
/// more optimized approach, such as using dynamic programming or memoization, would be 
/// advisable.
///
/// # Usage
///
/// The Fibonacci sequence appears in various areas of mathematics, computer science, 
/// and nature. It is commonly used in algorithms, recursive data structures, and 
/// algorithm analysis.
///
/// # Limitations
///
/// This recursive implementation is not suitable for large values of `n` due to its 
/// inefficiency and the risk of stack overflow. Consider alternative implementations 
/// for performance-critical applications or large `n`.
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// Computes the Greatest Common Divisor (GCD) of a list of non-negative integers using the Euclidean algorithm.
/// 
/// The GCD of two or more integers is the largest positive integer that divides each of the integers without leaving a remainder.
/// This implementation leverages the Euclidean algorithm, which is based on the principle that the GCD of two numbers also divides their difference.
/// The algorithm works by successively replacing the larger number by the remainder of dividing the two numbers, until one of them becomes zero. 
/// At that point, the other number is the GCD. This method is efficient and works well even for large integers.
/// 
/// This function accepts a list of integers (in the form of a slice) and iteratively computes the GCD of all the elements.
/// It uses the `reduce` function to apply the GCD operation over the entire slice, ensuring that the GCD is calculated pairwise.
/// 
/// # Arguments
/// 
/// * `numbers` - A slice of unsigned integers (`&[u32]`). The slice can contain any number of elements, and the function
/// will return the GCD of all the elements. If the slice is empty, the function returns `0`.
/// 
/// # Returns
/// 
/// * A `u32` representing the greatest common divisor of the integers in the input slice. If the slice is empty, the function returns `0`.
/// If there is only one element in the slice, the function will return that element itself, as the GCD of a single number is the number itself.
/// 
/// # Edge Cases
/// 
/// * An empty slice will result in a GCD of `0`, as there are no numbers to compute the GCD from.
/// * A slice containing a single element will return that element, as the GCD of a single number is trivially the number itself.
/// * If all elements in the slice are `0`, the GCD will also be `0` as `0` is not divisible by any number.
/// * If there is at least one non-zero number in the slice, the GCD will always be at least `1`, as `1` is a divisor of all integers.
/// 
/// # Panics
/// 
/// This function does not panic as it handles empty slices gracefully and avoids potential overflow or division errors.
/// 
/// # Time Complexity
/// 
/// The time complexity of the Euclidean algorithm for two numbers is `O(log(min(a, b)))`, where `a` and `b` are the two numbers. 
/// Since this function applies the algorithm across the entire slice, the overall complexity is approximately `O(n * log(min(a, b, c, ...)))`,
/// where `n` is the number of integers in the slice. This makes it efficient for large datasets.
/// 
/// # Examples
/// 
/// ```rust
/// let numbers = vec![48, 18, 30];
/// let result = gcd(&numbers);
/// assert_eq!(result, 6);
/// 
/// let numbers = vec![101, 103, 107];
/// let result = gcd(&numbers);
/// assert_eq!(result, 1);  // The numbers are prime, so the GCD is 1.
/// 
/// let numbers = vec![0, 0, 0];
/// let result = gcd(&numbers);
/// assert_eq!(result, 0);  // All zeros result in a GCD of 0.
/// 
/// let numbers = vec![48];
/// let result = gcd(&numbers);
/// assert_eq!(result, 48);  // GCD of a single number is the number itself.
/// ```
/// 
/// # References
/// 
/// * [Euclidean Algorithm - Wikipedia](https://en.wikipedia.org/wiki/Euclidean_algorithm)
/// 
pub fn gcd(numbers: &[u32]) -> u32 {
    numbers.iter().cloned().reduce(|a, b| gcd_two(a, b)).unwrap_or(0)
}

/// Computes the GCD of two non-negative integers using the Euclidean algorithm.
///
/// This is a helper function used by the `gcd` function to compute the GCD of two numbers.
fn gcd_two(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

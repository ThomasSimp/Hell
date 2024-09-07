use hell::gcd::gcd;

fn main() {
    let numbers = vec![90, 20, 30];
    let result = gcd(&numbers);
    println!("The GCD of {:?} is {}", numbers, result);
}

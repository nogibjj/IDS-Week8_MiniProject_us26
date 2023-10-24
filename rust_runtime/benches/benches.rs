use std::time::Instant;

fn main() {
    let start = Instant::now();
    let n: u128 = 10_000_000;
    let sum_of_squares: u128 = (n * (n + 1) * (2 * n + 1)) / 6;
    let duration = start.elapsed();
    println!("Sum of squares: {}", sum_of_squares);
    println!("Time taken: {} nanoseconds", duration.as_nanos());
}
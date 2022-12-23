// The sum of the squares of the first ten natural numbers is 385
// The square of the sum of the first ten natural numbers is 3025
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn sum_square_diff(bound: u64) -> u64 {
    let sum_of_squares = (1..=bound).fold(0, |acc, x| acc + (x * x));
    let square_of_sum = {
        let a: u64 = (1..=bound).sum();
        a * a
    };
    square_of_sum - sum_of_squares
}

fn main() {
    println!("{}", sum_square_diff(100));
}

// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

// Works only for 6 digit numbers, but it fits for this problem
fn is_palindrome(product: &u32) -> bool {
    (product / 100_000) % 10 == product % 10
        && (product / 10000) % 10 == (product / 10) % 10
        && (product / 1000) % 10 == (product / 100) % 10
}

fn get_max_pal() -> u32 {
    let mut largest: u32 = 0;
    for a in (111..=999).rev() {
        if let Some(x) = (111..=a)
            .rev()
            .map(|b| a * b)
            .filter(|x| x > &largest)
            .find(is_palindrome)
        {
            largest = x;
        }
    }
    largest
}

fn main() {
    println!("{:?}", get_max_pal());
}

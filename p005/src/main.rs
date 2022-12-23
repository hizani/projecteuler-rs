// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// LCM(1..n+1) = LCM(1..n) * (n+1) / GCD(LCM(1..n), n+1)
fn lcm(bound: u64) -> u64 {
    if bound < 1 {
        0
    } else {
        (1..=bound).fold(1, |acc, x| acc * x / gcd(acc, x))
    }
}

const BOUND: u64 = 20;
fn main() {
    println!("{}", lcm(BOUND));
    println!("{}", gcd(6, 4));
}

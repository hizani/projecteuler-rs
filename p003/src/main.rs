// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

const NUMBER: u64 = 600851475143;

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut a = Vec::new();
    while n % 2 == 0 {
        n /= 2;
        a.push(2);
    }
    let n_copy = n;
    for i in (3..).step_by(2).take_while(|i| i * i <= n_copy) {
        while n % i == 0 {
            a.push(i);
            n /= i;
        }
    }
    if n > 2 {
        a.push(n);
    }
    a
}

fn main() {
    println!("{}", factors(NUMBER).last().unwrap());
}

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?

// Implement iterator over prime numbers
struct Prime {
    curr: u64,
    next1: u64,
    next2: u64,
    next3: u64,
}

impl Default for Prime {
    fn default() -> Self {
        Self {
            curr: 2,
            next1: 3,
            next2: 5,
            next3: 7,
        }
    }
}

impl Iterator for Prime {
    type Item = u64;

    // iterator uses the fact that all primes greater than 3
    // are of the form 6k +- 1, where k is any integer greater than 0
    fn next(&mut self) -> Option<Self::Item> {
        let prime = self.curr;
        self.curr = self.next1;
        loop {
            self.next1 = self.next2;
            self.next2 = self.next3;
            self.next3 = self.next1 + 6;
            if is_prime(self.next1) {
                break;
            }
        }
        Some(prime)
    }
}

fn is_prime(n: u64) -> bool {
    if n < 4 {
        n > 1
    } else if n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        // check if n is evenly divisible by any 6k +- 1 up to the square root of n
        let max_p = (n as f64).sqrt().ceil() as u64;
        match (5..=max_p)
            .step_by(6)
            .find(|p| n % p == 0 || n % (p + 2) == 0)
        {
            Some(_) => false,
            None => true,
        }
    }
}

fn main() {
    println!("{}", is_prime(16));
    println!("{}", Prime::default().take(5).last().unwrap());
}

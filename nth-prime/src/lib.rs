use std::iter::Iterator;

pub fn nth(n: u32) -> u32 {
    PrimeSieve::new()
        .nth(n as usize)
        .expect("too infinity and beyond") as u32
}

struct PrimeSieve {
    past_primes: Vec<u64>,
}

impl PrimeSieve {
    fn new() -> Self {
        Self {
            past_primes: Vec::new(),
        }
    }
}

impl Iterator for PrimeSieve {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_prime = match self.past_primes.last() {
            None => 2,
            Some(&prime) => (prime..).find(|n| self.past_primes.iter().all(|p| n % p != 0))?,
        };

        self.past_primes.push(next_prime);
        Some(next_prime)
    }
}

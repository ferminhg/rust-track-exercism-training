use primes::{Sieve, PrimeSet};

pub fn nth(n: u32) -> u32 {
    let mut pset = Sieve::new();
    let prime_position: usize = (n+1) as usize;

    for (pos, prime) in pset.iter().enumerate().take(prime_position) {
        if pos as u32 == n { return prime as u32; }
    }
    panic!("Prime not found");
}

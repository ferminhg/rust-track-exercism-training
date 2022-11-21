use primes::{Sieve, PrimeSet, is_prime};

pub fn factors(n: u64) -> Vec<u64> {
    let mut primes = vec![];
    let mut number = n;
    
    while number >= 2 {
        if is_prime(number) {
            primes.push(number);
            break;
        }
        let prime = min_factor(number);
        primes.push(prime);
        number = number / prime;
    }

    primes
}

pub fn min_factor(n: u64) -> u64{
    let mut pset = Sieve::new();
    let mut number = n;
    
    for (_, prime) in pset.iter().enumerate() {
        if prime > n { break; }
        if n % prime == 0 {
            number = prime;
            break;
        }
    }
    number
}

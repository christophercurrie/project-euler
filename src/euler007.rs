use crate::primes::primes;

pub fn euler_007(nth: usize) -> i64 {
    let mut factor = 2;
    loop {
        let found: Vec<i64> = primes((nth * factor) as i64).collect();
        if nth < found.len() {
            return found[nth - 1];
        }
        factor += 1;
    }
    // Successors turns out to be super slow vs a correct sieve implementation.
    // successors(Some((2_i64, vec![2_i64], true)), |(prev, primes, _)| {
    //     let next = prev + 1;
    //     let is_prime = !primes.iter().any(|n| next % n == 0);
    //     let mut new_p = primes.clone();
    //     if is_prime { new_p.push(next); }
    //     Some((next, new_p, is_prime))
    // }).filter(
    //     |(_, _, is_prime)| *is_prime
    // ).take(nth).last().unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler007() {
        assert_eq!(euler_007(6), 13);
        assert_eq!(euler_007(10001), 104743);
    }
}

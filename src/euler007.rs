use std::iter::successors;

pub fn euler_007(nth: usize) -> i64 {
    successors(Some((2_i64, vec![2_i64], true)), |(prev, primes, _)| {
        let next = prev + 1;
        let is_prime = !primes.iter().any(|n| next % n == 0);
        let mut new_p = primes.clone();
        if is_prime { new_p.push(next); }
        Some((next, new_p, is_prime))
    }).filter(
        |(_, _, is_prime)| *is_prime
    ).take(nth).last().unwrap().0
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

use crate::primes::primes;

pub fn euler_010(less_than: u64) -> u64 {
    primes(less_than).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler010() {
        assert_eq!(euler_010(10), 17);
        assert_eq!(euler_010(2_000_000), 142913828922);
    }
}

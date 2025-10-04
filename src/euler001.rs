use std::collections::HashSet;
use std::iter::successors;
use itertools::{Itertools, merge};

pub fn euler001_limit(limit: i32) -> i32 {
    let mut values: HashSet<i32> = HashSet::new();

    let mut product: i32 = 3;
    while product < limit {
        values.insert(product);
        product += 3;
    }

    product = 5;
    while product < limit {
        values.insert(product);
        product += 5;
    }

    values.iter().sum()
}

pub fn euler001_limit2(limit: i32) -> i32 {
    let multiples_of = |i| successors(Some(i), move |n| Some(n + i));
    merge(multiples_of(3), multiples_of(5)).unique().take_while(|n| *n < limit).sum()
}

pub fn euler001() {
    let result = euler001_limit2(1000);
    println!("Euler 001: {}", result);
}

#[cfg(test)]
mod tests {
    use super::euler001_limit2;

    #[test]
    fn test_limit_10() {
        assert_eq!(euler001_limit2(10), 23);
    }

    #[test]
    fn test_limit_49() {
        assert_eq!(euler001_limit2(49), 543);
    }

    #[test]
    fn test_limit_1000() {
        assert_eq!(euler001_limit2(1_000), 233_168);
    }

    #[test]
    fn test_limit_8456() {
        assert_eq!(euler001_limit2(8_456), 16_687_353);
    }

    #[test]
    fn test_limit_19564() {
        assert_eq!(euler001_limit2(19_564), 89_301_183);
    }
}

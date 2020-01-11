use std::collections::HashSet;

fn euler001_limit(limit: i32) -> i32 {
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

pub fn euler001() {
    let result = euler001_limit(1000);
    println!("Euler 001: {}", result);
}

#[cfg(test)]
mod tests {
    use super::euler001_limit;

    #[test]
    fn test_limit_10() {
        assert_eq!(euler001_limit(10), 23);
    }

    #[test]
    fn test_limit_49() {
        assert_eq!(euler001_limit(49), 543);
    }

    #[test]
    fn test_limit_1000() {
        assert_eq!(euler001_limit(1000), 233168);
    }

    #[test]
    fn test_limit_8456() {
        assert_eq!(euler001_limit(8456), 16687353);
    }

    #[test]
    fn test_limit_19564() {
        assert_eq!(euler001_limit(19564), 89301183);
    }
}
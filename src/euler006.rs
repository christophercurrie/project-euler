pub fn euler_006(n: i64) -> i64 {
    let sum_squares: i64 = (1..=n).map(|n| n.pow(2)).sum();
    let square_sum: i64 = ((1..=n).sum::<i64>()).pow(2);
    square_sum - sum_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler006() {
        assert_eq!(euler_006(10), 2640);
        assert_eq!(euler_006(100), 25164150);
    }
}

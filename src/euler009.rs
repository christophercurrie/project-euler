pub fn euler_009() -> Result<i64,()> {
    // a can't be more that a third of the sum and still be less than b and c
    for a in 1_i64..333 {
        // b must be less than half the difference to be less than c
        for b in (a+1)..(1000 - a)/2 {
            let c = 1000 - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Ok(a * b * c);
            }
        }
    }
    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler009() {
        assert_eq!(euler_009(), Ok(31875000));
    }
}

pub fn euler_005(n: i64) -> i64 {
    // Stupid version just to get the result
    let mut result = 1_i64;

    loop {
        let mut x = 1;
        while x <= n && result % x == 0 {
            x += 1;
        }
        if x > n {
            break result;
        }
        result += 1;
    }
}

pub fn euler_005_less_stupid(n: i64) -> i64 {
    let mut result = n;

    for factor in 2..n {
        if result % factor != 0 {
            let mut result_factor = result;
            let mut factor_factor = factor;
            let mut remainder = 1_i64;
            for divisor in 2..=factor {
                while factor_factor % divisor == 0 {
                    if result_factor % divisor == 0 {
                        result_factor /= divisor;
                    }
                    else {
                        remainder *= divisor;
                    }
                    factor_factor /= divisor;
                }
            }
            result *= remainder;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // because it's really slow
    fn test_euler005() {
        test_euler005_fn(euler_005);
    }

    #[test]
    fn test_euler_005_less_stupid() {
        test_euler005_fn(euler_005_less_stupid);
    }

    fn test_euler005_fn(f: impl Fn(i64) -> i64) {
        assert_eq!(f(20), 232792560);
    }

}

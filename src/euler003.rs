use std::iter::successors;

pub fn euler_003(n: i64) -> Option<i64> {
    if n < 2 {
        return None;
    }
    let mut result = n;
    Some(loop {
        let mut div = 2;
        while div < result && result % div != 0 {
            div += 1;
        }
        if div == result {
            break result;
        }
        result = result / div;
    })
}

pub fn euler_003_optimized(n: i64) -> Option<i64> {
    if n < 2 {
        return None;
    }
    let mut result = 1;
    let mut composite = n;
    while composite % 2 == 0 {
        result = 2;
        composite = composite / 2;
    }
    let mut div = 3;
    while div * div < composite {
        if composite % div == 0 {
            result = div;
            composite = composite / div;
        }
        else {
            div += 2;
        }
    }
    if composite > 1 {
        return Some(composite);
    }
    Some(result)
}

pub fn euler_003_functional(n: i64) -> Option<i64> {
    successors(Some((n, 2_i64, 1, None)), |(composite, div, incr, result)|
        if div * div > *composite { None }
        else if composite % div == 0 { Some((composite / div, *div, *incr, Some(*div)))}
        else { Some((*composite, div + incr, 2, *result)) }
    ).last().and_then(|(composite, _, _, result)|
        if composite > 1 { Some(composite) } else { result }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler003() {
        test_euler003_fn(euler_003);
        test_euler003_fn(euler_003_optimized);
        test_euler003_fn(euler_003_functional);
    }

    fn test_euler003_fn(f: impl Fn(i64) -> Option<i64>) {
        assert_eq!(f(1), None);
        assert_eq!(f(-2), None);
        assert_eq!(f(2), Some(2));
        assert_eq!(f(600_851_475_143), Some(6857));
    }
}

pub fn euler_004() -> i64 {
    let mut largest: i64 = 0;
    for x in 100..1000 {
        for y in 100..1000 {
            let product = x * y;
            if product > largest {
                let product_string = format!("{}", product);
                let reversed: String = product_string.chars().rev().collect();
                if product_string.eq(&reversed) {
                    largest = product;
                }
            }
        }
    }
    largest
}

pub fn euler_004_optimized() -> i64 {
    let mut largest: i64 = 0;
    for x in (100..1000).rev() {
        for y in (x..1000).rev() {
            let product = x * y;
            if product < largest {
                break;
            }
            let product_string = format!("{}", product);
            let reversed: String = product_string.chars().rev().collect();
            if product_string.eq(&reversed) {
                largest = product;
            }
        }
    }
    largest
}

pub fn euler_004_optimized2() -> i64 {
    let mut largest: i64 = 0;
    for x in (100..1000).rev() {
        let (max_y, step) = if x % 11 == 0 { (999, 1) } else { (990, 11) };
        for y in (x..max_y).rev().step_by(step) {
            let product = x * y;
            if product < largest {
                break;
            }
            let product_string = format!("{}", product);
            let reversed: String = product_string.chars().rev().collect();
            if product_string.eq(&reversed) {
                largest = product;
            }
        }
    }
    largest
}

pub fn euler_004_functional() -> i64 {
    (100..1000).rev().fold(0, |largest, x| {
        let (max_y, step) = if x % 11 == 0 { (999, 1) } else { (990, 11) };
        (x..max_y).rev().step_by(step).try_fold(largest, |largest, y| {
            let product = x * y;
            if product < largest {
                return Err(largest);
            }

            let product_string = format!("{}", product);
            let reversed: String = product_string.chars().rev().collect();
            if product_string.eq(&reversed) {
                Ok(product)
            } else {
                Ok(largest)
            }
        }).unwrap_or_else(|err| err)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler004() {
        test_euler004_fn(euler_004);
        test_euler004_fn(euler_004_functional);
    }

    #[test]
    fn test_euler004_optimized() {
        test_euler004_fn(euler_004_optimized);
    }

    #[test]
    fn test_euler004_optimized2() {
        test_euler004_fn(euler_004_optimized2);
    }

    #[test]
    fn test_euler004_functional() {
        test_euler004_fn(euler_004_functional);
    }

    fn test_euler004_fn(f: impl Fn() -> i64) {
        assert_eq!(f(), 906609);
    }
}

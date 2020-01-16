use std::iter::successors;

#[allow(dead_code)]
pub fn euler002() -> i32 {
    let mut first: i32 = 1;
    let mut second: i32 = 2;
    let mut sum: i32 = 0;

    while second < 4_000_000 {
        if second % 2 == 0 {
            sum += second;
        }
        let next = first + second;
        first = second;
        second = next;
    }

    sum
}

pub fn euler002_functional() -> i32 {
    let fib  = successors(Some((1, 2)), |(n, m)| Some((*m, n + m)));
    fib.map(|(_, n)| n).filter(|n| n % 2 == 0).take_while(|&n| n < 4_000_000).sum()
}

pub fn euler002_free_code_camp(nth_term: usize) -> i32 {
    let fib  = successors(Some((1, 2)), |(n, m)| Some((*m, n + m)));
    fib.map(|(_, n)| n).take(nth_term - 1).filter(|n| n % 2 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::euler002_free_code_camp;

    #[test]
    fn test_limit_10() {
        assert_eq!(euler002_free_code_camp(10), 44);
    }

    #[test]
    fn test_limit_18() {
        assert_eq!(euler002_free_code_camp(18), 3382);
    }

    #[test]
    fn test_limit_23() {
        assert_eq!(euler002_free_code_camp(23), 60696);
    }

    #[test]
    fn test_limit_43() {
        assert_eq!(euler002_free_code_camp(43), 350704366);
    }
}

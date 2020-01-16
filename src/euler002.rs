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
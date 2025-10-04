use std::collections::HashMap;

fn euler_014_cached(limit: usize) -> usize {
    let mut data: HashMap<usize, usize> = HashMap::with_capacity(limit * 2);
    data.insert(1, 1);
    let mut max: usize = 1;

    fn compute(d: &mut HashMap<usize, usize>, n: usize) -> usize {
        match d.get(&n) {
            Some(length) => *length,
            None => {
                let next = match n {
                    _ if n % 2 == 0 => n / 2,
                    _ => 3 * n + 1
                };
                let len = compute(d, next) + 1;
                d.insert(n, len);
                len
            }
        }
    }

    for n in 2..limit {
        let len = compute(&mut data, n);
        if len > data[&max] {
            max = n;
        }
    }
    max
}

fn euler_014_loops(limit: u64) -> u64 {
    let mut max = 1_u64;
    let mut max_len = 1;

    for i in 2..limit {
        let mut n = i;
        let mut steps = 1;
        while n > 1 {
            steps += 1;
            n = match n {
                _ if n % 2 == 0 => n / 2,
                _ => 3 * n + 1
            };
        }
        if steps > max_len {
            max = i;
            max_len = steps;
        }
    }

    max
}

fn euler_014(max: u64) -> u64 {
    use std::iter::successors;
    let seq = |init| successors(Some(init), |n| match n {
        1 => None,
        _ if n % 2 == 0 => Some(n / 2),
        _ => Some(3 * n + 1)
    });

    (2..max).map(|n| (n, seq(n).count()))
        .max_by(|(_, l), (_, r)| l.cmp(r))
        .map(|(n, _)| n)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler014() {
        assert_eq!(euler_014(1_000_000), 837799);
    }

    #[test]
    fn test_euler014_loops() {
        assert_eq!(euler_014_loops(1_000_000), 837799);
    }

    #[test]
    fn test_euler014_cached() {
        assert_eq!(euler_014_cached(1_000_000), 837799);
    }
}

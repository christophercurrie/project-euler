use std::collections::HashSet;
use std::iter::successors;

// use itertools::Itertools;
// use crate::primes::primes;

fn triangle_numbers() -> impl Iterator<Item=u64> {
    successors(Some((1_u64,1_u64)), |(n,s)| Some((n+1, s+n+1)))
        .map(|(_,s)| s)
}

// fn euler_012_slow(min_divisors: u16) -> u64 {
//     triangle_numbers().map(|n| {
//         let mut prime_factors: Vec<u64> = vec![];
//         if n > 2 {
//             primes(n).for_each(|p| {
//                 let mut r = n;
//                 while r % p == 0 {
//                     prime_factors.push(p);
//                     r = r / p;
//                 }
//             });
//         }
//         let mut unique_factors: HashSet<u64> = HashSet::new();
//         unique_factors.insert(1);
//         unique_factors.insert(n);
//         for i in 1..prime_factors.len() {
//             prime_factors.iter().combinations(i).for_each(|c| {
//                 unique_factors.insert(c.iter().map(|x| *x).product());
//             });
//         }
//         (n, unique_factors.len())
//     }).filter(|(_,d)| *d > min_divisors as usize).next().unwrap().0
// }

pub fn euler_012(min_divisors: u16) -> u64 {
    triangle_numbers().map(|n| {
        let step = if n % 2 != 0 { 2 } else { 1 };
        let factors: HashSet<u64> = (1..((n as f64).sqrt() as u64 + 1))
            .step_by(step)
            .filter(|i| n % i == 0)
            .flat_map(|i| vec![i, n / i])
            .collect();
        (n, factors.len())
    }).filter(|(_,d)| *d > min_divisors as usize).next().unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler012() {
        assert_eq!(euler_012(5), 28);
        assert_eq!(euler_012(501), 76576500);
    }
}

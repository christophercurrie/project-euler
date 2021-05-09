pub fn primes(less_than: u64) -> impl Iterator<Item=u64> {
    let mut results = vec![true; less_than as usize];
    results[0] = false;
    results[1] = false;
    let mut p = 2_u64;
    while p * p < less_than {
        if results[p as usize] {
            let mut n = p;
            while p * n < less_than {
                results[(p * n) as usize] = false;
                n += 1;
            }
        }
        p += 1;
    }
    return results
        .into_iter()
        .enumerate()
        .filter(|(_, is_prime)| *is_prime)
        .map(|(i, _)| i as u64);
}

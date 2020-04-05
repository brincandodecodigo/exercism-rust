pub fn nth(n: u32) -> u32 {
    let n = n as usize;
    let mut primes: Vec<u32> = vec![2];
    let mut i: u32 = 3;
    while primes.len() <= n {
        if primes.iter().all(|x| i % x != 0) {
            primes.push(i);
        }
        i = i + 1;
    }
    primes[n]
}

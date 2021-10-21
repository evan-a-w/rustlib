use crate::bool_arr::BoolArr;

pub fn is_prime(n: usize) -> bool {
    if n == 2 || n == 3 {
        return true;
    }

    let mut i: usize = 5;
    while i * i < n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

pub fn sieve_bool(n: usize) -> BoolArr {
    let mut ba = BoolArr::new(n + 1, true);
    ba.set(0, false);
    ba.set(1, false);
    let mut i = 2;
    while i * i <= n {
        if ba.get(i) {
            for j in ((i * i)..=n).step_by(i) {
                ba.set(j, false);
            }
        }
        i += 1;
    }
    ba
}

pub fn segmented_sieve_bool(L: usize, R: usize) -> BoolArr {
    let lim = (R as f64).sqrt() as usize;
    let smaller_primes = sieve_bool(lim);
    let mut ba = BoolArr::new(R - L + 1, true);
    for i in 0..=lim {
        if smaller_primes.get(i) {
            // first multiple of i in range
            let mut f = L / i * i;
            if f < L {
                f += i;
            }
            while f <= R {
                ba.set(f - L, false);
                f += i;
            }
        }
    }
    ba
}

pub fn sieve(n: usize) -> Vec<usize> {
    let ba = sieve_bool(n);
    let mut res = vec![2];
    for i in (3..=n).step_by(2) {
        if ba.get(i) {
            res.push(i);
        }
    }
    res
}

pub fn segmented_sieve(L: usize, R: usize) -> Vec<usize> {
    let ba = segmented_sieve_bool(L, R);
    let mut res = vec![];
    for i in L..=R {
        if ba.get(i - L) {
            res.push(i);
        }
    }
    res
}
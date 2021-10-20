mod bool_arr;

use bool_arr::BoolArr;
use std::cmp::max;

#[cfg(test)]
mod tests {
    use crate::bool_arr::BoolArr;
    use crate::*;

    #[test]
    fn bool_arr_test() {
        let mut br = BoolArr::new(10, false);
        assert!(!br.get(5));
        br.set(5, true);
        assert!(br.get(5));
        br.push(true);
        assert!(br.get(10));
        br.push(false);
        assert!(!br.get(11));
        let br = BoolArr::new(10, true);
        for i in 0..10 {
            assert!(br.get(i));
        }
    }

    #[test]
    fn bool_arr_test_complex() {
        let mut br = BoolArr::new(100000, false);
        br.set(97, true);
        assert!(br.get(97));
        br.set(97, false);
        for i in 0..100000 {
            br.set(i, i % 2 == 0);
        }
        for i in 0..100000 {
            assert!(br.get(i) == (i % 2 == 0));
        }
        let mut br = BoolArr::new(100000, true);
        for i in 0..100000 {
            br.set(i, i % 2 == 0);
        }
        for i in 0..100000 {
            assert!(br.get(i) == (i % 2 == 0));
        }
        let mut br = BoolArr::new(1000000, false);
        let mut count = 0;
        br.set(12321, true);
        br.set(655637, true);
        for i in 0..1000000 {
            if br.get(i) {
                count += 1;
            }
        }
        assert!(count == 2);
    }

    #[test]
    fn sieve_bool_test() {
        let pb = sieve_bool(100);
        let p_to_100 = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43
                           , 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        let mut i = 0;
        for j in 0..=100 {
            if j == p_to_100[i] {
                if !pb.get(j) {
                    println!("fail at {} (not present)", j);
                }
                assert!(pb.get(j));
                i += 1;
            } else {
                if pb.get(j) {
                    println!("fail at {} (present but shouldnt be)", j);
                }
                assert!(!pb.get(j));
            }
        }
    }

    #[test]
    fn sieve_bool_vec_test() {
        let pb = sieve_bool_vec(100);
        let p_to_100 = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43
                           , 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        let mut i = 0;
        for j in 0..=100 {
            if i < 25 && j == p_to_100[i] {
                if !pb[j] {
                    println!("fail at {} (not present)", j);
                }
                assert!(pb[j]);
                i += 1;
            } else {
                if pb[j] {
                    println!("fail at {} (present but shouldnt be)", j);
                }
                assert!(!pb[j]);
            }
        }
    }

    #[test]
    fn sieve_test() {
        let primes = sieve(10);
        for i in primes.iter() {
            print!("{} ", i);
        }
        let p_to_10 = vec![2, 3, 5, 7];
        assert!(primes == p_to_10);
        let primes = sieve(100);
        let p_to_100 = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43
                           , 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        for i in primes.iter() {
            print!("{} ", i);
        }
        println!("");
        for i in p_to_100.iter() {
            print!("{} ", i);
        }
        println!("");
        assert!(primes == p_to_100);
    }
}

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

pub fn sieve_bool_vec(n: usize) -> Vec<bool> {
    let mut ba = vec![true; n + 1];
    ba[0] = false;
    ba[1] = false;
    let mut i = 2;
    while i * i <= n {
        if ba[i] {
            for j in ((i * i)..=n).step_by(i) {
                ba[j] = false;
            }
        }
        i += 1;
    }
    ba
}

pub fn segmented_sieve_bool(L: usize, R: usize) -> BoolArr {
    let mut ba = BoolArr::new(R - L + 1, true);
    let lim = (R as f64).sqrt() as usize;
    for i in (2..=lim).step_by(2) {
        let mut j = max(i * i, (L + i - 1) / i * i);
        while j <= R {
            ba.set(j - L, false);
            j += i;
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

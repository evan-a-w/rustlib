mod bool_arr;
mod primes;

use bool_arr::BoolArr;
use std::cmp::max;

#[cfg(test)]
mod tests {
    use crate::bool_arr::BoolArr;
    use crate::primes::*;

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
        let mut br = BoolArr::new(101, true);
        assert!(br.get(37));
        assert!(br.get(97));
    }

    #[test]
    fn sieve_bool_test() {
        let pb = sieve_bool(100);
        let p_to_100 = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43
                           , 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        let mut i = 0;
        for j in 0..=100 {
            if i < 25 && j == p_to_100[i] {
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

    #[test]
    fn seg_sieve_simple_test() {
        let primes = segmented_sieve(12, 100);
        let p_to_100 = vec![13, 17, 19, 23, 29, 31, 37, 41, 43
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

    #[test]
    fn seg_sieve_complex_test() {
        let primes = segmented_sieve(500, 1000);
        let p_500_to_1000 = vec![503, 509, 521, 523, 541, 547, 557, 563, 569
                                , 571, 577, 587, 593, 599, 601, 607, 613, 617
                                , 619, 631, 641, 643, 647, 653, 659, 661, 673
                                , 677, 683, 691, 701, 709, 719, 727, 733, 739
                                , 743, 751, 757, 761, 769, 773, 787, 797, 809
                                , 811, 821, 823, 827, 829, 839, 853, 857, 859
                                , 863, 877, 881, 883, 887, 907, 911, 919, 929
                                , 937, 941, 947, 953, 967, 971, 977, 983, 991
                                , 997];
        assert!(primes == p_500_to_1000);
    }
}

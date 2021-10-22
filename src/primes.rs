use crate::bool_arr::BoolArr;

pub fn is_prime(n: usize) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    
    if n & 1 == 0 {
        return false;
    }

    let mut i: usize = 3;
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
    let lim = (R as f64).sqrt().ceil() as usize;
    let smaller_primes = sieve_bool(lim);
    let mut ba = BoolArr::new(R - L + 1, true);
    for i in L..2 {
        ba.set(i - L, false);
    }
    for i in 2..=lim {
        if smaller_primes.get(i) {
            // first multiple of i in range
            let mut f = L / i * i;
            while f < L || f <= i {
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

pub fn segmented_sieve_till(n: usize) -> Vec<usize> {
    let mut del = (n as f64).sqrt().floor() as usize;
    let mut l: usize = del + 1;
    let mut h: usize = 2 * del; // go from del + 1 to 2*del
    let mut primes = sieve(del);

    let mut first_over: bool = true;
    while h <= n {
        let mut nbv = BoolArr::new(del, true);

        let max = (h as f64).sqrt().floor() as usize;
        for &i in &primes {
            if i > max {
                break;
            }
            let mut index_n = (((l as f64) / (i as f64)).ceil() as usize) * i - l;
            while index_n < del {
                nbv.set(index_n, false);
                index_n += i;
            }
        }

        for i in 0..del {
            if nbv.get(i) {
                primes.push(i + l);
            }
        }

        l = h + 1;
        h += del;

        if h > n && first_over {
            let dif = h - n;
            del -= dif;
            h = n;
            first_over = false;
        }
    }

    primes
}

pub fn prime_factors_big(mut n: usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    let mut i = 2;

    while n > 1 {
        if is_prime(i) && n % i == 0 {
            res.push(i);
            n /= i;
        }

        i += 1;
    }

    res
}

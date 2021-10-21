// Very simple big_bcd. Each digit is a u8, so space is not so efficient.
// Also, the algorithms used are very simple, and are far from optimal.

use std::convert::From;
use std::fmt;
use itertools::{
    Itertools,
    EitherOrBoth::*,
};
use std::cmp::{max, Eq, PartialEq, Ord, PartialOrd, Ordering};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BigBcd {
    digs: Vec<u8>,
}

impl From<String> for BigBcd {
    fn from(s: String) -> Self {
        let mut rv = vec![];
        rv.reserve(s.len());
        for c in s.chars().rev() {
            rv.push(c.to_digit(10).unwrap() as u8);
        }
        BigBcd { digs: rv }
    }
}

impl From<&str> for BigBcd {
    fn from(s: &str) -> Self {
        BigBcd::from(String::from(s))
    }
}

impl From<usize> for BigBcd {
    fn from(mut x: usize) -> Self {
        let mut rv = vec![];
        while x > 0 {
            rv.push((x % 10) as u8);
            x /= 10;
        }
        if rv.len() == 0 {
            rv.push(0);
        }
        BigBcd { digs: rv }
    }
}

impl fmt::Display for BigBcd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.digs
                .iter()
                .rev()
                .map(|x| (x + '0' as u8) as char)
                .collect::<String>()
        )
    }
}

impl BigBcd {
    pub fn add(&self, other: &Self) -> Self {
        let mut res_vec: Vec<u8> = vec![];
        res_vec.reserve(max(self.digs.len(), other.digs.len()));

        let mut i = 0;
        let mut carry: u8 = 0;

        for pair in self.digs.iter().zip_longest(other.digs.iter()) {
            match pair {
                Both(x, y) => {
                    let mut curr: u8 = x + y + carry;
                    carry = curr / 10;
                    curr %= 10;
                    res_vec.push(curr);
                }
                Left(x) => {
                    let mut curr: u8 = x + carry;
                    carry = curr / 10;
                    curr %= 10;
                    res_vec.push(curr);
                }
                Right(y) => {
                    let mut curr: u8 = y + carry;
                    carry = curr / 10;
                    curr %= 10;
                    res_vec.push(curr);
                }
            };
        }

        if carry > 0 {
            res_vec.push(carry);
        }

        BigBcd { digs: res_vec }
    }

    pub fn subtract(&self, other: &Self) -> Self {
        let mut res_vec: Vec<u8> = vec![];
        res_vec.reserve(self.digs.len());

        let mut carry: u8 = 0;

        for pair in self.digs.iter().zip_longest(other.digs.iter()) {
            match pair {
                Both(x, y) => {
                    let mut curr: i8 = (*x as i8) - (carry as i8) - (*y as i8);
                    carry = 0;
                    if curr < 0 {
                        carry = 1;
                        curr += 10;
                    }
                    res_vec.push(curr as u8);
                }
                Left(x) => {
                    let mut curr: i8 = (*x as i8) - (carry as i8);
                    carry = 0;
                    if curr < 0 {
                        carry = 1;
                        curr += 10;
                    }
                    res_vec.push(curr as u8);
                }
                Right(y) => {
                    // underflow
                    return BigBcd { digs: vec![0] };
                }
            };
        }

        for i in (1..res_vec.len()).rev() {
            if res_vec[i] == 0 {
                res_vec.pop();
            } else {
                break;
            }
        }

        BigBcd { digs: res_vec }
    }

    fn dig_mul(&mut self, large: &Self, small: &Self, place: usize) {
        let dig: u8 = small.digs[place];
        if dig == 0 {
            return;
        }
        
        let mut i: usize = 0;
        let mut carry: u8 = 0;
        while i < large.digs.len() {
            let mut curr: u8 = large.digs[i] * dig + carry;
            if i + place < self.digs.len() {
                curr += self.digs[i + place];
            } else {
                for _ in self.digs.len()..=(i + place) {
                    self.digs.push(0);
                }
            }
            carry = curr / 10;
            curr %= 10;
            self.digs[i + place] = curr;
            i += 1;
        }
        while carry > 0 {
            let mut curr = carry;
            if i + place < self.digs.len() {
                curr += self.digs[i + place];
            } else {
                for _ in self.digs.len()..(i + place) {
                    self.digs.push(0);
                }
            }
            carry = curr / 10;
            curr %= 10;
            self.digs[i + place] = curr;
            i += 1;
        }
    }

    pub fn multiply(&self, other: &Self) -> Self {
        let small;
        let large;
        if self.digs.len() > other.digs.len() {
            large = self;
            small = other;
        } else {
            large = other;
            small = self;
        }

        let mut res = BigBcd { digs: vec![0] };

        for place in 0..small.digs.len() {
            res.dig_mul(large, small, place);
        }

        res
    }

    fn sub_pow_inplace(&mut self, other: &Self, pow: usize) -> bool {
        let mut borrow: i8 = 0;
        let mut i: usize = 0;
        while i < other.digs.len() {
            let mut curr: i8 = (self.digs[i + pow] as i8) - (other.digs[i] as i8)  
                               - borrow;
            borrow = (curr < 0) as i8;
            curr += borrow * 10;
            self.digs[i + pow] = curr as u8;
            i += 1;
        }
        i += pow;

        while borrow > 0 && i < self.digs.len() {
            let mut curr: i8 = (self.digs[i + pow] as i8) - borrow;
            borrow = (curr < 0) as i8;
            curr += borrow * 10;
            self.digs[i + pow] = curr as u8;
            i += 1;
        }
        for i in (1..self.digs.len()).rev() {
            if self.digs[i] == 0 {
                self.digs.pop();
            } else {
                break;
            }
        }
        borrow > 0
    }

    fn greater_eq_pow(&self, other: &Self, pow: usize) -> bool {
        let diff = (self.digs.len() as isize) - (other.digs.len() as isize) - (pow as isize);
        if diff < 0 {
            false
        } else if diff > 0 {
            true
        } else {
            for i in (0..other.digs.len()).rev() {
                let d = (self.digs[i + pow] as i8) - (other.digs[i] as i8);
                if d > 0 {
                    return true;
                } else if d < 0 {
                    return false;
                }
            }

            true
        }
    }

    pub fn divide(&self, other: &Self) -> Self {
        let mut res = BigBcd::from(0);
        if self.digs.len() >= other.digs.len() {
            res.digs = vec![0; self.digs.len()];
            let mut n = self.clone();
            for pow in (0..=(self.digs.len() - other.digs.len())).rev() {
                res.digs[pow] = 0;
                while n.greater_eq_pow(other, pow) {
                    n.sub_pow_inplace(other, pow);
                    res.digs[pow] += 1;
                }
            }
            for i in (1..(res.digs.len() - 1)).rev() {
                if res.digs[i] == 0 {
                    res.digs.pop();
                } else {
                    break;
                }
            }
        }

        res
    }

    pub fn div_rem(&self, other: &Self) -> (Self, Self) {
        let mut res = BigBcd::from(0);
        if self.digs.len() >= other.digs.len() {
            res.digs = vec![0; self.digs.len()];
            let mut n = self.clone();
            for pow in (0..=(self.digs.len() - other.digs.len())).rev() {
                res.digs[pow] = 0;
                while n.greater_eq_pow(other, pow) {
                    n.sub_pow_inplace(other, pow);
                    res.digs[pow] += 1;
                }
            }
            for i in (1..res.digs.len()).rev() {
                if res.digs[i] == 0 {
                    res.digs.pop();
                } else {
                    break;
                }
            }
            (res, n)
        } else {
            (res, self.clone())
        }
    }

    pub fn rem(&self, other: &Self) -> Self {
        if self.digs.len() >= other.digs.len() {
            let mut n = self.clone();
            for pow in (0..=(self.digs.len() - other.digs.len())).rev() {
                while n.greater_eq_pow(other, pow) {
                    n.sub_pow_inplace(other, pow);
                }
            }
            n
        } else {
            self.clone()
        }
    }
}

impl Ord for BigBcd {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.digs.len() > other.digs.len() {
            Ordering::Greater
        } else if self.digs.len() == other.digs.len() {
            for (x, y) in self.digs.iter().zip(other.digs.iter()).rev() {
                let d = (*x as i8) - (*y as i8);
                if d > 0 {
                    return Ordering::Greater;
                } else if d < 0 {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for BigBcd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

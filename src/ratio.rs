use num_traits::Num;
use std::cmp::{Eq, PartialEq, PartialOrd, Ord, Ordering};
use std::ops::{Add, Sub, Mul, Div, Rem};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ratio<T: Num + Ord + Copy> {
    pub numerator: T,
    pub denominator: T,
}

pub fn gcd<T: Num + Ord + Copy>(a: T, b: T) -> T {
    let z = T::zero();

    if a == z {
        b
    } else if b == z || a == b {
        a
    } else if a > b {
        gcd(a - b, b)
    } else {
        gcd(a, b - a)
    }
}

fn abs_<T: Num + Ord + Copy>(x: T) -> T {
    if x < T::zero() {
        x - (x + x)
    } else {
        x
    }
}

impl<T: Num + Ord + Copy> Ratio<T> {
    pub fn new(num: T, den: T) -> Ratio<T> {
        let mut res = Self {
            numerator: num,
            denominator: den,
        };
        res.reduce();
        res
    }

    pub fn reduce(&mut self) {
        let div: T = gcd(abs_(self.numerator), abs_(self.denominator));
        self.numerator = self.numerator / div;
        self.denominator = self.denominator / div;
    }
}

impl<T: Num + Ord + Copy> Add for Ratio<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = Ratio {
            numerator: self.numerator * other.denominator + other.numerator * self.denominator,
            denominator: self.denominator * other.denominator,
        };
        res.reduce();
        res
    }
}

impl<T: Num + Ord + Copy> Sub for Ratio<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut res = Ratio {
            numerator: self.numerator * other.denominator - other.numerator * self.denominator,
            denominator: self.denominator * other.denominator,
        };
        res.reduce();
        res
    }
}

impl<T: Num + Ord + Copy> Mul for Ratio<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut res = Ratio {
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator,
        };
        res.reduce();
        res
    }
}

impl<T: Num + Ord + Copy> Div for Ratio<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut res = Ratio {
            numerator: self.numerator * other.denominator,
            denominator: self.denominator * other.numerator,
        };
        res.reduce();
        res
    }
}

impl<T: Num + Ord + Copy> Ord for Ratio<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.numerator * other.denominator;
        let b = other.numerator * self.denominator;
        T::cmp(&a, &b)
    }
}

impl<T: Num + Ord + Copy> PartialOrd for Ratio<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

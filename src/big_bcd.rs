use std::convert::From;

struct BigBcd {
    digs: Vec<u8>,
}

impl From<String> for BigBcd {
    fn from(s: String) -> Self {
        let mut rv = vec![];
        rv.reserve(s.len());
        for c in s.chars().rev() {
            rv.push(c.to_digit(10).unwrap() as u8);
        }
        BigBcd {
            digs: rv,
        }
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
        BigBcd {
            digs: rv,
        }
    }
}

impl BigBcd {

}

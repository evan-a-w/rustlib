use std::rc::Rc;
use std::fmt;
use std::mem::{replace, MaybeUninit};

pub struct FString {
    rope: Rope
}

impl FString {
    pub fn new(s: String) -> Self {
        Self {
            rope: Rope::new(s),
        }
    }

    pub fn concat(&mut self, other: Self) {
        // We are switching self.rope with an uninitialised value and immediately
        // replacing it, therefore it is safe.
        unsafe {
            let nr = replace(&mut self.rope, MaybeUninit::zeroed().assume_init());
            self.rope = nr.concat(other.rope);
        }
    }

    pub fn nth(&self, i: usize) -> Option<char> {
        self.rope.nth(i)
    }

    pub fn size(&self) -> usize {
        self.rope.size()
    }

    pub fn split(&mut self, i: usize) -> Option<Self> {
        // We are switching self.rope with an uninitialised value and immediately
        // replacing it, therefore it is safe.
        unsafe {
            let nr = replace(&mut self.rope, MaybeUninit::zeroed().assume_init());
            let (r, o) = nr.split(i);
            self.rope = r;
            match o {
                None => None,
                Some(x) => Some(Self {rope: x}),
            }
        }
    }

    pub fn insert(&mut self, i: usize, s: Self) {
        unsafe {
            let nr = replace(&mut self.rope, MaybeUninit::zeroed().assume_init());
            let nr = nr.insert(i, s.rope);
            self.rope = nr;
        }
    }

    pub fn insert_string(&mut self, i: usize, s: String) {
        unsafe {
            let nr = replace(&mut self.rope, MaybeUninit::zeroed().assume_init());
            let nr = nr.insert(i, Rope::new(s));
            self.rope = nr;
        }
    }

    pub fn delete(&mut self, i: usize, j: usize) {
        unsafe {
            let nr = replace(&mut self.rope, MaybeUninit::zeroed().assume_init());
            let nr = nr.delete(i, j);
            self.rope = nr;
        }
    }
}

impl fmt::Display for FString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rope)
    }
}

pub enum Rope {
    Node(RopeNode),
    Leaf(Rc<String>),
}

pub struct RopeNode {
    left:  Box<Rope>,
    right: Box<Rope>,
    size: usize,
    size_left: usize,
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rec_display(f);
        Ok(())
    }
}

impl Rope {
    pub fn new(s: String) -> Self {
        Rope::Leaf(Rc::new(s))
    }
    pub fn nth(&self, i: usize) -> Option<char> {
        match self {
            Rope::Node(rn) => {
                if rn.size_left <= i {
                    if i >= rn.size {
                        None
                    } else {
                        rn.right.nth(i - rn.size_left)
                    }
                } else {
                    rn.left.nth(i)
                }
            }
            Rope::Leaf(s) => s.chars().nth(i),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Rope::Leaf(s) => s.len(),
            Rope::Node(rn) => rn.size,
        }
    }

    // TODO: make efficient for special cases like both leaves and such
    pub fn concat(self, other: Self) -> Self {
        Rope::Node(RopeNode {
            size: Rope::size(&self) + Rope::size(&other),
            size_left: Rope::size(&self),
            left: Box::new(self),
            right: Box::new(other),
        })
    }

    pub fn construct_from_vec(v: Vec<Self>) -> Option<Self> {
        let mut curr = None;
        for i in v.into_iter() {
            match curr {
                None => { curr = Some(i); }
                Some(x) => { curr = Some(x.concat(i)); }
            };
        }
        curr
    }

    // TODO: instead of using a vector i think you can just keep a running
    // thingy.
    fn do_split(self, i: usize, mut v: Vec<Self>) -> (Self, Vec<Self>) {
        match self {
            Rope::Leaf(s) => {
                // In this case, we perform the split on the string itself,
                // then return the new node. We add the result thing to the vec
                // which will later fold with concatenation.
                if i >= s.len() {
                    return (Rope::Leaf(s), v);
                }
                let (a, b) = &s.split_at(i);
                let ns = a.to_string();
                let rs = b.to_string();
                v.push(Rope::Leaf(Rc::new(rs)));
                (Rope::Leaf(Rc::new(ns)), v)
            }
            Rope::Node(mut rn) => {
                let res: Rope;
                // Recursive thing + update.
                if i < rn.size_left {
                    // splitting on the left means that the result we get from
                    // splitting left will be the thing we wish to return.
                    // We need to add the current node (once updated) to the
                    // vector.
                    let (tres, tv) = rn.left.do_split(i, v);
                    res = tres;
                    v = tv;
                    // new rn.left will come from assembling v
                    match v.len() {
                        0 => { v.push(*rn.right); },
                        _ => {
                            rn.left = Box::new(Rope::construct_from_vec(v).unwrap());
                            v = vec![];
                            v.push(Rope::Node(rn));
                        }
                    }
                } else if i == rn.size_left {
                    // even split - we can just return the left and push the right
                    res = *rn.left;
                    v.push(*rn.right);
                } else {
                    // splitting somewhere on the right - we want to update 
                    // our right node and return the new thing.
                    let (tres, tv) = rn.right.do_split(i - rn.size_left, v);
                    v = tv;
                    rn.right = Box::new(tres);
                    if Rope::size(&rn.right) == 0 {
                        res = *rn.left;
                    } else {
                        rn.size = Rope::size(&rn.left) + Rope::size(&rn.right);
                        res = Rope::Node(rn);
                    }
                }
                (res, v)
            }
        } 
    }

    pub fn split(self, i: usize) -> (Self, Option<Self>) {
        let (left, v) = self.do_split(i, vec![]);
        (left, Rope::construct_from_vec(v))
    }

    pub fn rec_display(&self, f: &mut fmt::Formatter<'_>) {
        match self {
            Rope::Node(rn) => {
                rn.left.rec_display(f);
                rn.right.rec_display(f);
            }
            Rope::Leaf(s) => { write!(f, "{}", s); }
        }
    }

    pub fn insert(self, i: usize, s: Self) -> Self {
        let (mut res, r) = self.split(i);
        res = res.concat(s);
        match r {
            None => {},
            Some(x) => {
                res = res.concat(x);
            }
        }
        res
    }

    pub fn delete(self, i: usize, j: usize) -> Self {
        let (l, r) = self.split(i);
        match r {
            None => l,
            Some(x) => match x.split(j + 1).1 {
                None => l,
                Some(p) => l.concat(p),
            }
        }
    }
}

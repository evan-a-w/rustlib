use std::rc::Rc;

pub enum Rope {
    Node(RopeNode),
    Leaf(Rc<String>),
}

pub struct RopeNode {
    left:  Option<Box<Rope>>,
    right: Option<Box<Rope>>,
    size: usize,
    size_left: usize,
}

impl Rope {
    pub fn nth(&self, i: usize) -> Option<char> {
        match self {
            Rope::Node(rn) => {
                if rn.size_left <= i {
                    if i >= rn.size {
                        None
                    } else {
                        rn.right.as_ref().unwrap().nth(i - rn.size_left)
                    }
                } else {
                    rn.left.as_ref().unwrap().nth(i)
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

    pub fn concat(self, other: Self) -> Self {
        Rope::Node(RopeNode {
            size: Rope::size(&self) + Rope::size(&other),
            size_left: Rope::size(&self),
            left: Some(Box::new(self)),
            right: Some(Box::new(other)),
        })
    }

    fn do_split(self, i: usize, v: &mut Vec<Self>) -> Option<Self> {
        match self {
            Rope::Leaf(s) => {
                // In this case, we perform the split on the string itself,
                // then return the new node. We add the result thing to the vec
                // which will later fold with concatenation.
                if i >= s.len() {
                    return None;
                }
                let (a, b) = &s.split_at(i);
                let ns = a.to_string();
                let rs = b.to_string();
                v.push(Rope::Leaf(Rc::new(rs)));
                Some(Rope::Leaf(Rc::new(ns)))
            }
            Rope::Node(rn) => {
                // Recursive thing + update.
                None
            }
        } 
    }

    pub fn split(self, i: usize) -> (Self, Self) {
        match self {

        }
    }
}

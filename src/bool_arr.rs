pub struct BoolArr {
    vec: Vec<usize>,
    size: usize,
}

impl BoolArr {
    pub fn new(init_size: usize, init_val: bool) -> Self {
        let mut sz = init_size / 64;
        if init_size % 64 != 0 {
            sz += 1;
        }
        BoolArr {
            vec: vec![if init_val { !0 } else { 0 }; sz],
            size: init_size,
        }
    }

    pub fn get(&self, i: usize) -> bool {
        (self.vec[i / 64] & (1 << (i % 64))) > 0
    }

    pub fn set(&mut self, i: usize, v: bool) {
        self.vec[i / 64] &= !(1 << (i % 64));
        if v {
            self.vec[i / 64] |= 1 << (i % 64);
        }
    }

    pub fn push(&mut self, v: bool) {
        if self.size % 64 == 0 {
            self.vec.push(0);
        }
        self.size += 1;
        self.set(self.size - 1, v);
    }
}

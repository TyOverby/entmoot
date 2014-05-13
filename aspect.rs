extern crate collections;
use collections::bitv::BitvSet;

struct Aspect {
    bitset: BitvSet
}

impl Aspect {
    pub fn is_subset(&self, other: &Aspect)-> bool{
        self.bitset.is_subset(&other.bitset)
    }
    pub fn with_key(self, key: uint) -> Aspect {
        let mut new = self;
        new.bitset.insert(key);
    }
}

fn main() {}

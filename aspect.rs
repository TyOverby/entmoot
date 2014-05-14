#![allow(dead_code)]
extern crate collections;
use collections::bitv::BitvSet;

#[deriving(Clone)]
pub struct Aspect {
    bitset: BitvSet
}

impl Aspect {
    pub fn new()-> Aspect {
        Aspect {bitset: BitvSet::new()}
    }
    pub fn is_subset(&self, other: &Aspect)-> bool{
        self.bitset.is_subset(&other.bitset)
    }
    pub fn add_key(&mut self, key: uint) {
        self.bitset.insert(key);
    }
    pub fn del_key(&mut self, key: uint) {
        self.bitset.remove(&key);
    }
}


#[test]
fn test_subset() {
    let mut a1 = Aspect::new();
    assert!(a1.is_subset(&a1));

    let mut a2 = Aspect::new();
    a2.add_key(4);
    assert!(a1.is_subset(&a2));

    a1.add_key(4);
    assert!(a1.is_subset(&a2));

    a1.add_key(5);
    assert!(!a1.is_subset(&a2));
    assert!(a2.is_subset(&a1));

    a1.del_key(5);
    assert!(a1.is_subset(&a2));
}

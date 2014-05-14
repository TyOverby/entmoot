#![allow(dead_code)]
extern crate collections;
use collections::bitv::BitvSet;


pub struct Entity {
    id: uint,
    c_offsets: Vec<uint>,
    aspect: BitvSet
}

impl Entity {
    pub fn new(id: uint, prealloc: uint) -> Entity {
        let mut ret = Entity {id: id, c_offsets: Vec::new(), aspect: BitvSet::new()};
        ret.c_offsets.reserve(prealloc);
        ret
    }
    pub fn offsets<'a>(&'a mut self) -> &'a mut Vec<uint>{
        &mut self.c_offsets
    }
    pub fn aspect<'a>(&'a mut self) -> &'a mut BitvSet{
        &mut self.aspect
    }
    pub fn id(&self) -> uint {
        self.id
    }
}

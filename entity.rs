#![allow(dead_code)]
use collections::bitv::BitvSet;


pub struct Entity {
    id: uint,
    c_offsets: Vec<uint>,
    aspect: BitvSet
}

impl Entity {
    pub fn new(id: uint, prealloc: uint) -> Entity {
        Entity {
            id: id,
            c_offsets: Vec::with_capacity(prealloc),
            aspect: BitvSet::new()
        }
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

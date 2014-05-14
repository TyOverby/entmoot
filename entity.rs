#![allow(dead_code)]
use aspect::Aspect;

mod aspect;

pub struct Entity {
    id: uint,
    c_offsets: Vec<uint>,
    pub aspect: Aspect
}

impl Entity {
    pub fn new(id: uint, prealloc: uint) -> Entity {
        let mut ret = Entity {id: id, c_offsets: Vec::new(), aspect: Aspect::new()};
        ret.c_offsets.reserve(prealloc);
        ret
    }
    pub fn offsets<'a>(&'a mut self) -> &'a mut Vec<uint>{
        &mut self.c_offsets
    }
    pub fn aspect<'a>(&'a mut self) -> &'a mut Aspect{
        &mut self.aspect
    }
    pub fn id(&self) -> uint {
        self.id
    }
}

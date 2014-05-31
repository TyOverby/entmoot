#![allow(dead_code)]
use component::Component;
use std::cell::RefCell;
use std::cell::RefMut;

pub struct Bank<C> {
    components: Vec<RefCell<C>>,
    holes: RefCell<Vec<uint>>
}

impl <C: Component> Bank<C> {
    pub fn new() -> Bank<C> {
        Bank { components: Vec::new(), holes: RefCell::new(Vec::new()) }
    }

    pub fn add(&mut self, c: C) -> uint {
        let mut pos;
        let mut holes = self.holes.borrow_mut();
        if !holes.is_empty() {
            pos = holes.pop().unwrap();
            let loc = self.components.get_mut(pos);
            *loc = RefCell::new(c);
        } else {
            pos = self.components.len();
            self.components.push(RefCell::new(c));
        }
        pos
    }

    pub fn del(&self, pos: uint) {
        self.holes.borrow_mut().push(pos);
    }

    pub fn get<'a>(&'a self, pos: uint) -> RefMut<'a, C> {
        self.components.get(pos).borrow_mut()
    }
}

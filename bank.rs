#![allow(dead_code)]
use component::Component;

pub struct Bank<C> {
    components: Vec<C>,
    holes: Vec<uint>
}

impl <C: Component> Bank<C> {
    pub fn new() -> Bank<C> {
        Bank { components: Vec::new(), holes: Vec::new() }
    }

    pub fn add(&mut self, c: C) -> uint {
        let mut pos:uint = 0;
        if !self.holes.is_empty() {
            pos = self.holes.pop().unwrap();
            let loc = self.components.get_mut(pos);
            *loc = c;
        } else {
            pos = self.holes.len();
            self.components.push(c);
        }
        pos
    }

    pub fn del(&mut self, pos: uint) {
        self.holes.push(pos);
    }

    pub fn get<'a>(&'a mut self, pos: uint) -> &'a mut C {
        self.components.get_mut(pos)
    }
}

extern crate collections;
extern crate core;

use collections::hashmap::HashMap;
use std::any::Any;
use component::Component;
use core::intrinsics::get_tydesc;

mod component;

struct World {
    banks: Vec<Box<Any>>
}

impl World {
    fn register_component<C: Component>(&mut self, c: C) {
        let id = C::id();
    }
}

fn main(){}

extern crate collections;
extern crate core;

use collections::hashmap::HashMap;
use std::any::Any;
use core::intrinsics::get_tydesc;

use component::Component;
use component::Sigl;
use bank::Bank;

mod component;
mod bank;

struct World {
    banks: Vec<Box<Any>>
}

impl World {
    fn register_component<C: Component + 'static>(&mut self) {
        let id = Component::id(Sigl::<C>);
        assert!(self.banks.len() == id,
        "Components must be registered in ascending order.");
        let any: Box<Any> = box Bank::<C>::new();
        self.banks.push(any);
    }
}

fn main(){}

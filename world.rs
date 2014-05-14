#![allow(dead_code)]

extern crate collections;
extern crate core;

use std::any::Any;
use std::any::AnyMutRefExt;

use component::Component;
use component::Sigl;
use bank::Bank;
use entity::Entity;

mod component;
mod bank;
mod entity;
mod aspect;

struct World {
    banks: Vec<Box<Any>>,
}

impl World {
    pub fn new() -> World {
        World { banks: Vec::new() }
    }
    pub fn register_component<C: Component + 'static>(&mut self) {
        let id = Component::id(Sigl::<C>);
        assert!(self.banks.len() == id,
        "Components must be registered in ascending order.");
        let any: Box<Any> = box Bank::<C>::new();
        self.banks.push(any);
    }
    fn get_bank<'a, C: Component + 'static>(&'a mut self) -> (uint, Option<&'a mut Bank<C>>) {
        let id = Component::id(Sigl::<C>);
        assert!(id < self.banks.len(), "Component has not been registered with World.");
        let bank_any: &'a mut Box<Any>  = self.banks.get_mut(id);
        (id, bank_any.as_mut::<Bank<C>>())
    }

    pub fn map_component<C: Component + 'static>(&mut self, component: C, entity: &mut Entity) {
        let (component_id, bank_opt) = self.get_bank::<C>();
        let bank = match bank_opt {
            Some(b) => b,
            None => fail!("Internal Error: Found bank of wrong type.")
        };
        let pos = bank.add(component);
        {
            let offsets = entity.offsets();
            let len_off = offsets.len();
            if component_id <= len_off {
                offsets.grow(component_id - len_off + 1, &std::uint::MAX);
            }
            let inject = offsets.get_mut(component_id);
            *inject = pos;
        }
        {
            entity.aspect.add_key(component_id);
        }
    }

    pub fn unmap_component<C: Component + 'static>(&mut self, entity: &mut Entity) {
        let (id, bank_opt) = self.get_bank::<C>();
        let bank = match bank_opt {
            Some(b) => b,
            None => fail!("Internal Error: Found bank of wrong type.")
        };
        bank.del(entity.id());
        entity.aspect().del_key(id);
    }
}

#[cfg(test)]
mod world_tests {
    use super::World;
    use entity::Entity;

    #[test]
    fn test_component_map() {
        let mut world = World::new();
    }
}

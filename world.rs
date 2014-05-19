#![allow(dead_code)]

extern crate collections;
extern crate core;

use std::any::Any;
use std::any::AnyMutRefExt;
use std::any::AnyRefExt;

use component::Component;
use bank::Bank;
use entity::Entity;
use sigl::Sigl;

mod component;
mod bank;
mod entity;
mod sigl;

struct World {
    banks: Vec<Box<Any>>,
    component_count: uint,
    entity_id_counter: uint
}

impl World {
    pub fn new() -> World {
        World { banks: Vec::new(), component_count: 0, entity_id_counter: 0 }
    }
    pub fn spawn(&mut self) -> Entity {
        self.entity_id_counter += 1;
        Entity::new(self.entity_id_counter, self.component_count)
    }
    pub fn register_component<C: Component + 'static>(&mut self) {
        let id = Component::id(Sigl::<C>);
        assert!(self.banks.len() == id,
        "Components must be registered in ascending order with no duplicates.");
        let any: Box<Any> = box Bank::<C>::new();
        self.banks.push(any);
        self.component_count += 1;
    }
    fn get_bank<'a, C: Component + 'static>(&'a mut self) -> (uint, &'a mut Bank<C>) {
        let id = Component::id(Sigl::<C>);
        assert!(id < self.banks.len(), "Component has not been registered with World.");
        let bank_any: &'a mut Box<Any>  = self.banks.get_mut(id);
        let bank = match bank_any.as_mut::<Bank<C>>() {
            Some(e) => e,
            None => fail!("Internal Error: Found bank of wrong type.")
        };
        (id, bank)
    }

    pub fn map_component<C: Component + 'static>(&mut self, component: C, entity: &mut Entity) {
        let (component_id, bank) = self.get_bank::<C>();
        // TODO: check to see if the entity already has this component
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
            entity.aspect().insert(component_id);
        }
    }

    pub fn get_component<'a, C: Component + 'static>(&'a mut self, entity: &mut Entity) -> Option<&'a mut C>{
        let (component_id, bank) = self.get_bank::<C>();
        let offset = entity.offsets().get(component_id);
        if *offset == std::uint::MAX {
            return None;
        } else {
            return Some(bank.get(*offset));
        }

    }

    pub fn unmap_component<C: Component + 'static>(&mut self, entity: &mut Entity) {
        let (id, bank) = self.get_bank::<C>();
        bank.del(entity.id());
        entity.aspect().remove(&id);
        let ptr = entity.offsets().get_mut(id);
        *ptr = std::uint::MAX;
    }
}

#[cfg(test)]
mod world_tests {
    use super::World;
    use entity::Entity;
    use component::Component;
    use sigl::Sigl;

    struct Pos {
        x: uint, y: uint
    }
    impl Component for Pos {
        fn id(_: Sigl<Pos>) -> uint { 0 }
    }

    struct Vel {
        x: uint, y: uint
    }
    impl Component for Vel {
        fn id(_: Sigl<Vel>) -> uint { 1 }
    }

    #[test]
    fn test_component_map() {
        let mut world = World::new();
        world.register_component::<Pos>();
        world.register_component::<Vel>();

        let mut e1 = world.spawn();

        {
            world.map_component(Pos {x: 0, y: 5}, &mut e1);
            let c1 = world.get_component::<Pos>(&mut e1);
            assert!(c1.is_some());
            let c1 = c1.unwrap();
            assert!(c1.x == 0);
            assert!(c1.y == 5);
        } {
            world.map_component(Vel {x: 3, y: 3}, &mut e1);
            let c2 = world.get_component::<Vel>(&mut e1);
            assert!(c2.is_some());
            let c2 = c2.unwrap();
            assert!(c2.x == 3);
            assert!(c2.y == 3);
        }
    }

    #[test]
    fn test_component_remove() {
        let mut world = World::new();
        world.register_component::<Pos>();
        world.register_component::<Vel>();

        let mut e1 = world.spawn();

        {
            world.map_component(Pos {x: 4, y: 4}, &mut e1);
            let c1 = world.get_component::<Pos>(&mut e1);
            assert!(c1.is_some());
            let c1 = c1.unwrap();
            assert!(c1.x == 4);
            assert!(c1.y == 4);
        } {
            world.unmap_component::<Pos>(&mut e1);
            let c1 = world.get_component::<Pos>(&mut e1);
            assert!(c1.is_none());
        }
    }

    #[test]
    fn test_component_update() {
        let mut world = World::new();
        world.register_component::<Pos>();
        world.register_component::<Vel>();

        let mut e1 = world.spawn();

        {
            world.map_component(Pos {x: 4, y: 4}, &mut e1);
            let c1 = world.get_component::<Pos>(&mut e1);
            assert!(c1.is_some());
            let c1 = c1.unwrap();
            assert!(c1.x == 4);
            assert!(c1.y == 4);

            *c1 = Pos {x: 0, y: 0};
            assert!(c1.x == 0);
            assert!(c1.y == 0);
        } {
            let c1 = world.get_component::<Pos>(&mut e1);
            assert!(c1.is_some());
            let c1 = c1.unwrap();
            assert!(c1.x == 0);
            assert!(c1.y == 0);
        }
    }
}

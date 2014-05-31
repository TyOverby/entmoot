#![allow(dead_code)]
#![feature(macro_rules)]

extern crate collections;
extern crate core;

use std::any::Any;
use std::any::AnyRefExt;
use std::any::AnyMutRefExt;
use collections::bitv::BitvSet;
use std::cell::RefCell;
use std::cell::Ref;
use std::cell::RefMut;
use std::rc::Rc;
use std::cast::transmute;

use component::Component;
use bank::Bank;
use sigl::Sigl;
use system::System;

mod component;
mod bank;
mod sigl;
mod system;



macro_rules! aspect(
    () => ({
        use collections::bitv::BitvSet;
        BitvSet::new()
    });
    ($t: ty) => ({
        let mut bitset = aspect!();
        bitset.insert(Component::component_id(Sigl::<$t>));
        bitset
    });
    ($t: ty, $($rest: ty),+) => ({
        let mut bitset = aspect!($($rest),*);
        bitset.insert(Component::component_id(Sigl::<$t>));
        bitset
    });
)

#[cfg(test)]
mod test;

struct World<U> {
    banks: Vec<Box<Any>>,
    component_count: uint,
    entity_id_counter: uint,
    entities: Vec<Rc<Entity>>,
    systems: Vec<RefCell<Box<System<U>>>>
}

impl <U> World<U> {
    pub fn new() -> World<U> {
        World {
            banks: Vec::new(),
            component_count: 0,
            entity_id_counter: 0,
            entities: Vec::new(),
            systems: Vec::new()
        }
    }

    pub fn spawn<'a>(&'a mut self) -> Rc<Entity> {
        self.entity_id_counter += 1;
        let e = Rc::new(Entity::new(self.entity_id_counter, self.component_count));
        self.entities.push(e.clone());
        e
    }

    pub fn register_component<C: Component + 'static>(&mut self) {
        let id = Component::component_id(Sigl::<C>);
        assert!(self.banks.len() == id,
                "Components must be registered in ascending order with no duplicates.");
        let any: Box<Any> = box Bank::<C>::new();
        self.banks.push(any);
        self.component_count += 1;
    }

    pub fn add_system<S: System<U> + 'static>(&mut self, system: S) {
        let sysid = system.system_id();
        assert!(self.systems.len() == sysid,
                "systems must be registered in ascending order with no duplicates.")
        self.systems.push(RefCell::new(box system as Box<System<U>>))
    }

    pub fn update(&mut self, update_item: U) {
        for system in self.systems.iter() {
            system.borrow_mut().update(&update_item, &*self);
        }
    }

    fn get_bank<'a, C: Component + 'static>(&'a self) -> (uint, &'a Bank<C>) {
        let id = Component::component_id(Sigl::<C>);
        assert!(id < self.banks.len(), "Component has not been registered with World.");
        let bank_any: &'a Box<Any>  = self.banks.get(id);
        let bank = match bank_any.as_ref::<Bank<C>>() {
            Some(e) => e,
            None => fail!("Internal Error: Found bank of wrong type.")
        };
        (id, bank)
    }

    fn get_bank_mut<'a, C: Component + 'static>(&'a mut self) -> (uint, &'a mut Bank<C>) {
        let id = Component::component_id(Sigl::<C>);
        assert!(id < self.banks.len(), "Component has not been registered with World.");
        let bank_any: &'a mut Box<Any>  = self.banks.get_mut(id);
        let bank = match bank_any.as_mut::<Bank<C>>() {
            Some(e) => e,
            None => fail!("Internal Error: Found bank of wrong type.")
        };
        (id, bank)
    }

    pub fn map_component<C: Component + 'static>(&mut self, component: C, entity: Rc<Entity>) {
        let mut c_id;
        // TODO: check to see if the entity already has this component
        {
            let (component_id, bank) = self.get_bank_mut::<C>();
            c_id = component_id;
            let pos = bank.add(component);
            let mut offsets = entity.offsets_mut();
            let len_off = offsets.len();
            if component_id <= len_off {
                offsets.grow(component_id - len_off + 1, &std::uint::MAX);
            }
            let inject = offsets.get_mut(component_id);
            *inject = pos;
        } {
            entity.aspect_mut().insert(c_id);
        } {
            for system in self.systems.mut_iter() {
                let mut system = system.borrow_mut();
                let sys_id = system.system_id();
                if !entity.in_system(sys_id) && system.possible_add(entity.clone()) {
                    let mut sys = entity.systems_mut();
                    sys.insert(sys_id);
                }
            }
        }
    }

    fn remove_all_components(&mut self, entity: &Entity) {
        let mut aspect = entity.aspect_mut();
        let systems = entity.systems();
        for system in systems.iter() {
            self.systems.get_mut(system).borrow_mut().must_remove(entity);
        }
        aspect.clear();
    }

    pub fn get_component<'a, C: Component + 'static>(&'a self, entity: &Entity) -> Option<RefMut<'a, C>> {
        let (component_id, bank) = self.get_bank::<C>();
        let offsets = entity.offsets();

        let offset = offsets.get(component_id);
        if *offset == std::uint::MAX {
            return None;
        } else {
            return Some(bank.get(*offset));
        }

    }

    pub fn unmap_component<C: Component + 'static>(&mut self, entity: &Entity) {
        {
            let (id, bank) = self.get_bank_mut::<C>();
            let mut aspect = entity.aspect_mut();
            let mut offsets= entity.offsets_mut();

            bank.del(entity.id());
            aspect.remove(&id);
            let ptr = offsets.get_mut(id);
            *ptr = std::uint::MAX;
        } {
            let mut to_remove = Vec::new();
            {
                let systems = entity.systems();
                for system in systems.iter() {
                    if self.systems.get_mut(system).borrow_mut().possible_remove(entity) {
                        to_remove.push(system);
                    }
                }
            }
            let mut sys_mut = entity.systems_mut();
            for idx in to_remove.iter() {
                sys_mut.remove(idx);
            }
        }
    }
}



pub struct Entity {
    id: uint,
    c_offsets: RefCell<Vec<uint>>,
    aspect: RefCell<BitvSet>,
    systems: RefCell<BitvSet>
}

impl Entity {
    pub fn new(id: uint, prealloc: uint) -> Entity {
        Entity {
            id: id,
            c_offsets: RefCell::new(Vec::with_capacity(prealloc)),
            aspect: RefCell::new(BitvSet::new()),
            systems: RefCell::new(BitvSet::new())
        }
    }

    fn offsets<'a>(&'a self) -> Ref<'a, Vec<uint>> {
        self.c_offsets.borrow()
    }
    pub fn aspect<'a>(&'a self) -> Ref<'a, BitvSet> {
        self.aspect.borrow()
    }
    fn systems<'a>(&'a self) -> Ref<'a, BitvSet> {
        self.systems.borrow()
    }

    fn offsets_mut<'a>(&'a self) -> RefMut<'a, Vec<uint>>{
        self.c_offsets.borrow_mut()
    }
    fn aspect_mut<'a>(&'a self) -> RefMut<'a, BitvSet> {
        self.aspect.borrow_mut()
    }
    fn systems_mut<'a>(&'a self) -> RefMut<'a, BitvSet> {
        self.systems.borrow_mut()
    }

    fn has_component(&self, id: uint) -> bool {
        self.aspect.borrow().contains(&id)
    }

    fn in_system(&self, id: uint) -> bool {
        self.systems.borrow().contains(&id)
    }

    pub fn id(&self) -> uint {
        self.id
    }
}


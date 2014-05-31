use collections::bitv::BitvSet;
use super::Entity;
use super::World;
use std::rc::Rc;

pub trait System<U> {
    fn system_id(&self)-> uint;
    // Returns true if the entity was added.
    fn possible_add(&mut self, entity: Rc<Entity>)-> bool;
    // Returs true if the entity was removed.
    fn possible_remove(&mut self, entity: &Entity)-> bool;
    fn must_remove(&mut self, entity: &Entity);
    fn update(&mut self, update_item: &U, world: &World<U>);
}

pub struct AspectSystem<U, S> {
    aspect: BitvSet,
    entities: Vec<Rc<Entity>>,
    state: S
}

pub trait AspectSystemState<U> {
    fn system_id(&self)-> uint;
    fn update(&mut self, entity: &Entity, update_item: &U, world: &World<U>);
}

impl <U, S> AspectSystem<U, S> {
    fn try_remove(&mut self, id: uint)-> bool{
        let loc = self.entities.iter().position(|a| a.id() == id);
        if loc.is_some() {
            self.entities.swap_remove(loc.unwrap());
            true
        } else {
            false
        }
    }
}

impl <U, S: AspectSystemState<U>> AspectSystem<U, S> {
    pub fn new(aspect: BitvSet, state: S)-> AspectSystem<U, S> {
        AspectSystem {
            aspect: BitvSet::new(),
            entities: Vec::new(),
            state: state
        }
    }
}

impl <U, S: AspectSystemState<U>> System<U> for AspectSystem<U, S> {
    fn system_id(&self) -> uint {
        self.state.system_id()
    }

    // Returns true if the entity was added
    fn possible_add(&mut self, entity: Rc<Entity>)-> bool {
        if entity.aspect().is_superset(&self.aspect) {
            self.entities.push(entity);
            true
        } else {
            false
        }
    }


    // Returs true if the entity was removed.
    fn possible_remove(&mut self, entity: &Entity)-> bool{
        if entity.aspect().is_superset(&self.aspect) {
            return false
        }
        self.try_remove(entity.id())
    }
    fn must_remove(&mut self, entity: &Entity) {
        self.try_remove(entity.id());
    }
    fn update(&mut self, update_item: &U, world: &World<U>) {
        for e in self.entities.iter() {
            self.state.update(&**e, update_item, world);
        }
    }
}

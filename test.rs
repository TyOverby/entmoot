use component::Component;
use sigl::Sigl;
use super::World;
use system::AspectSystemState;
use system::AspectSystem;
use system::System;
use super::Entity;


////
//// Contained Test
////
struct Something {
    set: bool
}
impl Component for Something {
    fn component_id(_: Sigl<Something>) -> uint { 0 }
}
struct ContainsSystem {
    update_count: uint
}
impl AspectSystemState<bool> for ContainsSystem {
    fn system_id(&self) -> uint {0}
    fn update(&mut self, e:&Entity, b: &bool, world: &World<bool>) {
        assert!(b);
        self.update_count += 1;
        assert!(self.update_count == 1);

        let mut comp = world.get_component::<Something>(e).unwrap();
        comp.set = true;
    }
}

#[test]
fn test_off_on() {
    let mut world: World<bool> = World::new();
    world.register_component::<Something>();
    world.add_system(AspectSystem::new(aspect!(Something), ContainsSystem {update_count: 0}));
    let e = world.spawn();
    world.update(false);
    world.map_component(Something {set: false}, e.clone());
    world.update(true);
    assert!(world.get_component::<Something>(&*e).unwrap().set);
    world.unmap_component::<Something>(&*e);
    world.update(false);
}

////
//// Movement Tests
////

struct Pos {
    x: uint, y: uint
}
impl Component for Pos {
    fn component_id(_: Sigl<Pos>) -> uint { 0 }
}

struct Vel {
    x: uint, y: uint
}
impl Component for Vel {
    fn component_id(_: Sigl<Vel>) -> uint { 1 }
}

struct MovementSystem;
impl AspectSystemState<uint> for MovementSystem {
    fn system_id(&self)-> uint {0}
    fn update(&mut self, entity: &Entity, t:&uint,  world: &World<uint>) {
        let mut pos = world.get_component::<Pos>(entity).unwrap();
        let vel = world.get_component::<Vel>(entity).unwrap();
        assert!(pos.x == (entity.id() - 1) * (t + 1));
        pos.x += vel.x;
        pos.y += vel.y;
    }
}

#[test]
fn test_basic_system() {
    let mut world: World<uint> = World::new();
    world.register_component::<Pos>();
    world.register_component::<Vel>();
    world.add_system(AspectSystem::new(aspect!(Pos, Vel), MovementSystem));

    for i in range(0u, 5) {
        let e = world.spawn();
        world.map_component(Pos{x: i, y:i}, e.clone());
        world.map_component(Vel{x: i, y:i}, e.clone());
    }

    for t in range(0u, 10) {
        world.update(t);
    }
}

#[test]
fn test_component_map() {
    let mut world: World<uint> = World::new();
    world.register_component::<Pos>();
    world.register_component::<Vel>();

    let e1 = world.spawn();

    {
        world.map_component(Pos {x: 0, y: 5}, e1.clone());
        let c1 = world.get_component::<Pos>(&*e1);
        assert!(c1.is_some());
        let c1 = c1.unwrap();
        assert!(c1.x == 0);
        assert!(c1.y == 5);
    } {
        world.map_component(Vel {x: 3, y: 3}, e1.clone());
        let c2 = world.get_component::<Vel>(&*e1);
        assert!(c2.is_some());
        let c2 = c2.unwrap();
        assert!(c2.x == 3);
        assert!(c2.y == 3);
    }
}

#[test]
fn test_component_remove() {
    let mut world: World<uint> = World::new();
    world.register_component::<Pos>();
    world.register_component::<Vel>();

    let e1 = world.spawn();

    {
        world.map_component(Pos {x: 4, y: 4}, e1.clone());
        let c1 = world.get_component::<Pos>(&*e1);
        assert!(c1.is_some());
        let c1 = c1.unwrap();
        assert!(c1.x == 4);
        assert!(c1.y == 4);
    } {
        world.unmap_component::<Pos>(&*e1);
        let c1 = world.get_component::<Pos>(&*e1);
        assert!(c1.is_none());
    }
}

#[test]
fn test_component_update() {
    let mut world: World<uint> = World::new();
    world.register_component::<Pos>();
    world.register_component::<Vel>();

    let e1 = world.spawn();

    {
        world.map_component(Pos {x: 4, y: 4}, e1.clone());
        let c1 = world.get_component::<Pos>(&*e1);
        assert!(c1.is_some());
        let mut c1 = c1.unwrap();
        assert!(c1.x == 4);
        assert!(c1.y == 4);

        *c1 = Pos {x: 0, y: 0};
        assert!(c1.x == 0);
        assert!(c1.y == 0);
    } {
        let c1 = world.get_component::<Pos>(&*e1);
        assert!(c1.is_some());
        let c1 = c1.unwrap();
        assert!(c1.x == 0);
        assert!(c1.y == 0);
    }
}

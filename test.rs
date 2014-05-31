#[feature(phase)]

use component::Component;
use sigl::Sigl;
use super::World;
use system::AspectSystemState;
use system::AspectSystem;
use system::System;
use super::Entity;

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
impl AspectSystemState<()> for MovementSystem {
    fn system_id(&self)-> uint {0}
    fn update(&mut self, entity: &Entity, _:&(),  world: &World<()>) {
        let mut pos = world.get_component::<Pos>(entity).unwrap();
        let mut vel = world.get_component::<Vel>(entity).unwrap();
        pos.x += vel.x;
        pos.y += vel.y;
        println!("{}: ({}, {})", entity.id(), pos.x, pos.y);
    }
}

#[test]
fn test_basic_system() {
    let mut world: World<()> = World::new();
    world.register_component::<Pos>();
    world.register_component::<Vel>();
    world.add_system(AspectSystem::new(aspect!(Pos, Vel), MovementSystem));

    for i in range(0u, 5) {
        let e = world.spawn();
        world.map_component::<Pos>(Pos{x: i, y:i}, e.clone());
        world.map_component::<Vel>(Vel{x: i, y:i}, e.clone());
    }

    for _ in range(0, 10) {
        world.update(());
    }

    assert!(false);
}

#[test]
fn test_component_map() {
    let mut world: World<uint> = World::new();
    world.register_component::<Pos>();
    world.register_component::<Vel>();

    let e1 = world.spawn();

    {
        world.map_component(Pos {x: 0, y: 5}, e1.clone());
        let c1 = world.get_component::<Pos>(e1.deref());
        assert!(c1.is_some());
        let c1 = c1.unwrap();
        assert!(c1.x == 0);
        assert!(c1.y == 5);
    } {
        world.map_component(Vel {x: 3, y: 3}, e1.clone());
        let c2 = world.get_component::<Vel>(e1.deref());
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
        let c1 = world.get_component::<Pos>(e1.deref());
        assert!(c1.is_some());
        let c1 = c1.unwrap();
        assert!(c1.x == 4);
        assert!(c1.y == 4);
    } {
        world.unmap_component::<Pos>(e1.deref());
        let c1 = world.get_component::<Pos>(e1.deref());
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
        let mut c1 = world.get_component::<Pos>(e1.deref());
        assert!(c1.is_some());
        let mut c1 = c1.unwrap();
        assert!(c1.x == 4);
        assert!(c1.y == 4);

        *c1 = Pos {x: 0, y: 0};
        assert!(c1.x == 0);
        assert!(c1.y == 0);
    } {
        let c1 = world.get_component::<Pos>(e1.deref());
        assert!(c1.is_some());
        let c1 = c1.unwrap();
        assert!(c1.x == 0);
        assert!(c1.y == 0);
    }
}

use std::any::TypeId;
use wuestite_ecs::{Component, System, World, SparseSet};

#[derive(Component)]
struct Red;

#[derive(Component)]
struct Blue;

#[derive(System)]
struct CountRedComponents;

impl CountRedComponents {
    fn run(&self, world: &mut World) {
        let count = world.components
            .get(&TypeId::of::<Red>())
            .and_then(|any| any.downcast_ref::<SparseSet<Red>>())
            .map(|sparse_set| sparse_set.len())
            .unwrap_or(0);
            
        println!("Red components: {}", count);
    }
}

fn main() {
    let mut world = World::new();

    let e1 = world.spawn();
    world.add_component(e1, Red);

    let e2 = world.spawn();
    world.add_component(e2, Blue);

    let e3 = world.spawn();
    world.add_component(e3, Red);

    CountRedComponents.run(&mut world);
}
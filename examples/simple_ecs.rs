use hematite_ecs::World;
use hematite_ecs::Entity;
use hematite_ecs::Component;
use hematite_ecs::System;

#[derive(Component)]
struct Red;

#[derive(Component)]
struct Blue;

#[derive(System)]
struct CountComponentRed;

impl System for CountComponentRed {
    fn run(&self, world: &World) {
        let mut count = 0;
        for entity in &world.entities {
            if world.get_component::<Red>(*entity).is_some() {
                count += 1;
            }
        }
        println!("Entities with Red component: {}", count);
    }
}

#[derive(Entity)]
struct MyEntity(usize);

fn main() {
    let mut world = World::new();

    let entity1 = world.spawn();
    world.add_component(entity1, Red);

    let entity2 = world.spawn();
    world.add_component(entity2, Blue);

    let entity3 = world.spawn();
    world.add_component(entity3, Red);

    let system = CountComponentRed;
    system.run(&world);
}

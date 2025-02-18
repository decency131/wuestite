use hematite::ecs::world::World;
use hematite::ecs::system::System;
use hematite::ecs::component::Component;

#[derive(Component)]
struct Red;

#[derive(Component)]
struct Blue;

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
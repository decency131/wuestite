use hematite_ecs::{World, Component, System};

#[derive(Component)]
struct Red;

#[derive(Component)]
struct Blue;

struct CountRedComponents;

impl System for CountRedComponents {
    fn run(&self, world: &World) {
        let count = world.entities.iter()
            .filter(|&&e| world.get_component::<Red>(e).is_some())
            .count();
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
    
    CountRedComponents.run(&world);
}
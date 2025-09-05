use wuestite::prelude::*;

#[derive(Debug)]
struct Balance {
    money: u32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Info {
    name: String,
}

impl Component for Balance {}
impl Component for Info {}

fn main() {
    let mut world = World::new();

    let e0 = world.spawn();
    world.add_component(
        e0,
        Info {
            name: "Alice".to_string(),
        },
    );

    let e1 = world.spawn();
    world.add_component(e1, Balance { money: 100 });

    let e2 = world.spawn();
    world.add_component(
        e2,
        Info {
            name: "Bob".to_string(),
        },
    );
    world.add_component(e2, Balance { money: 500 });

    let balance1 = world.get_component_mut::<Balance>(e1).unwrap();
    balance1.money += 50;

    world.remove_component::<Info>(e2);

    world.despawn(e0);
    world.despawn(e1);
}

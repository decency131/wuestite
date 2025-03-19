use hematite_ecs::{Component, Entity, Event, System, World};

#[derive(Component)]
struct Bomb {
    name: String,
    explosion_power: u32,
}

#[derive(Component)]
struct ForceField {
    name: String,
    strength: u32,
}

#[derive(Event)]
struct ExplosionEvent {
    source: u64,
    power: u32,
}

struct BombSystem;

impl System for BombSystem {
    fn run(&self, world: &mut World) {
        let entities: Vec<Entity> = world.entities.clone();

        for entity in entities {
            if let Some(bomb) = world.get_component::<Bomb>(entity) {
                println!(
                    "Bomb '{}' exploded with power {}!",
                    bomb.name, bomb.explosion_power
                );

                world.send_event(ExplosionEvent {
                    source: entity.id(),
                    power: bomb.explosion_power,
                });

                world.despawn(entity);
            }
        }
    }
}

struct ForceFieldSystem;

impl System for ForceFieldSystem {
    fn run(&self, world: &mut World) {
        let events: Vec<ExplosionEvent> = world.get_events();

        for event in events {
            let entities: Vec<Entity> = world.entities.clone();

            for entity in entities {
                if let Some(force_field) = world.get_component_mut::<ForceField>(entity) {
                    if force_field.strength > event.power {
                        force_field.strength -= event.power;
                        println!(
                            "Force field '{}' absorbed explosion from bomb({})! Remaining strength: {}",
                            force_field.name,event.source, force_field.strength
                        );
                    } else {
                        force_field.strength = 0;
                        println!("Force field '{}' destroyed!", force_field.name);
                        world.despawn(entity);
                    }
                }
            }
        }
    }
}

fn main() {
    let mut world = World::new();

    let force_field_entity = world.spawn();
    world.add_component(
        force_field_entity,
        ForceField {
            name: "Shield Alpha".to_string(),
            strength: 75,
        },
    );

    let bomb_entity = world.spawn();
    world.add_component(
        bomb_entity,
        Bomb {
            name: "Big Boom".to_string(),
            explosion_power: 50,
        },
    );

    let bomb_system = BombSystem;
    let force_field_system = ForceFieldSystem;

    bomb_system.run(&mut world);
    force_field_system.run(&mut world);
    force_field_system.run(&mut world);
}

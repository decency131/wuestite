use std::any::TypeId;
use wuestite_ecs::{Component, Entity, Event, SparseSet, System, World};

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
    source: Entity,
    power: u32,
}

#[derive(System)]
struct BombSystem;

impl BombSystem {
    fn run(&self, world: &mut World) {
        if let Some(bombs) = world.components.get(&TypeId::of::<Bomb>()) {
            if let Some(sparse_set) = bombs.downcast_ref::<SparseSet<Bomb>>() {
                // Временное решение для итерации по сущностям с бомбами
                let mut entities_with_bombs = Vec::new();
                for entity_id in 0..world.next_entity_id() {
                    let entity = Entity::new(entity_id);
                    if world.get_component::<Bomb>(entity).is_some() {
                        entities_with_bombs.push(entity);
                    }
                }

                for entity in entities_with_bombs {
                    if let Some(bomb) = world.get_component::<Bomb>(entity) {
                        println!(
                            "Bomb '{}' exploded with power {}!",
                            bomb.name, bomb.explosion_power
                        );
                        world.send_event(ExplosionEvent {
                            source: entity,
                            power: bomb.explosion_power,
                        });
                        world.despawn(entity);
                    }
                }
            }
        }
    }
}

#[derive(System)]
struct ForceFieldSystem;

impl ForceFieldSystem {
    fn run(&self, world: &mut World) {
        let events = world.get_events::<ExplosionEvent>();
        for event in events {
            // Временное решение для итерации по сущностям с силовыми полями
            let mut entities_with_fields = Vec::new();
            for entity_id in 0..world.next_entity_id() {
                let entity = Entity::new(entity_id);
                if world.get_component::<ForceField>(entity).is_some() {
                    entities_with_fields.push(entity);
                }
            }

            for entity in entities_with_fields {
                if let Some(force_field) = world.get_component_mut::<ForceField>(entity) {
                    if force_field.strength > event.power {
                        force_field.strength -= event.power;
                        println!(
                            "Force field '{}' absorbed explosion! Remaining strength: {}",
                            force_field.name, force_field.strength
                        );
                    } else {
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

    let force_field = world.spawn();
    world.add_component(
        force_field,
        ForceField {
            name: "Shield Alpha".to_string(),
            strength: 75,
        },
    );

    let bomb = world.spawn();
    world.add_component(
        bomb,
        Bomb {
            name: "Big Boom".to_string(),
            explosion_power: 50,
        },
    );

    let bomb_system = BombSystem;
    let force_field_system = ForceFieldSystem;

    bomb_system.run(&mut world);
    force_field_system.run(&mut world);
}

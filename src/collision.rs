use bevy::{prelude::*, transform::commands, utils::HashMap};

use crate::{
    asset_loader::GameAssets, laser::Laser, meteor::Meteor, schedule::InGameSet, ship::Ship,
};

#[derive(Component)]
pub struct Health {
    pub value: f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Component)]
pub struct Damage {
    pub value: f32,
}

impl Damage {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

#[derive(Bundle)]
pub struct CollisionBundle {
    pub collider: Collider,
    pub health: Health,
    pub damage: Damage,
}

#[derive(Event)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collided_entity: Entity,
}

impl CollisionEvent {
    fn new(entity: Entity, collided_entity: Entity) -> Self {
        Self {
            entity,
            collided_entity,
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detection.in_set(InGameSet::CollisionDetection),
        )
        .add_systems(
            Update,
            (
                (
                    handle_collisions::<Meteor>,
                    handle_collisions::<Ship>,
                    handle_collisions::<Laser>,
                ),
                apply_collision_damage,
            )
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .add_event::<CollisionEvent>();
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a
                    .translation()
                    .distance(transform_b.translation());
                let radius = collider_a.radius + collider_b.radius;

                if distance < radius {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }

    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();

        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}

fn handle_collisions<T: Component>(
    mut collision_event_writer: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            if query.get(collided_entity).is_ok() {
                continue;
            }
            collision_event_writer.send(CollisionEvent::new(entity, collided_entity));
        }
    }
}

pub fn apply_collision_damage(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut health_query: Query<&mut Health>,
    damage_query: Query<&Damage>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    for &CollisionEvent {
        entity,
        collided_entity,
    } in collision_event_reader.read()
    {
        let Ok(mut health) = health_query.get_mut(entity) else {
            continue;
        };
        let Ok(collision_damage) = damage_query.get(collided_entity) else {
            continue;
        };

        health.value -= collision_damage.value;

        commands.spawn(AudioBundle {
            source: game_assets.collision_audio.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

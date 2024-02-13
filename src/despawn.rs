use bevy::prelude::*;

use crate::{collision::Health, schedule::InGameSet, state::GameState};

#[derive(Event)]
pub struct ScoreEvent;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_dead, despawn_far_entities).in_set(InGameSet::DespawnEntities),
        )
        .add_systems(
            OnEnter(GameState::GameOver),
            despawn_all.in_set(InGameSet::DespawnEntities),
        )
        .add_event::<ScoreEvent>();
    }
}

fn despawn_all(mut commands: Commands, query: Query<Entity, With<Sprite>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_dead(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
    mut score_event_writer: EventWriter<ScoreEvent>,
) {
    for (entity, health) in query.iter() {
        if health.value <= 0.0 {
            commands.entity(entity).despawn_recursive();
            score_event_writer.send(ScoreEvent);
        }
    }
}

fn despawn_far_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        if distance > 1000.0 {
            commands.entity(entity).despawn();
        }
    }
}

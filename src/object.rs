use bevy::prelude::*;

use crate::{collision::CollisionBundle, schedule::InGameSet};

#[derive(Component)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component)]
pub struct AngularSpeed {
    pub value: f32,
}

impl AngularSpeed {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct ObjectBundle {
    pub velocity: Velocity,
    pub angular_speed: AngularSpeed,
    pub sprite: SpriteBundle,
    pub collision: CollisionBundle,
}

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_rotation, update_position)
                .chain()
                .in_set(InGameSet::EntityUpdates),
        );
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}

fn update_rotation(mut query: Query<(&AngularSpeed, &mut Transform)>, time: Res<Time>) {
    for (angular_speed, mut transform) in query.iter_mut() {
        transform.rotate_z(angular_speed.value * time.delta_seconds());
    }
}

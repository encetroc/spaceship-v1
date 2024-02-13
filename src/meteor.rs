use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

const SPAWN_RANGE_X: Range<f32> = -600.0..600.0;
const SPAWN_RANGE_Y: Range<f32> = -320.0..320.0;
const VELOCITY_SCALAR: f32 = 50.0;
const ANGULAR_SPEED_RANGE: Range<f32> = -5.0..5.0;

const SPAWN_X_LEFT: (f32, Range<f32>) = (-700.0, 0.1..1.0);
const SPAWN_X_RIGHT: (f32, Range<f32>) = (700.0, -1.0..-0.1);
const METEOR_HEALTH: f32 = 3.0;
const METEOR_DAMAGE: f32 = 1.0;
pub const METEOR_RADIUS: f32 = 40.0;

use crate::{
    asset_loader::GameAssets,
    collision::{Collider, CollisionBundle, Damage, Health},
    object::{AngularSpeed, ObjectBundle, Velocity},
    schedule::InGameSet,
};

#[derive(Component)]
pub struct Meteor;

#[derive(Debug, Resource)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct MeteorPlugin;

impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_meteor.in_set(InGameSet::EntityUpdates));
    }
}

fn spawn_meteor(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
) {
    spawn_timer.timer.tick(time.delta());

    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let spawn: (f32, Range<f32>);

    if rng.gen_range(0.0..1.0) < 0.5 {
        spawn = SPAWN_X_LEFT;
    } else {
        spawn = SPAWN_X_RIGHT;
    }

    let translation = Vec3::new(spawn.0, rng.gen_range(SPAWN_RANGE_Y), 0.);

    let velocity = Vec3::new(rng.gen_range(spawn.1), 0.0, 0.0) * VELOCITY_SCALAR;

    commands.spawn((
        ObjectBundle {
            velocity: Velocity::new(velocity),
            angular_speed: AngularSpeed::new(rng.gen_range(ANGULAR_SPEED_RANGE)),
            sprite: SpriteBundle {
                transform: Transform::from_translation(translation),
                texture: game_assets.meteor.clone(),
                ..default()
            },
            collision: CollisionBundle {
                health: Health::new(METEOR_HEALTH),
                damage: Damage::new(METEOR_DAMAGE),
                collider: Collider::new(METEOR_RADIUS),
            },
        },
        Meteor,
    ));
}

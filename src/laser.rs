use bevy::prelude::*;

use crate::{
    asset_loader::GameAssets,
    collision::{Collider, CollisionBundle, Damage, Health},
    object::{AngularSpeed, ObjectBundle, Velocity},
    schedule::InGameSet,
    ship::Ship,
};

const LASER_HEALTH: f32 = 1.0;
const LASER_DAMAGE: f32 = 1.0;
pub const LASER_RADIUS: f32 = 10.0;
const LASER_SPEED: f32 = 1000.0;

#[derive(Component)]
pub struct Laser;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fire.in_set(InGameSet::EntityUpdates));
    }
}

fn fire(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    ship_query: Query<&Transform, With<Ship>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let Ok(ship_transform) = ship_query.get_single() else {
        return;
    };

    let movement_direction = ship_transform.rotation * Vec3::Y;
    let movement_distance = 50.0;
    let translation_delta = movement_direction * movement_distance;

    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn(AudioBundle {
            source: game_assets.laser_audio.clone(),
            settings: PlaybackSettings::DESPAWN,
        });

        commands.spawn((
            ObjectBundle {
                velocity: Velocity::new(ship_transform.rotation * Vec3::Y * LASER_SPEED),
                angular_speed: AngularSpeed::new(0.0),
                sprite: SpriteBundle {
                    texture: game_assets.laser.clone(),
                    transform: Transform {
                        rotation: ship_transform.rotation,
                        translation: ship_transform.translation + translation_delta,
                        ..Default::default()
                    },
                    ..default()
                },
                collision: CollisionBundle {
                    health: Health::new(LASER_HEALTH),
                    damage: Damage::new(LASER_DAMAGE),
                    collider: Collider::new(LASER_RADIUS),
                },
            },
            Laser,
        ));
    }
}

use bevy::prelude::*;

use crate::{
    asset_loader::GameAssets,
    collision::{Collider, CollisionBundle, Damage, Health},
    object::{AngularSpeed, ObjectBundle, Velocity},
    schedule::InGameSet,
    state::GameState,
};

const SHIP_ROTATION_SPEED: f32 = 5.0;
const SHIP_SPEED: f32 = 500.0;
pub const SHIP_RADIUS: f32 = 30.0;
const SHIP_HEALTH: f32 = 3.0;
const SHIP_DAMAGE: f32 = 3.0;

#[derive(Component)]
pub struct Ship;

#[derive(Component)]
pub struct Life;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            (spawn_ship).in_set(InGameSet::EntityUpdates),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            (spawn_life).in_set(InGameSet::Score),
        )
        .add_systems(Update, update_life.in_set(InGameSet::EntityUpdates))
        .add_systems(Update, ship_control.in_set(InGameSet::UserInnput))
        .add_systems(Update, ship_destroyed.in_set(InGameSet::EntityUpdates));
    }
}

fn spawn_ship(mut commands: Commands, game_assets: Res<GameAssets>, query: Query<&Ship>) {
    if query.get_single().is_ok() {
        return;
    }

    commands.spawn((
        ObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            angular_speed: AngularSpeed::new(0.0),
            sprite: SpriteBundle {
                texture: game_assets.ship.clone(),
                ..default()
            },
            collision: CollisionBundle {
                health: Health::new(SHIP_HEALTH),
                damage: Damage::new(SHIP_DAMAGE),
                collider: Collider::new(SHIP_RADIUS),
            },
        },
        Ship,
    ));
}

fn ship_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Transform, &mut Velocity, &mut AngularSpeed), With<Ship>>,
) {
    let Ok((transform, mut velocity, mut angular_speed)) = query.get_single_mut() else {
        return;
    };

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 0.5;
    }

    movement_factor += 0.5;

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    angular_speed.value = rotation_factor * SHIP_ROTATION_SPEED;

    // get the ship's forward vector by applying the current rotation to the ships initial facing
    // vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta
    // time
    let movement_distance = movement_factor * SHIP_SPEED;
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    velocity.value = translation_delta;
}

fn ship_destroyed(mut next_state: ResMut<NextState<GameState>>, query: Query<(), With<Ship>>) {
    if query.get_single().is_err() {
        next_state.set(GameState::GameOver);
    }
}

fn spawn_life(mut commands: Commands, game_assets: Res<GameAssets>) {
    for i in 0..(SHIP_HEALTH as usize) {
        commands.spawn((
            SpriteBundle {
                texture: game_assets.life.clone(),
                transform: Transform {
                    translation: Vec3::new(-600.0 + ((i as f32) * 40.0), 340.0, 0.0),
                    ..Default::default()
                },
                ..default()
            },
            Life,
        ));
    }
}

fn update_life(
    mut commands: Commands,
    ship_health_query: Query<&Health, With<Ship>>,
    life_query: Query<(Entity, &Life)>,
) {
    let Ok(Health { value }) = ship_health_query.get_single() else {
        return;
    };

    let life = life_query.iter().len();

    let life_diff = life - (*value as usize);

    if life_diff == 0 {
        return;
    }

    for (entity, _) in life_query.iter() {
        commands.entity(entity).despawn_recursive();
        return;
    }
}

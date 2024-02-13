use bevy::prelude::*;

use crate::{
    laser::{Laser, LASER_RADIUS},
    meteor::{Meteor, METEOR_RADIUS},
    ship::{Ship, SHIP_RADIUS},
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (draw_asterioid_shape, draw_player_shape, draw_laser_shape),
        );
    }
}

fn draw_asterioid_shape(mut gizmos: Gizmos, asteroid_query: Query<&Transform, With<Meteor>>) {
    for transform in asteroid_query.iter() {
        gizmos.circle_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            METEOR_RADIUS,
            Color::GREEN,
        );
    }
}

fn draw_laser_shape(mut gizmos: Gizmos, laser_query: Query<&Transform, With<Laser>>) {
    for transform in laser_query.iter() {
        gizmos.circle_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            LASER_RADIUS,
            Color::RED,
        );
    }
}

fn draw_player_shape(mut gizmos: Gizmos, player_query: Query<&Transform, With<Ship>>) {
    for transform in player_query.iter() {
        gizmos.circle_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            SHIP_RADIUS,
            Color::BLUE,
        );
    }
}

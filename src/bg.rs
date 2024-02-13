use bevy::prelude::*;

use crate::{asset_loader::GameAssets, schedule::InGameSet, state::GameState};

const BG_SIZE: f32 = 256.0;

pub struct BgPlugin;

#[derive(Component)]
struct Bg;

impl Plugin for BgPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_bg);
    }
}

fn spawn_bg(mut commands: Commands, game_assets: Res<GameAssets>, query: Query<&Bg>) {
    for x in 0..6 {
        for y in 0..5 {
            commands.spawn((
                SpriteBundle {
                    texture: game_assets.bg.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            -600.0 + BG_SIZE * (x as f32),
                            -340.0 + BG_SIZE * (y as f32),
                            0.0,
                        ),
                        ..Default::default()
                    },
                    ..default()
                },
                Bg,
            ));
        }
    }
}

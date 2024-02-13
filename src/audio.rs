use bevy::prelude::*;

use crate::{asset_loader::GameAssets, schedule::InGameSet, state::GameState};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            play_sound.in_set(InGameSet::EntityUpdates),
        );
    }
}

fn play_sound(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn(AudioBundle {
        source: game_assets.laser_audio.clone(),
        settings: PlaybackSettings::ONCE,
    });
}

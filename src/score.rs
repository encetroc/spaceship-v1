use bevy::prelude::*;

use crate::{asset_loader::GameAssets, despawn::ScoreEvent, schedule::InGameSet, state::GameState};

#[derive(Component)]
pub struct Score {
    pub value: u32,
}

impl Score {
    fn new(value: u32) -> Self {
        Self { value }
    }
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            init_score.in_set(InGameSet::Score),
        )
        .add_systems(OnEnter(GameState::GameOver), reset_score)
        .add_systems(Update, update_score.in_set(InGameSet::EntityUpdates));
    }
}

fn init_score(mut commands: Commands, query: Query<&Score>) {
    if query.get_single().is_ok() {
        return;
    }
    commands.spawn(Score::new(0));
}

fn reset_score(mut commands: Commands, mut query: Query<&mut Score>) {
    let Ok(mut score) = query.get_single_mut() else {
        return;
    };

    score.value = 0;
}

fn update_score(mut query: Query<&mut Score>, mut score_event_reader: EventReader<ScoreEvent>) {
    let Ok(mut score) = query.get_single_mut() else {
        return;
    };

    for &ScoreEvent in score_event_reader.read() {
        score.value += 1;
    }
}

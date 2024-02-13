use bevy::prelude::*;

use crate::{asset_loader::GameAssets, schedule::InGameSet, score::Score, state::GameState};

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct PauseText;

#[derive(Component)]
pub struct ScoreText;

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), game_over_screen)
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_screen)
            .add_systems(OnEnter(GameState::Paused), pause_text)
            .add_systems(OnExit(GameState::Paused), despawn_pause_text)
            .add_systems(
                OnEnter(GameState::InGame),
                spawn_score_text.in_set(InGameSet::EntityUpdates),
            )
            .add_systems(
                OnEnter(GameState::GameOver),
                despawn_score_text.in_set(InGameSet::DespawnEntities),
            )
            .add_systems(Update, update_score.in_set(InGameSet::EntityUpdates));
    }
}

fn game_over_screen(mut commands: Commands, game_assets: Res<GameAssets>) {
    let text_style = TextStyle {
        font: game_assets.font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("S: start", text_style.clone())
                .with_alignment(TextAlignment::Center),
            ..default()
        },
        GameOverText,
    ));
}

fn despawn_game_over_screen(mut commands: Commands, query: Query<Entity, With<GameOverText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn pause_text(mut commands: Commands, game_assets: Res<GameAssets>) {
    let text_style = TextStyle {
        font: game_assets.font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("pause", text_style.clone())
                .with_alignment(TextAlignment::Center),
            ..default()
        },
        PauseText,
    ));
}

fn despawn_pause_text(mut commands: Commands, query: Query<Entity, With<PauseText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_score_text(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    query_score: Query<&Score>,
    query_score_text: Query<&ScoreText>,
) {
    /* let Ok(score) = query_score.get_single() else {
        return;
    }; */

    if query_score_text.get_single().is_ok() {
        return;
    };

    let text_style = TextStyle {
        font: game_assets.font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(0.to_string(), text_style.clone())
                .with_alignment(TextAlignment::Right),
            transform: Transform {
                translation: Vec3::new(580.0, 340.0, 0.0),
                ..Default::default()
            },
            ..default()
        },
        ScoreText,
    ));
}

fn update_score(
    mut query_score_text: Query<&mut Text, With<ScoreText>>,
    query_score: Query<&Score>,
) {
    let Ok(mut score_text) = query_score_text.get_single_mut() else {
        return;
    };

    let Ok(score) = query_score.get_single() else {
        return;
    };

    score_text.sections[0].value = score.value.to_string();
}

fn despawn_score_text(mut commands: Commands, query: Query<Entity, With<ScoreText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

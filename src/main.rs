mod asset_loader;
mod audio;
mod bg;
mod collision;
mod debug;
mod despawn;
mod laser;
mod meteor;
mod object;
mod schedule;
mod score;
mod ship;
mod state;
mod text;

use asset_loader::{AssetLoaderPlugin, GameAssets};
use audio::AudioPlugin;
use bevy::prelude::*;
use bg::BgPlugin;
use collision::CollisionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use laser::LaserPlugin;
use meteor::MeteorPlugin;
use object::ObjectPlugin;
use schedule::SchedulePlugin;
use score::ScorePlugin;
use ship::ShipPlugin;
use state::{GameState, StatePlugin};
use text::TextPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AssetLoaderPlugin)
        .add_systems(Startup, spawn_camera)
        .add_plugins(ShipPlugin)
        .add_plugins(ObjectPlugin)
        .add_plugins(StatePlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(TextPlugin)
        .add_plugins(MeteorPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(LaserPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(BgPlugin)
        //.add_plugins(AudioPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

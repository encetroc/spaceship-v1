use bevy::prelude::*;

use crate::state::GameState;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    UserInnput,
    EntityUpdates,
    CollisionDetection,
    DespawnEntities,
    Score,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::DespawnEntities,
                InGameSet::UserInnput,
                InGameSet::Score,
                InGameSet::EntityUpdates,
                InGameSet::CollisionDetection,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(InGameSet::DespawnEntities)
                .before(InGameSet::UserInnput),
        );
    }
}

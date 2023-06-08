use bevy::prelude::*;

use crate::states::{application_states::ApplicationState, game_states::SimulationState};

use self::systems::*;

mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(ApplicationState::Game)))
            .add_system(
                player_movement
                    .in_set(OnUpdate(ApplicationState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(ApplicationState::Game)));
    }
}

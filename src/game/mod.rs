use bevy::prelude::*;

use crate::states::application_states::ApplicationState;
use crate::states::game_states::SimulationState;

use self::player::PlayerPlugin;
use self::systems::{pause_simulation, resume_simulation, toggle_simulation};

mod constants;
mod player;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_system(pause_simulation.in_schedule(OnEnter(ApplicationState::Game)))
            .add_system(toggle_simulation.run_if(in_state(ApplicationState::Game)))
            .add_plugin(PlayerPlugin)
            .add_system(resume_simulation.in_schedule(OnExit(ApplicationState::Game)));
    }
}

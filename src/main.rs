use bevy::prelude::*;
use game::GamePlugin;
use states::application_states::ApplicationState;
use systems::*;

mod game;
mod states;

mod systems;

fn main() {
    App::new()
        .add_state::<ApplicationState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(application_is_running)
        .add_system(handle_application_state_change)
        .add_system(exit_game)
        .run();
}

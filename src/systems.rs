use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::states::application_states::ApplicationState;
use crate::states::game_states::SimulationState;

pub fn application_is_running() {
    println!("Application is running.")
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn handle_application_state_change(
    appication_state: Res<State<ApplicationState>>,
    mut next_application_state: ResMut<NextState<ApplicationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if appication_state.0 != ApplicationState::Game {
            next_application_state.set(ApplicationState::Game);
        }
    }

    if keyboard_input.just_pressed(KeyCode::M) {
        if appication_state.0 != ApplicationState::MainMenu {
            next_application_state.set(ApplicationState::MainMenu);
            next_simulation_state.set(SimulationState::Paused);
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

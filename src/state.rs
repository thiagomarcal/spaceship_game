use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    GameOver,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, (game_state_input_events, transition_to_running.run_if(in_state(GameState::GameOver))));
    }
}

fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Running => {
                next_state.set(GameState::Paused)
            }
            GameState::Paused => {
                next_state.set(GameState::Running)
            }
            _ => {}
        }
    }
}

fn transition_to_running(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::Running);
}


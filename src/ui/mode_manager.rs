use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UiState {
    #[default]
    InGame,
    Management,
}

pub struct ModeManagerPlugin;

impl Plugin for ModeManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<UiState>()
            .enable_state_scoped_entities::<UiState>();

        app.add_systems(Update, toggle_mode);
    }
}

pub fn toggle_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<UiState>>,
    mut next_state: ResMut<NextState<UiState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match state.get() {
            UiState::InGame => next_state.set(UiState::Management),
            UiState::Management => next_state.set(UiState::InGame),
        }
    }
}
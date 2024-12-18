use bevy::prelude::*;

mod hotbar;
use hotbar::HotbarPlugin;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HotbarPlugin);
    }
}
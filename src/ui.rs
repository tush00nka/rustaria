use bevy::prelude::*;

mod hotbar;
use hotbar::HotbarPlugin;

pub mod mode_manager;
use mode_manager::ModeManagerPlugin;

mod inventory;
use inventory::InventoryPlugin;

mod crafting;
use crafting::CraftingPlugin;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HotbarPlugin,
            ModeManagerPlugin,
            InventoryPlugin,
            CraftingPlugin
        ));
    }
}
use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Hotbar {
            size: 9,
            selected_slot: 0,
        });

        app.add_systems(Update, select_slot);
    }
}

#[derive(Resource, Default)]
pub struct Hotbar {
    size: usize,
    pub selected_slot: usize,
}

impl Hotbar {
    fn slot_up(&mut self) {
        if self.selected_slot + 1 >= self.size {
            self.selected_slot = 0;
        }
        else {
            self.selected_slot += 1;
        }
    }

    fn slot_down(&mut self) {
        if self.selected_slot > 0 {
            self.selected_slot -= 1;
        }
        else {
            self.selected_slot = self.size-1;
        }
    }
}

fn select_slot(
    mut hotbar: ResMut<Hotbar>,
    mut ev_mouse_wheel: EventReader<MouseWheel>, 
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in ev_mouse_wheel.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if ev.y > 0.0 {
                    hotbar.slot_up();
                }
                else if ev.y < 0.0 {
                    hotbar.slot_down();
                }
            },
            _ => {}
        }
    }
}
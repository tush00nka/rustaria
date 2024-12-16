use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::BLOCK_SIZE_PX;

use super::Player;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_player, jump_player));
    }
}

fn move_player(
    mut player_query: Query<(&mut Velocity, &Player)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let Ok((mut velocity, player)) = player_query.get_single_mut() else { return };

    let direction;

    if keyboard.pressed(KeyCode::KeyA) {
        direction = -1.0;
    }
    else if keyboard.pressed(KeyCode::KeyD) {
        direction = 1.0;
    }
    else {
        direction = 0.0;
    }

    velocity.linvel.x = direction * player.speed * time.delta_secs();
}

fn jump_player(
    mut player_query: Query<(&mut Velocity, &Transform, &Player, &Collider)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    rapier_context_query: Query<&RapierContext>,
) {
    let Ok((mut velocity, transform, player, collider)) = player_query.get_single_mut() else { return };
    let Ok(rapier_context) = rapier_context_query.get_single() else { return };

    if keyboard.just_pressed(KeyCode::Space) {
        let Some((_, hit)) = rapier_context.cast_shape(
            transform.translation.truncate(), 
            0.0, 
            Vec2::NEG_Y, 
            collider, 
            ShapeCastOptions::with_max_time_of_impact(100.0), 
            QueryFilter::exclude_dynamic())
        else { return };

        if hit.time_of_impact <= BLOCK_SIZE_PX*2. {
            velocity.linvel.y = player.jump_force;
        }
    }
}
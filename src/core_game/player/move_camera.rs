use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{Collider, InteractionGroups, QueryFilter, RapierContext},
    rapier::prelude::Group,
};

use crate::core_game::player::player_structs::PlayerGraphics;
use crate::core_game::player::player_structs::Player;
use crate::core_game::player::player_structs::PlayerState;
use crate::core_game::player::player_structs::PlayerMoveState;
use crate::core_game::player::player_structs::CameraVariables;


use crate::core_game::player::player_structs::PlayerStateVariables;


pub fn move_camera (
    mut qcamera: Query<(&mut Transform, &mut CameraVariables), (With <Camera>, Without <Player>, Without <PlayerGraphics>)>,
    qplayer: Query<(&Transform, &PlayerState, &Collider, &PlayerStateVariables), (With <Player>, Without <PlayerGraphics>, Without <Camera>)>,
    rapier_context: Res<RapierContext>,
){
    let scalar_x = 0.05;
    let scalar_y = 0.025;
    let horizontal_offset = 28.0;
    let raycast = 16.0;
    let box_ceilling = 32.0;
    let box_floor = -12.0;



    for (player_transform, state, collider, var) in qplayer.iter() {
        for (mut camera_transform, mut camera_var) in qcamera.iter_mut() {

            

            let player_x = (player_transform.translation.x * 8.0) / 8.0;
            let player_y = (player_transform.translation.y * 8.0) / 8.0;
            let distance = camera_var.new_ground_height - camera_transform.translation.y;
            let velocity = (scalar_y * distance * 8.0).round() / 8.0;

            
            

            let hit_right = rapier_context.cast_shape(
                player_transform.translation.truncate(),
                0.0,
                Vec2::new(raycast, 0.0),
                collider,
                1.0,
                QueryFilter::default()
                .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1))
            );

            let hit_left = rapier_context.cast_shape(
                player_transform.translation.truncate(),
                0.0,
                Vec2::new(-raycast, 0.0),
                collider,
                1.0,
                QueryFilter::default()
                .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1))
            );

            let is_wall_jumping = (state.new.0 == PlayerMoveState::Jump || state.new.0 == PlayerMoveState::Fall) && (hit_left.is_some() || hit_right.is_some()) && var.walljump_counter > 0;



            if var.sprite_flipped == false && is_wall_jumping == false {
                camera_transform.translation.x = camera_transform.translation.x + ((player_x + horizontal_offset) - camera_transform.translation.x) * scalar_x;
                camera_transform.translation.z = player_transform.translation.z;
            }

            if var.sprite_flipped == true && is_wall_jumping == false {
                camera_transform.translation.x = camera_transform.translation.x + ((player_x - horizontal_offset) - camera_transform.translation.x) * scalar_x;
                camera_transform.translation.z = player_transform.translation.z;
            }

            if state.new.0 == PlayerMoveState::Idle || state.new.0 == PlayerMoveState::Run {
                camera_var.new_ground_height = player_y; //
            }
            else if (camera_transform.translation.y - player_y) < -box_ceilling  {
                camera_var.new_ground_height = player_y; //
            }
            else if (camera_transform.translation.y - player_y) > -box_floor  {
                camera_var.new_ground_height = player_y; //
            }


            //camera_transform.translation.y = camera_transform.translation.y + (camera_var.new_ground_height - camera_transform.translation.y) * scalar_y;
            camera_transform.translation.y += velocity;
            
            camera_transform.translation = (camera_transform.translation * 8.0).round() / 8.0;

            //dbg!(camera_transform.translation.x);



                    
        }
    }
}
use bevy::prelude::*;
use bevy_rapier2d::{
	prelude::{Collider, InteractionGroups, QueryFilter, RapierContext},
	rapier::prelude::Group,
};

use crate::core_game::player::player_structs::PlayerInput;
use crate::core_game::player::player_structs::PlayerCasts;
use crate::core_game::player::player_structs::Player;

pub fn player_casts (
	mut query: Query<
		(
			&Collider,
			&Transform,
			&PlayerInput,
			&mut PlayerCasts,
		),
		With<Player>,
	>,
	rapier_context: Res<RapierContext>,
) {
    
    for (collider, transform, input, mut cast) in query.iter_mut() {
        
        
        
        // INITIALIZE VARIABLES
	   cast.basic_up = false;
	   cast.basic_down = false;
	   cast.basic_left = false;
	   cast.basic_right = false;
	   cast.directional_x = false;
	   cast.enemy_penetration = false;
	   let mut move_direction = 0.0;
	   let raycast = 0.0101;
	   let big_raycast = 16.0;
    
    
    
        // DETERMINE MOVE DIRECTION
        let move_left = input.pressing_left;
	   let move_right = input.pressing_right;
	   if move_right && !move_left {
		  move_direction = 1.0;
	   }
	   if move_left && !move_right {
		  move_direction = -1.0
		  
	   }
	   if move_right && move_left {
		  move_direction = 0.0;
	   }
	   if !move_right && !move_left {
		  move_direction = 0.0;
	   }
	   
	   
	   
	   // BASIC UP
	   let hit_up = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::new(0.0, raycast),
            collider,
            1.0,
            QueryFilter::default()
            .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1)),
        );
        if hit_up.is_some() {
            cast.basic_up = true;
        }
        
        
        
        // BASIC DOWN
        let hit_down = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::new(0.0, -raycast),
            collider,
            1.0,
            QueryFilter::default()
            .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1)),
        );
        if hit_down.is_some() {
            cast.basic_down = true;
        }
        
        
        
        // BASIC LEFT
        let hit_left = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::new(-raycast, 0.0),
            collider,
            1.0,
            QueryFilter::default()
            .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1)),
        );
        if hit_left.is_some() {
            cast.basic_left = true;
        }
        
        
        
        // BASIC RIGHT
        let hit_right = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::new(raycast, 0.0),
            collider,
            1.0,
            QueryFilter::default()
            .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1)),
        );
        if hit_right.is_some() {
            cast.basic_right = true;
        }
        
        
        
        // DIRECTIONAL X
        if move_direction != 0.0 {
            let hit_dir_x = rapier_context.cast_shape(
                transform.translation.truncate(),
                0.0,
                Vec2::new(raycast * move_direction, 0.0),
                collider,
                1.0,
                QueryFilter::default()
                .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1)),
            );
            if hit_dir_x.is_some() {
                cast.directional_x = true;
            }
        }
        
        
        
        // BIG LEFT
        let hit_big_left = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::new(-big_raycast, 0.0),
            collider,
            1.0,
            QueryFilter::default()
            .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1)),
        );
        if hit_big_left.is_some() {
            cast.big_left = true;
        }
        
        
        
        // BIG RIGHT
        let hit_big_right = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::new(big_raycast, 0.0),
            collider,
            1.0,
            QueryFilter::default()
            .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1)),
        );
        if hit_big_right.is_some() {
            cast.big_right = true;
        }
        

        
        // WALLSLIDE ANIM UP
        let hit_ws_up = rapier_context.cast_shape(
            transform.translation.truncate() + Vec2::new(0.0, 18.0),
            0.0,
            Vec2::new(move_direction, 0.0),
            collider,
            1.0,
            QueryFilter::default()
            .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1)),
        );
        if hit_ws_up.is_some() {
            cast.wallslide_anim_up = true;
        }
        
        
        
        // WALLSLIDE ANIM DOWN
        let hit_ws_down = rapier_context.cast_shape(
            transform.translation.truncate() + Vec2::new(0.0, -18.0),
            0.0,
            Vec2::new(move_direction, 0.0),
            collider,
            1.0,
            QueryFilter::default()
            .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1)),
        );
        if hit_ws_down.is_some() {
            cast.wallslide_anim_down = true;
        }
        
        
        
        // ENEMY PENETRATION
        let hit_enemy = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::new(0.0, 0.01),
            collider,
            1.0,
            QueryFilter::default().groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_3)),
        );
        if hit_enemy.is_some() {
            cast.enemy_penetration = true;
        }
        
        
        // ENEMIES ON SCREEN
        let mut targets = Vec::new();
        loop {
            let hit_left = rapier_context.cast_shape(
                transform.translation.truncate(),
                0.0,
                Vec2::new(-100.0, 0.0),
                &Collider::cuboid(2.0, 36.0),
                1.0,
                QueryFilter::default()
                .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_3))
                .predicate(&|e| !targets.iter().any(|t| *t == e)),
            );
            let hit_right = rapier_context.cast_shape(
                transform.translation.truncate(),
                0.0,
                Vec2::new(100.0, 0.0),
			 &Collider::cuboid(2.0, 36.0),
                1.0,
                QueryFilter::default()
                .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_3))
                .predicate(&|e| !targets.iter().any(|t| *t == e)),
            );    
            if let Some((entity, _toii)) = hit_left {
                targets.push(entity);
            } else if let Some((entity, _toii)) = hit_right {
                targets.push(entity);
            } else {
                break;
            }
        }
        
        cast.nearby_enemies = targets.len();
        
        //dbg!(cast.nearby_enemies);
    }
}

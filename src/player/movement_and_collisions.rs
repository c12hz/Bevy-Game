use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{Collider, InteractionGroups, QueryFilter, RapierContext, TOIStatus},
    rapier::prelude::Group,
};
use crate::player::Vel;

use crate::player::Player;
use crate::player::PlayerState;
use crate::player::PlayerMoveState;

pub fn movement_and_collisions(
    mut queryyy: Query<(&mut Transform, &Collider, &Vel, &PlayerState), With<Player>>,
    rapier_context: Res<RapierContext>){

    for (mut transform, collider, velocity, state) in queryyy.iter_mut() {
        let mut velocity_vector = Vec2::new(velocity.x, velocity.y);
        let mut direction_vector = Vec3::new(0.0, 0.0, 0.0);
        if velocity_vector.x > 0.0 {
            direction_vector.x = 0.01;
        }
        if velocity_vector.x < 0.0 {
            direction_vector.x = -0.01;
        }
        if velocity_vector.y > 0.0 {
            direction_vector.y = 0.01;
        }
        if velocity_vector.y < 0.0 {
            direction_vector.y = -0.01;
        }

        let mut collided = false;

        

    
        for iter in 0..4 {
        // vertical shape cast
            let hit = rapier_context.cast_shape(
                transform.translation.truncate(),
                0.0,
                velocity_vector,
                collider,
                1.0,
                QueryFilter::default()
                .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1))
            );

            if let Some((collision, toi)) = hit {
                if let TOIStatus::Converged = toi.status {
                    if iter == 3 {
                        velocity_vector = Vec2::ZERO;
                        break;
                    }
                    let cross = Vec3::new(toi.normal2.x, toi.normal2.y, 0.0).cross(Vec3::Z);
                    velocity_vector = (cross * (cross.dot(Vec3::new(velocity_vector.x, velocity_vector.y, 0.0)))).truncate();
                    //dbg!(info.self_end_position);
                    transform.translation = toi.witness2.extend(0.0) - direction_vector;
                    collided = true;
                } else if let TOIStatus::Penetrating = toi.status {
                }
            }
            else {
                collided = false;
            }
        }

        transform.translation += velocity_vector.extend(0.0);

        
        // the code below rounds up the player transform to multiples of 0.125 (game scale unit) whenever it is safe to do so.
            //this ensures there are no ugly long decimal points in the player transform whenever possible

        if collided == false && (state.new.0 == PlayerMoveState::Idle || state.new.0 == PlayerMoveState::Run || state.new.0 == PlayerMoveState::Whirlwind)  {
            let hit1 = rapier_context.cast_shape(
                transform.translation.truncate(),
                0.0,
                Vec2::new(((transform.translation.x * 8.0).round() / 8.0) - transform.translation.x, 0.0) * 1.5,
                collider,
                1.0,
                QueryFilter::default()
                .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1))
            );
            if hit1.is_none() {

                transform.translation.x = (transform.translation.x * 8.0).round() / 8.0;
            }

            let hit2 = rapier_context.cast_shape(
                transform.translation.truncate(),
                0.0,
                Vec2::new(0.0, ((transform.translation.y * 8.0).round() / 8.0) - transform.translation.y) * 1.5,
                collider,
                1.0,
                QueryFilter::default()
                .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1))
            );
            if hit2.is_none() {

                transform.translation.y = (transform.translation.y * 8.0).round() / 8.0;
            }
            
        }

        if collided == false {
            let hit3 = rapier_context.cast_shape(
                transform.translation.truncate(),
                0.0,
                (((transform.translation.truncate() * 8.0).round() / 8.0) - transform.translation.truncate()) * 3.0,
                collider,
                1.0,
                QueryFilter::default()
                .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1))
            );
            if hit3.is_none() {

                transform.translation = (transform.translation * 8.0).round() / 8.0;
            }



        }

        if collided == true {   
        }
    }
}





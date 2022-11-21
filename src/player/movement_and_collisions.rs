use bevy::prelude::*;
use heron::rapier_plugin::{PhysicsWorld, ShapeCastCollisionType};
use heron::{CollisionLayers, CollisionShape};

use crate::player::Vel;
use crate::world::ColliderTypes;
use crate::player::Player;
use crate::player::PlayerState;
use crate::player::PlayerMoveState;

pub fn movement_and_collisions(
    mut queryyy: Query<(&mut Transform, &CollisionShape, &Vel, &PlayerState), With<Player>>,
    physics_world: PhysicsWorld){

    for (mut transform, collider, velocity, state) in queryyy.iter_mut() {
        let mut velocity_vector = Vec3::new(velocity.x, velocity.y, 0.0);
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
            let hit = physics_world.shape_cast_with_filter(
                collider,
                transform.translation,
                transform.rotation,
                velocity_vector * 1.1,
                CollisionLayers::none()
                .with_group(ColliderTypes::Player)
                .with_mask(ColliderTypes::World),
                |_enityty| true,
            );

            if let Some(collision) = hit {
                if let ShapeCastCollisionType::Collided(info) = collision.collision_type {
                    if iter == 3 {
                        velocity_vector = Vec3::ZERO;
                        break;
                    }
                    let cross = info.self_normal.cross(Vec3::Z);
                    velocity_vector = cross * (cross.dot(velocity_vector));
                    //dbg!(info.self_end_position);
                    transform.translation = info.self_end_position - direction_vector;
                    collided = true;
                } else if let ShapeCastCollisionType::AlreadyPenetrating = collision.collision_type {
                }
            }
            else {
                collided = false;
            }
        }

        transform.translation += velocity_vector;



        // the code below rounds up the player transform to multiples of 0.25 (game scale unit) whenever it is safe to do so.
            //this ensures there are no ugly long decimal points in the player transform whenever possible

        if collided == false && (state.new.0 == PlayerMoveState::Idle || state.new.0 == PlayerMoveState::Run || state.new.0 == PlayerMoveState::Whirlwind)  {
            let hit1 = physics_world.shape_cast_with_filter(
                collider,
                transform.translation,
                transform.rotation,
                Vec3::new(((transform.translation.x * 8.0).round() / 8.0) - transform.translation.x, 0.0, 0.0) * 1.5,
                CollisionLayers::none()
                .with_group(ColliderTypes::Player)
                .with_mask(ColliderTypes::World),
                |_enityty| true,
            );
            if hit1.is_none() {

                transform.translation.x = (transform.translation.x * 8.0).round() / 8.0;
            }

            let hit2 = physics_world.shape_cast_with_filter(
                collider,
                transform.translation,
                transform.rotation,
                Vec3::new(0.0, ((transform.translation.y * 8.0).round() / 8.0) - transform.translation.y, 0.0) * 1.5,
                CollisionLayers::none()
                .with_group(ColliderTypes::Player)
                .with_mask(ColliderTypes::World),
                |_enityty| true,
            );
            if hit2.is_none() {

                transform.translation.y = (transform.translation.y * 8.0).round() / 8.0;
            }
            
        }

        if collided == false {
            let hit3 = physics_world.shape_cast_with_filter(
                collider,
                transform.translation,
                transform.rotation,
                (((transform.translation * 8.0).round() / 8.0) - transform.translation) * 3.0,
                CollisionLayers::none()
                .with_group(ColliderTypes::Player)
                .with_mask(ColliderTypes::World),
                |_enityty| true,
            );
            if hit3.is_none() {

                transform.translation = (transform.translation * 8.0).round() / 8.0;
            }



        }

        if collided == true {   
        }
    }
}





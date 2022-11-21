use bevy::prelude::*;
use rand::{thread_rng, Rng};
use heron::rapier_plugin::PhysicsWorld;
use heron::{CollisionLayers, CollisionShape};
use crate::creature::Creature;
use crate::creature::CreatureGraphics;
use crate::creature::CreatureState;
use crate::creature::CreatureStateVariables;
use crate::creature::CreatureUsefulVariables;
use crate::creature::Vel;
use crate::creature::MoveSpeed;
use crate::creature::CreatureMoveState;

use crate::world::ColliderTypes;
use crate::player::Player;

use super::CreatureGraphicsEntity;




pub fn apply_creature_state (
    mut query: Query<(&CollisionShape, &mut CreatureUsefulVariables, &Transform, &mut Vel, &MoveSpeed, &CreatureState, &CreatureStateVariables, &CreatureGraphicsEntity), With<Creature>>,
    mut query2: Query<&mut TextureAtlasSprite, With<CreatureGraphics>>,
    mut query_player: Query<&Transform, With<Player>>,
    physics_world: PhysicsWorld,
    mut reset_velocity: Local<bool>,
    mut chase_delay: Local<u32>,
) {

    for (collider, mut othervar, transform, mut velocity, speed, state, var, cg) in query.iter_mut() {
        if let Ok(mut sprite) = query2.get_mut(cg.0) {
            for transform_player in query_player.iter_mut() {

                let raycast = 40.0;
                let mut rng = thread_rng();




                // PATROL

                if state.new.0 == CreatureMoveState::Patrol {

                    let hit_right = physics_world.shape_cast_with_filter(
                        collider,
                        transform.translation + Vec3::new(raycast, 0.0, 0.0),
                        transform.rotation,
                        Vec3::new(0.0, -1.0, 0.0), 
                        CollisionLayers::none()
                        .with_group(ColliderTypes::Enemy)
                        .with_mask(ColliderTypes::World),
                        |_enityty| true,
                    );
        
                    let hit_left = physics_world.shape_cast_with_filter(
                        collider,
                        transform.translation + Vec3::new(-raycast, 0.0, 0.0),
                        transform.rotation,
                        Vec3::new(0.0, -1.0, 0.0),
                        CollisionLayers::none()
                        .with_group(ColliderTypes::Enemy)
                        .with_mask(ColliderTypes::World),
                        |_enityty| true,
                    );
        
        
        
                    // simple patrolling from one platform edge to another
                    if hit_left.is_some() && hit_right.is_none() {
                        velocity.x = -speed.x;
                        *reset_velocity = false;
                    }
                    
                    if hit_left.is_none() && hit_right.is_some() {
                        velocity.x = speed.x;
                        *reset_velocity = false;
                    }


                    if state.new.0 == CreatureMoveState::Patrol && state.old.0 != CreatureMoveState::Patrol {
                        *reset_velocity = true;
                    }
        
                    if *reset_velocity == true {
                        if sprite.flip_x == false {
                            velocity.x = speed.x;
                        }
                        if sprite.flip_x == true {
                            velocity.x = -speed.x;
                        }
                    }

                    //random direction switching
                    if state.new.0 == CreatureMoveState::Patrol && state.old.0 != CreatureMoveState::Patrol {
                        if hit_left.is_some() && hit_right.is_some() {
                            if rng.gen_range(0..9) > 4 {
                                velocity.x = velocity.x * -1.0;
                            }
                        }
                    }
                


                }




                // IDLE

                if state.new.0 == CreatureMoveState::Idle {
                    if velocity.x > 0.0 {
                        velocity.x -= 0.25;
                    }

                    if velocity.x < 0.0 {
                        velocity.x += 0.25;
                    }
                }




                // CHASE

                if state.new.0 == CreatureMoveState::Chase {
                    if state.old.0 != CreatureMoveState::Chase {
                        othervar.chase_delay = rng.gen_range(0..=25);
                    }
                    if othervar.chase_delay == 0 {
                        velocity.x = var.chase_direction * speed.x * 3.0;
                    }
                    if othervar.chase_delay > 0 {
                        othervar.chase_delay -= 1;
                    }
                }
                if state.new.0 != CreatureMoveState::Chase {
                    othervar.chase_delay = 0;
                }




                // ATTACK

                if state.new.0 == CreatureMoveState::Attack {
                    velocity.x = 0.0;

                    if state.old.0 != CreatureMoveState::Attack {
                        othervar.attack_delay = rng.gen_range(0..=25);
                    }
                    if othervar.attack_delay == 0 {
                        // play the attack animation
                    }
                    if othervar.chase_delay > 0 {
                        othervar.chase_delay -= 1;
                    }
                }
                if state.new.0 != CreatureMoveState::Attack {
                    othervar.attack_delay = 0;
                }



                // COLLISIONS

                let collision_right = physics_world.shape_cast_with_filter(
                    collider,
                    transform.translation,
                    transform.rotation,
                    Vec3::new(velocity.x, 0.0, 0.0), 
                    CollisionLayers::none()
                    .with_group(ColliderTypes::Enemy)
                    .with_mask(ColliderTypes::World),
                    |_enityty| true,
                );

                let collision_left = physics_world.shape_cast_with_filter(
                    collider,
                    transform.translation,
                    transform.rotation,
                    Vec3::new(velocity.x, 0.0, 0.0), 
                    CollisionLayers::none()
                    .with_group(ColliderTypes::Enemy)
                    .with_mask(ColliderTypes::World),
                    |_enityty| true,
                );

                if collision_right.is_some() || collision_left.is_some() {
                    velocity.x *= -1.0;
                }
                


                // SPRITE FLIP

                if velocity.x < 0.0 {
                    sprite.flip_x = true;
                }
                if velocity.x > 0.0 {
                    sprite.flip_x = false;
                }

                if state.new.0 == CreatureMoveState::Attack {
                    if transform.translation.x - transform_player.translation.x < 0.0 {
                        sprite.flip_x = false;
                    }
                    if transform.translation.x - transform_player.translation.x > 0.0 {
                        sprite.flip_x = true;
                    }
                }
            }
        }
    }
}

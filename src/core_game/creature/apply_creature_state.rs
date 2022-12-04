use bevy::prelude::*;
use rand::{thread_rng, Rng};
use bevy_rapier2d::{
    prelude::{Collider, InteractionGroups, QueryFilter, RapierContext, TOIStatus},
    rapier::prelude::Group,
};

use crate::core_game::creature::creature_structs::Creature;
use crate::core_game::creature::creature_structs::CreatureGraphics;
use crate::core_game::creature::creature_structs::CreatureState;
use crate::core_game::creature::creature_structs::CreatureStateVariables;
use crate::core_game::creature::creature_structs::CreatureUsefulVariables;
use crate::core_game::creature::creature_structs::Vel;
use crate::core_game::creature::creature_structs::MoveSpeed;
use crate::core_game::creature::creature_structs::CreatureMoveState;

use crate::core_game::player::player_structs::Player;

use crate::core_game::creature::creature_structs::CreatureGraphicsEntity;




pub fn apply_creature_state (
    mut query: Query<(&Collider, &mut CreatureUsefulVariables, &Transform, &mut Vel, &MoveSpeed, &CreatureState, &CreatureStateVariables, &CreatureGraphicsEntity), With<Creature>>,
    mut query2: Query<&mut TextureAtlasSprite, With<CreatureGraphics>>,
    mut query_player: Query<&Transform, With<Player>>,
    rapier_context: Res<RapierContext>,
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

                    let hit_right = rapier_context.cast_shape(
                        transform.translation.truncate() + Vec2::new(raycast, 0.0),
                        0.0,
                        Vec2::new(0.0, -1.0),
                        collider,
                        1.0,
                        QueryFilter::default()
                        .groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_1))
                    );
        
                    let hit_left = rapier_context.cast_shape(
                        transform.translation.truncate() + Vec2::new(-raycast, 0.0),
                        0.0,
                        Vec2::new(0.0, -1.0),
                        collider,
                        1.0,
                        QueryFilter::default()
                        .groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_1))
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
                        if sprite.flip_x == true {
                            velocity.x = speed.x;
                        }
                        if sprite.flip_x == false {
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
                        velocity.x -= 0.125;
                    }

                    if velocity.x < 0.0 {
                        velocity.x += 0.125;
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

                let collision_right = rapier_context.cast_shape(
                    transform.translation.truncate(),
                    0.0,
                    Vec2::new(velocity.x, 0.0),
                    collider,
                    1.0, 
                    QueryFilter::default()
                    .groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_1))
                );

                let collision_left = rapier_context.cast_shape(
                    transform.translation.truncate(),
                    0.0,
                    Vec2::new(velocity.x, 0.0),
                    collider,
                    1.0, 
                    QueryFilter::default()
                    .groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_1))
                );

                if collision_right.is_some() || collision_left.is_some() {
                    velocity.x *= -1.0;
                }
                


                // SPRITE FLIP

                if velocity.x < 0.0 {
                    sprite.flip_x = false;
                }
                if velocity.x > 0.0 {
                    sprite.flip_x = true;
                }

                if state.new.0 == CreatureMoveState::Attack && sprite.index == 0 {
                    if transform.translation.x - transform_player.translation.x < 0.0 {
                        sprite.flip_x = true;
                    }
                    if transform.translation.x - transform_player.translation.x > 0.0 {
                        sprite.flip_x = false;
                    }
                }
            }
        }
    }
}

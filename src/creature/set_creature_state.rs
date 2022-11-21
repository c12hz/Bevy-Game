
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use heron::rapier_plugin::{PhysicsWorld, ShapeCastCollisionType};
use heron::{CollisionLayers, CollisionShape};
use crate::creature::Creature;
use crate::creature::CreatureState;
use crate::creature::CreatureStateVariables;
use crate::creature::CreatureMoveState;

use crate::world::ColliderTypes;
use crate::player::StealthMode;
use crate::player::Player;





pub fn set_creature_state (
    mut commands: Commands,
    mut query: Query<(Entity, &CollisionShape, &Transform, &mut CreatureState, &mut CreatureStateVariables), With<Creature>>,
    query_player: Query<&StealthMode, With<Player>>,
    physics_world: PhysicsWorld,
) {
    let mut rng = thread_rng();


    let aggro_range = 96.0;
    let attack_range = 0.0;


    
    for (e, collider, transform, mut state, mut var) in query.iter_mut() {

        let patrol_duration: u32 = rng.gen_range(100..=300);
        let idle_duration: u32 = rng.gen_range(50..=150);

        if state.new.0 == CreatureMoveState::Chase && state.old.0 != CreatureMoveState::Chase {
            var.attack_range_offset = rng.gen_range(0.0..=6.0);
        }

        let look_right = physics_world.shape_cast_with_filter(
            collider,
            transform.translation,
            transform.rotation,
            Vec3::new(aggro_range, 0.0, 0.0),
            CollisionLayers::none()
            .with_group(ColliderTypes::Enemy)
            .with_mask(ColliderTypes::Player),
            |_enityty| true,
        );
    
        let look_left = physics_world.shape_cast_with_filter(
            collider,
            transform.translation,
            transform.rotation,
            Vec3::new(-aggro_range, 0.0, 0.0),
            CollisionLayers::none()
            .with_group(ColliderTypes::Enemy)
            .with_mask(ColliderTypes::Player),
            |_enityty| true,
        );

        let attack_range_right = physics_world.shape_cast_with_filter(
            collider,
            transform.translation,
            transform.rotation,
            Vec3::new(attack_range + var.attack_range_offset, 0.0, 0.0),
            CollisionLayers::none()
            .with_group(ColliderTypes::Enemy)
            .with_mask(ColliderTypes::Player),
            |_enityty| true,
        );
    
        let attack_range_left = physics_world.shape_cast_with_filter(
            collider,
            transform.translation,
            transform.rotation,
            Vec3::new(-attack_range - var.attack_range_offset, 0.0, 0.0),
            CollisionLayers::none()
            .with_group(ColliderTypes::Enemy)
            .with_mask(ColliderTypes::Player),
            |_enityty| true,
        );


        for stealth in query_player.iter() {


            if (look_right.is_none() && look_left.is_none()) || stealth.active {
                if var.patrol_timer > 0 {
                    state.old.0 = state.new.0;
                    state.new.0 = CreatureMoveState::Patrol;
                    var.patrol_timer -= 1;
                }
                if var.patrol_timer == 0 {
                    state.old.0 = state.new.0;
                    state.new.0 = CreatureMoveState::Idle;
                    if state.new.0 != state.old.0 {
                        var.idle_timer = idle_duration;
                    }
                }

                if var.idle_timer > 0 {
                    var.idle_timer -= 1;
                }
                else if state.new.0 == CreatureMoveState::Idle {
                    var.patrol_timer = patrol_duration;
                }
            }

            if ((look_right.is_some() || look_left.is_some()) && !(attack_range_right.is_some() || attack_range_left.is_some())) && !stealth.active {

                if look_right.is_some() {
                    let collision_right = physics_world.shape_cast_with_filter(
                        collider,
                        transform.translation,
                        transform.rotation,
                        Vec3::new(aggro_range, 0.0, 0.0),
                        CollisionLayers::none()
                        .with_group(ColliderTypes::Enemy)
                        .with_mask(ColliderTypes::World),
                        |_enityty| true,
                    );

                    if let Some(collision_wall) = collision_right {
                        if let ShapeCastCollisionType::Collided(infowall) = collision_wall.collision_type {
                            let look_right = physics_world.shape_cast_with_filter(
                                collider,
                                transform.translation,
                                transform.rotation,
                                Vec3::new(aggro_range, 0.0, 0.0),
                                CollisionLayers::none()
                                .with_group(ColliderTypes::Enemy)
                                .with_mask(ColliderTypes::Player),
                                |_enityty| true,
                            );
                            if let Some(collision_player) = look_right {
                                if let ShapeCastCollisionType::Collided(infoplayer) = collision_player.collision_type {
                                    if (infoplayer.self_end_position.x  - transform.translation.x).abs() < (infowall.self_end_position.x - transform.translation.x).abs() {
                                        state.old.0 = state.new.0;
                                        state.new.0 = CreatureMoveState::Chase;
                                        var.chase_direction = 1.0;
                                    }
                                    else {
                                        state.old.0 = state.new.0;
                                        state.new.0 = CreatureMoveState::Patrol;
                                    }
                                }
                            
                            }
                        }
                    }
                    else if collision_right.is_none() {
                        state.old.0 = state.new.0;
                        state.new.0 = CreatureMoveState::Chase;
                        var.chase_direction = 1.0;
                    }
                }

                if look_left.is_some() {
                    let collision_left = physics_world.shape_cast_with_filter(
                        collider,
                        transform.translation,
                        transform.rotation,
                        Vec3::new(-aggro_range, 0.0, 0.0),
                        CollisionLayers::none()
                        .with_group(ColliderTypes::Enemy)
                        .with_mask(ColliderTypes::World),
                        |_enityty| true,
                    );

                    if let Some(collision_wall) = collision_left {
                        if let ShapeCastCollisionType::Collided(infowall) = collision_wall.collision_type {
                            let look_left = physics_world.shape_cast_with_filter(
                                collider,
                                transform.translation,
                                transform.rotation,
                                Vec3::new(-aggro_range, 0.0, 0.0),
                                CollisionLayers::none()
                                .with_group(ColliderTypes::Enemy)
                                .with_mask(ColliderTypes::Player),
                                |_enityty| true,
                            );
                            if let Some(collision_player) = look_left {
                                if let ShapeCastCollisionType::Collided(infoplayer) = collision_player.collision_type {
                                    if (infoplayer.self_end_position.x  - transform.translation.x).abs() < (infowall.self_end_position.x - transform.translation.x).abs() {
                                        state.old.0 = state.new.0;
                                        state.new.0 = CreatureMoveState::Chase;
                                        var.chase_direction = -1.0;
                                    }
                                    else {
                                        state.old.0 = state.new.0;
                                        state.new.0 = CreatureMoveState::Patrol;
                                    }
                                }
                            
                            }
                        }
                    }
                    else if collision_left.is_none() {
                        state.old.0 = state.new.0;
                        state.new.0 = CreatureMoveState::Chase;
                        var.chase_direction = -1.0;
                    }
                }
            }
    
            if (attack_range_right.is_some() || attack_range_left.is_some()) && !stealth.active {
                state.old.0 = state.new.0;
                state.new.0 = CreatureMoveState::Attack;
            }
        }
    }
}

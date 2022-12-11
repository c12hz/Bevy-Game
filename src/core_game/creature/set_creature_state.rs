
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use bevy_rapier2d::{
    prelude::{Collider, InteractionGroups, QueryFilter, RapierContext, TOIStatus},
    rapier::prelude::Group,
};

use crate::core_game::creature::creature_structs::Creature;
use crate::core_game::creature::creature_structs::CreatureState;
use crate::core_game::creature::creature_structs::CreatureStateVariables;
use crate::core_game::creature::creature_structs::CreatureMoveState;

use crate::core_game::player::player_structs::StealthMode;
use crate::core_game::player::player_structs::Player;





pub fn set_creature_state (
    mut query: Query<(Entity, &Collider, &Transform, &mut CreatureState, &mut CreatureStateVariables), With<Creature>>,
    query_player: Query<&StealthMode, With<Player>>,
    rapier_context: Res<RapierContext>,
    mut chase_timer: Local<u32>,
) {
    let mut rng = thread_rng();



    let aggro_range = 96.0;
    let attack_range = 0.0;
    let chase_max_duration:u32 = 150;

    
    for (e, collider, transform, mut state, mut var) in query.iter_mut() {

        let patrol_duration: u32 = rng.gen_range(100..=300);
        let idle_duration: u32 = rng.gen_range(50..=150);

        if state.new.0 == CreatureMoveState::Chase && state.old.0 != CreatureMoveState::Chase {
            var.attack_range_offset = rng.gen_range(0.0..=6.0);
        }
        
        let look_right = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::new(aggro_range, 0.0),
            collider,
            1.0,
            QueryFilter::default()
            .groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2))
        );
    
        let look_left = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::new(-aggro_range, 0.0),
            collider,
            1.0,
            QueryFilter::default()
            .groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2))
        );

        let attack_range_right = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::new(attack_range + var.attack_range_offset, 0.0),
            collider,
            1.0,
            QueryFilter::default()
            .groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2))
        );
    
        let attack_range_left = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::new(-attack_range - var.attack_range_offset, 0.0),
            collider,
            1.0,
            QueryFilter::default()
            .groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2))
        );

        let isolation_range_left = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::new(-96.0, 0.0),
            collider,
            1.0,
            QueryFilter::default()
            .groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_3))
        );

        let isolation_range_right = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::new(96.0, 0.0),
            collider,
            1.0,
            QueryFilter::default()
            .groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_3))
        );


        // 'ISOLATED' STATE
        if isolation_range_left.is_some() || isolation_range_right.is_some() {
            var.isolated = false;
        }
        else {
            var.isolated = true;
        }


        for stealth in query_player.iter() {


            // PATROL STATE

            if (look_right.is_none() && look_left.is_none()) || stealth.active {
                if var.patrol_timer > 0 {
                    state.old.0 = state.new.0;
                    state.new.0 = CreatureMoveState::Patrol;
                    var.patrol_timer -= 1;
                }

            // IDLE STATE
            
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
                    let collision_right = rapier_context.cast_shape(
                        transform.translation.truncate(),
                        0.0, 
                        Vec2::new(aggro_range, 0.0),
                        collider,
                        1.0,
                        QueryFilter::default()
                        .groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_1))
                    );

                    if let Some((wall, toi1)) = collision_right {
                        if let TOIStatus::Converged = toi1.status {
                            let look_right = rapier_context.cast_shape(
                                transform.translation.truncate(),
                                0.0,
                                Vec2::new(aggro_range, 0.0),
                                collider,
                                1.0,
                                QueryFilter::default()
                                .groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2))
                            );
                            if let Some((player, toi2)) = look_right {
                                if let TOIStatus::Converged = toi2.status {
                                    if (toi2.witness1.x - transform.translation.x).abs() < (toi1.witness1.x - transform.translation.x).abs() {
                                        *chase_timer = chase_max_duration;
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
                        *chase_timer = chase_max_duration;
                        var.chase_direction = 1.0;
                    }
                }

                if look_left.is_some() {
                    let collision_left = rapier_context.cast_shape(
                        transform.translation.truncate(),
                        0.0,
                        Vec2::new(-aggro_range, 0.0),
                        collider,
                        1.0,
                        QueryFilter::default()
                        .groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_1))
                    );

                    if let Some((wall, toi1)) = collision_left {
                        if let TOIStatus::Converged = toi1.status {
                            let look_left = rapier_context.cast_shape(
                                transform.translation.truncate(),
                                0.0,
                                Vec2::new(-aggro_range, 0.0),
                                collider,
                                1.0,
                                QueryFilter::default()
                                .groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2))
                            );
                            if let Some(((player, toi2))) = look_left {
                                if let TOIStatus::Converged = toi2.status {
                                    if (toi2.witness1.x  - transform.translation.x).abs() < (toi1.witness1.x - transform.translation.x).abs() {
                                        *chase_timer = chase_max_duration;
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
                        *chase_timer = chase_max_duration;
                        var.chase_direction = -1.0;
                    }
                    
                }
            }

            if *chase_timer > 0 {
                state.old.0 = state.new.0;
                state.new.0 = CreatureMoveState::Chase;
                *chase_timer -= 1;
            }
    
            if (attack_range_right.is_some() || attack_range_left.is_some()) && !stealth.active {
                state.old.0 = state.new.0;
                state.new.0 = CreatureMoveState::Attack;
            }
        }
    }
}

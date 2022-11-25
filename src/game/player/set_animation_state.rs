use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{Collider, InteractionGroups, QueryFilter, RapierContext},
    rapier::prelude::Group,
};


use crate::player::PlayerState;
use crate::player::PlayerMoveState;
use crate::player::PlayerAnimationState;
use crate::player::PlayerDirectionState;
use crate::player::PlayerStateVariables;
use crate::player::Player;

use super::WallKick;



// this function generates player states
// the idea here is that player states are only dependant on: 
// 1) key presses,
// 2) basic collision checks,
// 3) basic variables (PlayerStateVariables) 
//  - those variables are only modified by the state machine and nothing else.
// this results in a clean 'signal flow', 
// where the outputs of the state machine (like velocity) 
// never feed back into its input.


pub fn set_animation_state (
    mut query: Query<(&mut PlayerState, &mut PlayerStateVariables, &Collider, &Transform, &WallKick), With<Player>>,
    rapier_context: Res<RapierContext>,
    keys: Res<Input<KeyCode>>,
    mut timer1: Local<u32>,
    mut timer2: Local<u32>,
    mut attacking: Local<bool>,
) {




    for (mut state, mut var, collider, transform, wall) in query.iter_mut() {


        if keys.just_pressed(KeyCode::G) {
            *timer1 = 15;
        }


        if keys.just_pressed(KeyCode::B) {
            *timer2 = 25;
        }




        if *timer1 > 0 || *timer2 > 0 {
            *attacking = true;
        }
        else {
            *attacking = false;
        }



        // RESET PARAMETERS WHEN STATE CHANGES

        if state.new.0 != state.old.0 {
            var.runidle_counter = 0;
            var.idlewhirl_counter = 0;
            var.whirlidle_counter = 0;
        }

        let mut frame_count = 0;
        let frame_duration = 5;



        //RUN > IDLE TRANSITION ANIMATION STATE
        if state.old.0 == PlayerMoveState::Run && state.new.0 == PlayerMoveState::Idle && !*attacking {
            frame_count = 5;
            var.runidle_counter = (frame_count * frame_duration) + 1;
        }

        if var.runidle_counter > 0 {
            state.old.2 = state.new.2;
            state.new.2 = PlayerAnimationState::RunIdle;
            var.runidle_counter -= 1;

        }

        //IDLE > WHIRLWIND STATE - currently something is not working > stuck in transition animation when trying to use when not running
        if state.old.0 == PlayerMoveState::Idle && state.new.0 == PlayerMoveState::Whirlwind && !*attacking {
            frame_count = 2;
            var.idlewhirl_counter = (frame_count * frame_duration) + 1;
        }
        if var.idlewhirl_counter > 0 {
            state.old.2 = state.new.2;
            state.new.2 = PlayerAnimationState::IdleWhirlwind;
            var.idlewhirl_counter -= 1;
 
        }

        //WHIRLWIND > IDLE STATE
        if state.old.0 == PlayerMoveState::Whirlwind && state.new.0 == PlayerMoveState::Idle && !*attacking {
            frame_count = 2;
            var.whirlidle_counter = (frame_count * frame_duration) + 1;
        }
        if var.whirlidle_counter > 0 {
            state.old.2 = state.new.2;
            state.new.2 = PlayerAnimationState::WhirlwindIdle;
            var.whirlidle_counter -= 1;
            
        }

        // FALL > IDLE STATE

        if state.old.0 == PlayerMoveState::Fall && state.new.0 == PlayerMoveState::Run && !*attacking {
            //frame_count = 1;
            //var.fallidle_counter = (frame_count * frame_duration) + 1;
        }
        if var.fallidle_counter > 0 {
            //state.old.2 = state.new.2;
            //state.new.2 = PlayerAnimationState::FallIdle;
            //var.fallidle_counter -= 1;
            
        }


        let mut currently_transitioning = var.runidle_counter != 0 || var.idlewhirl_counter != 0 || var.whirlidle_counter != 0 || var.fallidle_counter != 0;






        // IDLE ANIMATION STATE
        if currently_transitioning == false && !*attacking {
            if state.new.0 == PlayerMoveState::Idle {
                state.old.2 = state.new.2;
                state.new.2 = PlayerAnimationState::Idle;
            }
        }


        // RUN ANIMATION STATE
        if currently_transitioning == false && !*attacking {
            if state.new.0 == PlayerMoveState::Run {
                state.old.2 = state.new.2;
                state.new.2 = PlayerAnimationState::Run;
            }
        }


        // JUMP ANIMATION STATE
        if currently_transitioning == false && !*attacking {
            if state.new.0 == PlayerMoveState::Jump {
                state.old.2 = state.new.2;
                state.new.2 = PlayerAnimationState::Jump;
            }
        }


        // FALL ANIMATION STATE
        if currently_transitioning == false && !*attacking {
            if state.new.0 == PlayerMoveState::Fall {
                state.old.2 = state.new.2;
                state.new.2 = PlayerAnimationState::Fall;
            }
        }


        // WALL SLIDE ANIMATION STATE
        if currently_transitioning == false && !*attacking {
            if state.new.0 == PlayerMoveState::WallSlide {
                let mut side = 0.0;

                if state.new.1 == PlayerDirectionState::Left {
                    side = -1.0;
                }

                if state.new.1 == PlayerDirectionState::Right {
                    side = 1.0;
                }


                let hit_ws_up = rapier_context.cast_shape(
                    Vec2::new(transform.translation.x, transform.translation.y) + Vec2::new(0.0, 18.0),
                    0.0,
                    Vec2::new(side, 0.0),
                    collider,
                    1.0,
                    QueryFilter::exclude_dynamic()
                    .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1))
                );

                let hit_ws_down = rapier_context.cast_shape(
                    Vec2::new(transform.translation.x, transform.translation.y) + Vec2::new(0.0, -18.0),
                    0.0,
                    Vec2::new(side, 0.0),
                    collider,
                    1.0,
                    QueryFilter::exclude_dynamic()
                    .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1))
                );

                if hit_ws_up.is_some() && hit_ws_down.is_some() {
                    state.old.2 = state.new.2;
                    state.new.2 = PlayerAnimationState::WallSlide;
                }

                if hit_ws_up.is_none() || hit_ws_down.is_none() {
                    state.old.2 = state.new.2;
                    state.new.2 = PlayerAnimationState::Fall;
                }

                
            }
        }

        // BASIC SWORD ATTACK ANIMATION STAT
        if *timer1 > 0 {
            state.old.2 = state.new.2;
            state.new.2 = PlayerAnimationState::SwordHitBasic;
            *timer1 -= 1;
            currently_transitioning = false
        }

        // BASIC HAMMER ATTACK ANIMATION STAT
        if *timer2 > 0 {
            state.old.2 = state.new.2;
            state.new.2 = PlayerAnimationState::HammerHitBasic;
            *timer2 -= 1;
            currently_transitioning = false
        }



        // WHIRLWIND ANIMATION STATE - currently something is not working > stuck in transition animation when trying to use when not running
        if currently_transitioning == false  {
            if state.new.0 == PlayerMoveState::Whirlwind {
                state.old.2 = state.new.2;
                state.new.2 = PlayerAnimationState::Whirlwind;
            }
        }

        // DASH ANIMATION STATE
        if currently_transitioning == false {
            if state.new.0 == PlayerMoveState::Dash {
                state.old.2 = state.new.2;
                state.new.2 = PlayerAnimationState::Run;
            }
        }
    }
}




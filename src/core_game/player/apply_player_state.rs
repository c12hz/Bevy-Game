use bevy::prelude::*;
//use bevy::render::render_resource::PipelineLayout;

use crate::core_game::player::player_structs::MoveSpeed;
use crate::core_game::player::player_structs::Vel;
use crate::core_game::player::player_structs::PlayerState;
use crate::core_game::player::player_structs::PlayerMoveState;
use crate::core_game::player::player_structs::PlayerDirectionState;
use crate::core_game::player::player_structs::Grav;
use crate::core_game::player::player_structs::Player;
use crate::core_game::player::player_structs::PlayerGraphics;
use crate::core_game::player::player_structs::WallKick;
use crate::core_game::player::player_structs::StealthMode;
use crate::core_game::player::player_structs::PlayerInput;

use crate::core_game::player::player_structs::PlayerStateVariables;



// This function applies the various movement related player states into actual variables like velocity etc.
// Improtant to note is I'm handling gravity in a very weird way, but I quite like this solution
// The value of gravity is always 1, and the strength of gravity depends on how often it is applied
// For instance during jump state gravity is applied every 4 frames
// While in Fall state it is applied every 3 frames.
// This results in a fall that's slighly faster than the jump, which is a common practice in platformer
// I quite like this solution with determining gravity strenth based on how often it is applied
// It gives much more precision and control over what's going on, and I'm really happy with how the jump/fall feels now
// Also you might notice there are 7 time-step frames with 0 vertical velocity at the very top of every jump (or 3 frames if you end the jump early)
// This is intentional, as I find it makes the jump feel better, and gives the player a little more time to position themselves in air


pub fn apply_player_state (
    mut query: Query<(&PlayerState, &PlayerStateVariables, &MoveSpeed, &mut Vel, &mut Grav, &mut WallKick, &StealthMode, &PlayerInput), With<Player>>,
    mut query2: Query<&mut TextureAtlasSprite, With<PlayerGraphics>>,
){

    
    let mut looking_direction: f32 = 0.0;


    for (state, var, speed, mut velocity, mut gravity, mut wall_kick, stealth, input) in query.iter_mut() {
        for mut sprite in query2.iter_mut() {


            let move_left = input.pressing_left;
            let move_right = input.pressing_right;

            let mut speed_x = speed.x;


            //FLIP SPRITE

            if state.new.1 == PlayerDirectionState::Right {
                sprite.flip_x = false;
            }
            if state.new.1 == PlayerDirectionState::Left {
                sprite.flip_x = true;
            }

            if sprite.flip_x == false {
                looking_direction = 1.0;
            }
            else {
                looking_direction = -1.0;
            }

            // STEALTH MODE

            if stealth.active {
                sprite.color = Color::Rgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                    alpha: 0.2,
                };
                if state.new.0 == PlayerMoveState::Run || state.new.0 == PlayerMoveState::Whirlwind {
                    speed_x = stealth.speed_x;
                }
            }
            else {
                sprite.color = Color::Rgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                    alpha: 1.0,
                };
            }

            // IDLE

            if state.new.0 == PlayerMoveState::Idle {

                // Deceleration in the Right direction
                if state.old.1 == PlayerDirectionState::Right {
                    velocity.dir = 1.0;
                }
                if velocity.dir == 1.0 {
                    if velocity.x > 0.0 {
                        velocity.x -= 0.25;
                    }
                    if velocity.x <= 0.0 {
                        velocity.x = 0.0;
                        velocity.dir = 0.0;
                    }
                }

                // Deceleration in the Left direction
                if state.old.1 == PlayerDirectionState::Left {
                    velocity.dir = -1.0;
                }
                if velocity.dir == -1.0 {
                    if velocity.x < 0.0 {
                        velocity.x += 0.25;
                    }
                    if velocity.x >= 0.0 {
                         velocity.x = 0.0;
                         velocity.dir = 0.0;
                    }
                }

                // Resetting various variables
                gravity.speed = 0.0;
                gravity.counter = 0;
                velocity.y = 0.0;
            }


            // RUN

            if state.new.0 == PlayerMoveState::Run {
                if state.new.1 != state.old.1 {
                    velocity.x = 0.0;
                }
                // Acceleration + Run in the Right direction
                if state.new.1 == PlayerDirectionState::Right {
                    if velocity.x < speed_x {
                        velocity.x += 0.5;
                    }
                    if velocity.x > speed_x {
                        velocity.x = speed_x;
                    }
                }
                // Acceleration + Run in the Left direction
                if state.new.1 == PlayerDirectionState::Left {
                    if velocity.x > -speed_x {
                        velocity.x -= 0.5;
                    }
                    if velocity.x < -speed_x {
                        velocity.x = -speed_x;
                    }
                }

                // Resetting various variables
                gravity.speed = 0.0;
                gravity.counter = 0;
                velocity.y = 0.0
            }



            // JUMP

            if state.new.0 == PlayerMoveState::Jump {

                if state.new.1 != state.old.1 {
                    velocity.x = 0.0;
                }

                if state.new.0 != state.old.0 {
                    velocity.y = 0.0;
                    velocity.y += speed.y;
                }

                if state.new.0 != state.old.0 {
                    gravity.counter = 0;
                }
                

                if gravity.counter == 3 {
                velocity.y -= gravity.strength;
                }

                gravity.counter += 1;

                if gravity.counter > 3 {
                    gravity.counter = 1;
                }

                if velocity.y < 0.0 {
                    velocity.y = 0.0;
                }

                // Acceleration + Jump in the Right direction
                if state.new.1 == PlayerDirectionState::Right {
                    if velocity.x < speed_x {
                        velocity.x += 0.25;
                    }
                    if velocity.x > speed_x {
                        velocity.x = speed_x;
                    }
                }

                // Acceleration + Jump in the Left direction
                if state.new.1 == PlayerDirectionState::Left {
                    if velocity.x > -speed_x {
                        velocity.x -= 0.25;
                    }
                    if velocity.x < -speed_x {
                        velocity.x = -speed_x;
                    }
                }

                // Deceleration during Jump, currently instant (feels precise)
                if state.new.1 == PlayerDirectionState::None{
                    velocity.x = 0.0;
                }


                // Wall Jump
                if state.old.0 == PlayerMoveState::WallSlide {
                    wall_kick.timer += 1;
                    if move_right && !move_left {
                        wall_kick.wall_direction = 1.0;
                    } else if move_left && !move_right {
                        wall_kick.wall_direction = -1.0;
                    } else {
                       wall_kick.wall_direction = 0.0;
                    }
                }

                if wall_kick.timer > 0 {
                    velocity.x = speed_x * -wall_kick.wall_direction;
                    wall_kick.timer += 1;
                }

                if wall_kick.timer >= 7 {
                    wall_kick.timer = 0;
                }

            }



            // FALL

            if state.new.0 == PlayerMoveState::Fall {
                if state.new.1 != state.old.1 {
                    velocity.x = 0.0;
                }

                if state.old.0 == PlayerMoveState::Idle || state.old.0 == PlayerMoveState::Run {
                    if velocity.y > 0.0 {
                        velocity.y = 0.0;
                    }
                    gravity.counter = 2;
                }

                // Adds a bit of smoothing when players end jump early, so velocity doesn't go to 0 instantly
                if state.old.0 == PlayerMoveState::Jump {
                    if velocity.y > 0.0 {
                        velocity.y = (velocity.y / 3.0).round();
                    }
                    gravity.counter = 1;
                }

                
                if gravity.counter == 2 {
                    velocity.y -= gravity.strength;
                }

                gravity.counter += 1;

                if gravity.counter > 2 {
                    gravity.counter = 1;
                }

                if velocity.y < -gravity.max_speed {
                    velocity.y = -gravity.max_speed;
                }


                // Acceleration + Fall in the Right direction
                if state.new.1 == PlayerDirectionState::Right {
                    if velocity.x < speed_x {
                        velocity.x += 1.0;
                    }
                    if velocity.x > speed_x {
                        velocity.x = speed_x;
                    }
                }

                // Acceleration + Fall in the Left direction
                if state.new.1 == PlayerDirectionState::Left {
                    if velocity.x > -speed_x {
                        velocity.x -= 1.0;
                    }
                    if velocity.x < -speed_x {
                        velocity.x = -speed_x;
                    }
                }

                // Deceleration during Fall, currently instant (feels precise)
                if state.new.1 == PlayerDirectionState::None{
                    velocity.x = 0.0;
                }
            }




            // WALL SLIDE

            if state.new.0 == PlayerMoveState::WallSlide {
                if state.old.0 == PlayerMoveState::Idle || state.old.0 == PlayerMoveState::Run {
                    if velocity.y > 0.0 {
                        velocity.y = 0.0;
                    }
                    gravity.counter = 3;
                }

                if state.old.0 == PlayerMoveState::Jump {
                    if velocity.y > 0.0 {
                        velocity.y = 0.0;
                    }
                    gravity.counter = 1;
                }

                
                if gravity.counter == 3 {
                    velocity.y -= 1.0;
                }

                gravity.counter += 1;

                if gravity.counter > 3 {
                    gravity.counter = 1;
                }

                if velocity.y < -gravity.slide_speed {
                    velocity.y = -gravity.slide_speed;
                }


                if state.new.1 == PlayerDirectionState::Right {
                    velocity.x = speed_x;
                }

                if state.new.1 == PlayerDirectionState::Left {
                    velocity.x = -speed_x;
                }

                if state.new.1 == PlayerDirectionState::None{
                    velocity.x = 0.0;
                }

            }



            // WHIRLWIND

            if state.new.0 == PlayerMoveState::Whirlwind {
                if state.new.1 != state.old.1 {
                    velocity.x = 0.0;
                }
                if state.new.1 == PlayerDirectionState::Right {
                    if velocity.x < speed_x {
                        velocity.x += 2.5;
                    }
                    if velocity.x > speed_x {
                        velocity.x = speed_x;
                    }
                }

                if state.new.1 == PlayerDirectionState::Left {
                    if velocity.x > -speed_x {
                        velocity.x -= 2.5;
                    }
                    if velocity.x < -speed_x {
                        velocity.x = -speed_x;
                    }
                }

                if state.new.1 == PlayerDirectionState::None{
                    velocity.x = 0.0;
                }
                gravity.speed = 0.0;
                gravity.counter = 0;
                velocity.y = 0.0
            }


            // DASH

            if state.new.0 == PlayerMoveState::DashForward {
                velocity.y = 0.0;
                velocity.x = 7.0 * looking_direction;
            }
            if state.old.0 == PlayerMoveState::DashForward && state.new.0 == PlayerMoveState::Idle {
                velocity.x = 0.0;
            }


            // DASH STRIKE

            if state.new.0 == PlayerMoveState::DashDown45 {
                velocity.y = -6.0;
                velocity.x = 6.0 * looking_direction;
            }
            if state.old.0 == PlayerMoveState::DashDown45 && state.new.0 != PlayerMoveState::DashDown45 {
                velocity.x = 0.0;
                velocity.y = 0.0;
            }
            // ACTIVE COLLISION WITH VERTICAL WALL
            // this removes ugly numbers from shapecast collision calculations
            if var.actively_colliding == true {
                if wall_kick.timer == 0 {
                    velocity.x = 0.0;
                }
            }



            // ADD "FRICTION" BETWEEN PLAYER AND ENEMIES
            if var.penetrating_enemy {
                if !(state.new.0 == PlayerMoveState::DashForward || state.new.0 == PlayerMoveState::DashDown45) {
                    velocity.x = ((velocity.x / 1.2) * 8.0).round() / 8.0;
                }
                
            }
        }
    }
}
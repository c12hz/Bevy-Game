use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::core_game::creature::creature_structs::Creature;
use crate::core_game::creature::creature_structs::CreatureCasts;
use crate::core_game::creature::creature_structs::CreatureGraphics;
use crate::core_game::creature::creature_structs::CreatureMoveState;
use crate::core_game::creature::creature_structs::CreatureState;
use crate::core_game::creature::creature_structs::CreatureStateVariables;
use crate::core_game::creature::creature_structs::CreatureUsefulVariables;
use crate::core_game::creature::creature_structs::MoveSpeed;
use crate::core_game::creature::creature_structs::Vel;

use crate::core_game::player::player_structs::Player;

use crate::core_game::creature::creature_structs::CreatureGraphicsEntity;

pub fn apply_creature_state(
	mut query: Query<
		(
			&mut CreatureUsefulVariables,
			&Transform,
			&mut Vel,
			&MoveSpeed,
			&CreatureState,
			&CreatureStateVariables,
			&CreatureGraphicsEntity,
			&CreatureCasts
		),
		With<Creature>,
	>,
	mut query2: Query<&mut TextureAtlasSprite, With<CreatureGraphics>>,
	mut query_player: Query<&Transform, With<Player>>,
	mut reset_velocity: Local<bool>,
	mut chase_delay: Local<u32>,
) {
	for (mut othervar, transform, mut velocity, speed, state, var, cg, cast) in query.iter_mut()
	{
		if let Ok(mut sprite) = query2.get_mut(cg.0) {
			for transform_player in query_player.iter_mut() {
				let mut rng = thread_rng();
				let mut on_edge = false;
				let mut colliding = false;
				let mut player_position = 0.0;
				if !cast.down_left || !cast.down_right {
					on_edge = true;
				}
				if cast.basic_left || cast.basic_right {
					colliding = true;
				}
				if transform.translation.x - transform_player.translation.x < 0.0 {
					player_position = 1.0
				}
				
				if transform.translation.x - transform_player.translation.x > 0.0 {
					player_position = -1.0
				}
				
				

				// PATROL
				if state.new.0 == CreatureMoveState::Patrol {
					// simple patrolling from one platform edge to another
					if cast.down_left && !cast.down_right {
						velocity.x = -speed.x;
						*reset_velocity = false;
					}

					if !cast.down_left && cast.down_right {
						velocity.x = speed.x;
						*reset_velocity = false;
					}

					if state.new.0 == CreatureMoveState::Patrol
						&& state.old.0 != CreatureMoveState::Patrol
					{
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
					if state.new.0 == CreatureMoveState::Patrol
						&& state.old.0 != CreatureMoveState::Patrol
					{
						if cast.down_left && cast.down_right {
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
					if !cast.down_right && player_position >= 0.0 {
						velocity.x *= 0.0;
					}
					if !cast.down_left && player_position <= 0.0 {
						velocity.x *= 0.0;
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
				
				
				
				// RETREAT
				if state.new.0 == CreatureMoveState::Retreat {
					if player_position == 1.0 {
						velocity.x = -speed.x
					}
					if player_position == -1.0 {
						velocity.x = speed.x;
					}
				}
				
				// RETREAT
				if state.new.0 == CreatureMoveState::RangedAttack {
					velocity.x = 0.0;
				}
				
				
				
				// DEFENCE
				if state.new.0 == CreatureMoveState::Defence {
					velocity.x = 0.0;
				}

				
				
				// COLLISIONS
				if colliding {
					velocity.x *= -1.0;
				}

				
				
				// SPRITE FLIP
				if velocity.x < 0.0 {
					sprite.flip_x = false;
				}
				if velocity.x > 0.0 {
					sprite.flip_x = true;
				}
				if state.new.0 == CreatureMoveState::Retreat || state.new.0 == CreatureMoveState::RangedAttack || state.new.0 == CreatureMoveState::Defence {
					if player_position == 1.0 {
						sprite.flip_x = true;
					}
					if player_position == -1.0 {
						sprite.flip_x = false;
					}
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

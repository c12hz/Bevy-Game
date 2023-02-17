use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::core_game::creature::creature_structs::Creature;
use crate::core_game::creature::creature_structs::CreatureCasts;
use crate::core_game::creature::creature_structs::CreatureMoveState;
use crate::core_game::creature::creature_structs::CreatureState;
use crate::core_game::creature::creature_structs::CreatureStateVariables;

use crate::core_game::player::player_structs::Player;
use crate::core_game::player::player_structs::PlayerCasts;
use crate::core_game::player::player_structs::StealthMode;

pub fn set_creature_state(
	mut query: Query<
		(
			Entity,
			&Transform,
			&mut CreatureState,
			&mut CreatureStateVariables,
			& CreatureCasts,
		),
		With<Creature>,
	>,
	query_player: Query<(&Transform, &StealthMode, &PlayerCasts), With<Player>>,
) {
	
	for (e, transform, mut state, mut var, cast) in query.iter_mut() {
		let mut rng = thread_rng();
		var.isolated = true;

		for (transform_player, stealth, player_cast) in query_player.iter() {
			
			var.distance_from_player = (transform_player.translation.x - transform.translation.x).abs();
			
			
			
			// PATROL STATE 			
			if !cast.sight_range && var.chase_timer == 0 || stealth.active {
				if !var.switch && var.idle_timer == 0 {
					var.patrol_timer = rng.gen_range(100..=300);
					var.switch = true;
				}
				if var.patrol_timer > 0 {
					state.old.0 = state.new.0;
					state.new.0 = CreatureMoveState::Patrol;
					var.patrol_timer -= 1;
				}
			}
			
			
			
			// IDLE STATE
			if !cast.sight_range && var.chase_timer == 0 || stealth.active {
				if var.switch && var.patrol_timer == 0 {
					var.idle_timer = rng.gen_range(50..=150);
					var.switch = false;
				}
				if var.idle_timer > 0 {
					state.old.0 = state.new.0;
					state.new.0 = CreatureMoveState::Idle;
					var.idle_timer -= 1;
				}
			}
			
			
			
			// 'ISOLATED' STATE
			if player_cast.nearby_enemies > 1 {
				var.isolated = false;
			}

			
			
			// CHASE STATE
			if transform.translation.x < transform_player.translation.x {
				var.chase_direction = 1.0;
			} else {
				var.chase_direction = -1.0;
			}
			if cast.chase_range && !stealth.active && !var.isolated && !cast.attack_range {
				var.chase_timer = 180;
			}
			if var.isolated {
				var.chase_timer = 0;
			}
			if var.chase_timer > 0 {
				var.chase_timer -= 1;
				if !cast.attack_range && !var.isolated {
					state.old.0 = state.new.0;
					state.new.0 = CreatureMoveState::Chase;	
				}
			}
			
			
			
			// ATTACK STATE
			if cast.attack_range && !var.isolated {
				state.old.0 = state.new.0;
				state.new.0 = CreatureMoveState::Attack;
			}	
			

			
			
			// RANGED ATTACK STATE
			if cast.sight_range && var.isolated {
				state.old.0 = state.new.0;
				state.new.0 = CreatureMoveState::RangedAttack;
				if var.switch2 && var.retreat_timer == 0.0 {
					var.retreating_attack_timer = 75;
					var.switch2 = false;
				}
				if var.retreating_attack_timer > 0 {
					state.old.0 = state.new.0;
					state.new.0 = CreatureMoveState::RangedAttack;
					var.retreating_attack_timer -= 1;
				}
			} else {
				var.switch2 = false;
				var.retreat_timer = 0.0;
				var.retreating_attack_timer = 0;
			}
			
			// HELPING CHASE
			if var.isolated && cast.help_range && var.distance_from_player >= 100.0 {
				state.old.0 = state.new.0;
				state.new.0 = CreatureMoveState::Chase;	
			}
			
			
			
			// RETREAT STATE
			if cast.retreat_range && var.isolated {
				if !var.switch2 && var.retreating_attack_timer == 0 {
					var.retreat_timer = rng.gen_range(50.0..=150.0);
					var.retreat_timer = (var.retreat_timer / 50.0).round() * 50.0;
					var.switch2 = true;
				}
				if var.retreat_timer > 0.0 {
					state.old.0 = state.new.0;
					state.new.0 = CreatureMoveState::Retreat;
					var.retreat_timer -= 1.0;
				}
			} else {
				var.switch2 = false;
				var.retreat_timer = 0.0;
				var.retreating_attack_timer = 0;
			}
				
			
			
			
			// DEFENCE STATE
			if cast.defence_range && var.isolated {
				state.old.0 = state.new.0;
				state.new.0 = CreatureMoveState::Defence;
			}
			
			//dbg!(state.new.0);
		}

	}
}

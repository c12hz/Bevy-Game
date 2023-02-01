use bevy::prelude::*;

use crate::core_game::player::player_structs::Player;
use crate::core_game::player::player_structs::PlayerAttackState;
use crate::core_game::player::player_structs::PlayerDamage;
use crate::core_game::player::player_structs::PlayerDirectionState;
use crate::core_game::player::player_structs::PlayerInput;
use crate::core_game::player::player_structs::PlayerMoveState;
use crate::core_game::player::player_structs::PlayerState;
use crate::core_game::player::player_structs::PlayerStateVariables;
use crate::core_game::player::player_structs::StealthMode;
use crate::core_game::player::player_structs::PlayerCasts;

// this function generates player states
// the idea here is that player states are only dependant on:
// 1) key presses,
// 2) basic collision checks,
// 3) basic variables (PlayerStateVariables)
//  - which are only used and modified by the state machine and nothing else.
// this results in a very clean 'signal flow',
// where the outputs of the state machine (like velocity)
// never feed back into its input.

pub fn set_move_state(
	mut query: Query<
		(
			&Transform,
			&PlayerDamage,
			&mut PlayerState,
			&mut PlayerStateVariables,
			&mut StealthMode,
			&PlayerInput,
			&PlayerCasts,
		),
		With<Player>,
	>,
) {
	
	for (transform, damage, mut state, mut var, mut stealth, input, cast) in query.iter_mut() {
		
		
		
		// INITIATE VARIABLES
		let jump_start = input.just_pressed_jump;
		let jump_pressed = input.pressing_jump;
		let stealth_mode = false;
		let whirl = false;
		let mut move_direction = 0.0;
		let raycast = 0.0101;
		let max_jumps = 2;
		let max_jump_duration = 24;	
		
		
		
		// DETERMINE MOVE DIRACTION
		let move_left = input.pressing_left;
		let move_right = input.pressing_right;
		if move_right && !move_left {
			move_direction = 1.0;
		}
		if move_left && !move_right {
			move_direction = -1.0
		}
		if move_right && move_left {
			move_direction = 0.0;
		}
		if !move_right && !move_left {
			move_direction = 0.0;
		}

		
		
		// FLIP SPRITE
		if move_direction == 1.0 {
			var.sprite_flipped = false;
		}
		if move_direction == -1.0 {
			var.sprite_flipped = true;
		}

		
		
		// IDLE MOVE STATE
		if !move_left
			&& !move_right
			&& !jump_start
			&& var.dash_counter == 0
			&& var.dash_strike_counter == 0
			&& !whirl
		{
			if cast.basic_down {
				state.old.0 = state.new.0;
				state.new.0 = PlayerMoveState::Idle;
				var.jumps_remaining = max_jumps;
				var.jump_frame_counter = 0;
			}
		}

		
		
		// RUN MOVE STATE
		if move_left
			|| move_right && !jump_start && var.dash_counter == 0 && var.dash_strike_counter == 0
		{
			if cast.basic_down {
				state.old.0 = state.new.0;
				state.new.0 = PlayerMoveState::Run;
				var.jumps_remaining = max_jumps;
			}
		}

		
		
		// JUMP MOVE STATE
		if jump_pressed && var.jumps_remaining > 0 && var.jump_frame_counter <= max_jump_duration {
			if !cast.basic_up && var.dash_counter == 0 && var.dash_strike_counter == 0 {
				state.old.0 = state.new.0;
				state.new.0 = PlayerMoveState::Jump;
				var.jump_frame_counter += 1;
			} else {
				var.jump_frame_counter = 100;
			}
		}
		if jump_pressed == false {
			var.jump_frame_counter = 0;
		}
		if state.new.0 == PlayerMoveState::Fall && state.old.0 == PlayerMoveState::Jump {
			if var.jumps_remaining > 0 {
				var.jumps_remaining -= 1;
			}
		}

		
		
		// FALL MOVE STATE
		if !jump_pressed || var.jump_frame_counter >= max_jump_duration {
			if !cast.basic_down
				&& !cast.directional_x
				&& var.dash_counter == 0
				&& var.dash_strike_counter == 0
			{
				state.old.0 = state.new.0;
				state.new.0 = PlayerMoveState::Fall;
			}
		} else {
			if cast.basic_up
				&& !cast.directional_x
				&& var.dash_counter == 0
				&& var.dash_strike_counter == 0
			{
				state.new.0 = PlayerMoveState::Fall;
			}
		}

		
		
		// WALL SLIDE MOVE STATE
		if (!jump_pressed || var.jump_frame_counter >= max_jump_duration)
			&& (move_left || move_right)
		{
			if !cast.basic_down {
				if cast.directional_x && var.dash_counter == 0 && var.dash_strike_counter == 0 {
					state.old.0 = state.new.0;
					state.new.0 = PlayerMoveState::WallSlide;
					var.jumps_remaining = max_jumps;
				};
			} else {
				if cast.basic_up
					&& cast.directional_x && var.dash_counter == 0
					&& var.dash_strike_counter == 0
				{
					state.old.0 = state.new.0;
					state.new.0 = PlayerMoveState::WallSlide;
					var.jumps_remaining = max_jumps;
					var.walljump_counter = 0;
				}
			}
		}
		if state.new.0 == PlayerMoveState::Jump && state.old.0 == PlayerMoveState::WallSlide {
			var.walljump_counter = 24;
		} else if var.walljump_counter > 0 {
			var.walljump_counter -= 1;
		}

		
		
		// DIRECTION STATE
		if move_right && !move_left {
			state.old.1 = state.new.1;
			state.new.1 = PlayerDirectionState::Right;
		}
		if move_left && !move_right {
			state.old.1 = state.new.1;
			state.new.1 = PlayerDirectionState::Left;
		}
		if !move_left && !move_right {
			state.old.1 = state.new.1;
			state.new.1 = PlayerDirectionState::None;
		}

		
		
		// DASH MOVE STATE
		if state.new.3 == PlayerAttackState::DashForward {
			state.old.0 = state.new.0;
			state.new.0 = PlayerMoveState::DashForward;
		}

		
		
		// DASH STRIKE STATE
		if state.new.3 == PlayerAttackState::DashDown45 {
			state.old.0 = state.new.0;
			state.new.0 = PlayerMoveState::DashDown45;
		}

		
		
		// STEALTH MODE
		if stealth.counter > 0 {
			stealth.counter -= 1;
		}
		if stealth_mode {
			stealth.counter = stealth.duration;
		}
		if damage.dealt {
			stealth.counter = 0;
		}
		if stealth.counter > 0 {
			stealth.active = true;
		} else {
			stealth.active = false;
		}

		
		
		// ENEMY PENETRATION STATE
		if cast.enemy_penetration {
			var.penetrating_enemy = true;
		} else {
			var.penetrating_enemy = false;
		}

		
		
		//HORIZONTAL COLLISION CHECK (detects if player is currenlty actively pushing into a vertical wall collision,
		//this is used by the apply_player_state function to make X velocity zero when colliding with vertical wall)
		//this improves stuff with collision shape casting, nothing major but it gets rid of ugly numbers
		if state.new.1 == PlayerDirectionState::Right {
			if cast.basic_right {
				var.actively_colliding = true;
			} else {
				var.actively_colliding = false;
			}
		}

		if state.new.1 == PlayerDirectionState::Left {
			if cast.basic_left {
				var.actively_colliding = true;
			} else {
				var.actively_colliding = false;
			}
		}
	}
}

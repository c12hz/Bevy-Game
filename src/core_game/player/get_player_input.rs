use bevy::prelude::*;

use crate::core_game::player::player_structs::PlayerInput;

pub fn get_player_input(mut qinput: Query<&mut PlayerInput>, keys: Res<Input<KeyCode>>) {
	for mut input in qinput.iter_mut() {
		// RESET 'PRESSING' INPUTS
		input.pressing_jump = false;
		input.pressing_left = false;
		input.pressing_right = false;
		input.pressing_dodge = false;
		input.pressing_skill1 = false;
		input.pressing_skill2 = false;
		input.pressing_skill3 = false;
		input.pressing_skill4 = false;
		input.pressing_up = false;

		// JUMP
		if keys.pressed(KeyCode::W) {
			input.pressing_jump = true;
		}

		if keys.just_pressed(KeyCode::W) {
			input.just_pressed_jump = true;
		}

		// RIGHT
		if keys.pressed(KeyCode::D) {
			input.pressing_right = true;
		}

		if keys.just_pressed(KeyCode::D) {
			input.just_pressed_right = true;
		}

		// LEFT
		if keys.pressed(KeyCode::A) {
			input.pressing_left = true;
		}

		if keys.just_pressed(KeyCode::A) {
			input.just_pressed_left = true;
		}

		// DODGE
		if keys.pressed(KeyCode::S) {
			input.pressing_dodge = true;
		}

		if keys.just_pressed(KeyCode::S) {
			input.just_pressed_dodge = true;
		}

		// SKILL1
		if keys.pressed(KeyCode::H) {
			input.pressing_skill1 = true;
		}

		if keys.just_pressed(KeyCode::H) {
			input.just_pressed_skill1 = true;
		}

		// SKILL2
		if keys.pressed(KeyCode::J) {
			input.pressing_skill2 = true;
		}

		if keys.just_pressed(KeyCode::J) {
			input.just_pressed_skill2 = true;
		}

		// SKILL3
		if keys.pressed(KeyCode::K) {
			input.pressing_skill3 = true;
		}

		if keys.just_pressed(KeyCode::K) {
			input.just_pressed_skill3 = true;
		}

		// SKILL4
		if keys.pressed(KeyCode::L) {
			input.pressing_skill4 = true;
		}

		if keys.just_pressed(KeyCode::L) {
			input.just_pressed_skill4 = true;
		}

		// SKILL4
		if keys.pressed(KeyCode::I) {
			input.pressing_up = true;
		}

		if keys.just_pressed(KeyCode::I) {
			input.just_pressed_up = true;
		}
	}
}

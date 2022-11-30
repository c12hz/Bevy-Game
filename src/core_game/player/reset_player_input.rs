use bevy::prelude::*;


use crate::core_game::player::player_structs::PlayerInput;


pub fn reset_player_input (
    mut qinput: Query<&mut PlayerInput>,
){

    for mut input in qinput.iter_mut() {
        // RESET INPUTS
        input.just_pressed_jump = false;
        input.just_pressed_left = false;
        input.just_pressed_right = false;
        input.just_pressed_dodge = false;
        input.just_pressed_skill1 = false;
        input.just_pressed_skill2 = false;
        input.just_pressed_skill3 = false;
        input.just_pressed_skill4 = false;
        input.just_pressed_up = false;
    }
}




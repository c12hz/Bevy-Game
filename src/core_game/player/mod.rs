use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub mod player_structs;
mod animate;
mod apply_player_state;
mod audio_test;
pub mod get_player_input;
pub mod reset_player_input;
mod move_camera;
mod movement_and_collisions;
mod player_deal_damage;
mod screen_shake;
mod set_animation_state;
mod set_move_state;
pub mod setup_player;
pub mod setup_camera;
mod switch_animation;
mod teleport_to_spawn;
mod time_divisions;
mod transfer_data;
mod set_attack_state;



pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_timestep_system("my_fixed", 0, set_attack_state::set_attack_state.label("set_attack_state"))
            .add_fixed_timestep_system("my_fixed", 0, set_move_state::set_move_state.label("set_move_state").after("set_attack_state"))
                .add_fixed_timestep_system("my_fixed", 0, apply_player_state::apply_player_state.label("apply_state").after("set_move_state"))
                    .add_fixed_timestep_system("my_fixed", 0, movement_and_collisions::movement_and_collisions.label("move").after("apply_state"))
                        .add_fixed_timestep_system("my_fixed", 0, teleport_to_spawn::teleport_to_spawn.after("move"))
                        .add_fixed_timestep_system("my_fixed", 0, transfer_data::transfer_data.after("move"))
                        .add_fixed_timestep_system("my_fixed", 0, move_camera::move_camera.label("move_camera").after("move"))
                            .add_fixed_timestep_system("my_fixed", 0, screen_shake::screen_shake.after("move_camera"))
                    .add_fixed_timestep_system("my_fixed", 0, set_animation_state::set_animation_state.label("set_anim").after("apply_state"))           
                        .add_fixed_timestep_system("my_fixed", 0, switch_animation::switch_animation.label("switch_anim").after("set_anim"))
                            .add_fixed_timestep_system("my_fixed", 0, time_divisions::time_divisions.label("time").after("set_anim"))
                                .add_fixed_timestep_system("my_fixed", 0, animate::animate.after("time").label("animate"))
                                    .add_fixed_timestep_system("my_fixed", 0, player_deal_damage::player_deal_damage.after("animate").label("deal_damage"))
                                        .add_fixed_timestep_system("my_fixed", 0, reset_player_input::reset_player_input.after("deal_damage"))
                                .add_fixed_timestep_system("my_fixed", 0, audio_test::audio_test.after("time"));
    }
}

/*
    let mut fixed_first = SystemStage::parallel();
    fixed_first
    .add_system(set_player_state.label("set_move_state"))
        .add_system(apply_player_state.label("apply_state").after("set_move_state"))
            .add_system(movement_and_collisions.label("move").after("apply_state"))
                .add_system(teleport_to_spawn.after("move"))
                .add_system(transfer_data.after("move"))
                .add_system(move_camera.label("move_camera").after("move"))
                    .add_system(screen_shake.after("move_camera"))
            .add_system(set_animation_state.label("set_anim").after("apply_state"))           
                .add_system(switch_animation.label("switch_anim").after("set_anim"))
                    .add_system(time_divisions.label("time").after("set_anim"))
                        .add_system(animate.after("time").label("animate"))
                            .add_system(player_deal_damage.after("animate").label("deal_damage"))
                        //.add_system(audio_test.after("time"))

*/
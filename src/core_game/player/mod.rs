use bevy::prelude::*;
use iyes_loopless::prelude::*;

mod animate;
mod apply_player_state;
mod audio_test;
mod generate_randomness;
pub mod get_player_input;
mod move_camera;
mod movement_and_collisions;
mod player_deal_damage;
pub mod player_structs;
pub mod reset_player_input;
mod screen_shake;
mod set_animation_state;
mod set_attack_state;
mod set_move_state;
pub mod setup_camera;
pub mod setup_player;
mod switch_animation;
mod teleport_to_spawn;
mod time_divisions;
mod transfer_data;
mod player_casts;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(
		&self,
		app: &mut App,
	) {
		app
		.add_fixed_timestep_system("my_fixed", 0, generate_randomness::generate_randomness)
		.add_fixed_timestep_system("my_fixed", 0, player_casts::player_casts.label("player_casts"))
		.add_fixed_timestep_system("my_fixed", 0, set_attack_state::set_attack_state.label("set_attack_state"))
			.add_fixed_timestep_system("my_fixed", 0, set_move_state::set_move_state.label("set_move_state").after("set_attack_state").after("player_casts"))
				.add_fixed_timestep_system("my_fixed", 0, apply_player_state::apply_player_state.label("apply_state").after("set_move_state"))
					.add_fixed_timestep_system("my_fixed", 0, movement_and_collisions::movement_and_collisions.label("move").after("apply_state"))
						.add_fixed_timestep_system("my_fixed", 0, teleport_to_spawn::teleport_to_spawn.after("move"))
						.add_fixed_timestep_system("my_fixed", 0, transfer_data::transfer_data.after("move"))
						.add_fixed_timestep_system("my_fixed", 0, move_camera::move_camera.label("move_camera").after("move").after("player_casts"))
							.add_fixed_timestep_system("my_fixed", 0, screen_shake::screen_shake.after("move_camera"))
					.add_fixed_timestep_system("my_fixed", 0, set_animation_state::set_animation_state.label("set_anim").after("apply_state").after("player_casts"))
						.add_fixed_timestep_system("my_fixed", 0, switch_animation::switch_animation.label("switch_anim").after("set_anim"))
						.add_fixed_timestep_system("my_fixed", 0, time_divisions::time_divisions.label("time").after("set_anim"))
							.add_fixed_timestep_system("my_fixed", 0, animate::animate.label("animate").after("time"))
								.add_fixed_timestep_system("my_fixed", 0, player_deal_damage::player_deal_damage.label("deal_damage").after("animate"))
									.add_fixed_timestep_system("my_fixed", 0, reset_player_input::reset_player_input.after("deal_damage"))
								.add_fixed_timestep_system("my_fixed", 0, audio_test::audio_test.label("audio").after("time"));
	}
}
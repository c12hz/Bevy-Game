use crate::core_game::player::player_structs::Player;
use crate::core_game::player::player_structs::PlayerDamage;
use crate::core_game::player::player_structs::PlayerGraphics;
use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

pub fn screen_shake(
	mut qcamera: Query<&mut Transform, (With<Camera>, Without<Player>, Without<PlayerGraphics>)>,
	qplayer: Query<&PlayerDamage, (With<Player>, Without<PlayerGraphics>, Without<Camera>)>,
	mut shake_counter: Local<u32>,
	mut shake_x: Local<f32>,
	mut shake_y: Local<f32>,
) {
	let shake_power = 0.25;

	for damage in qplayer.iter() {
		for mut transform in qcamera.iter_mut() {
			let mut rng = thread_rng();
			let vec = vec![-1.0, 1.0];

			if damage.applied {
				*shake_counter = 3;
			}

			if *shake_counter == 1 {
				transform.translation.x -= *shake_x;
				transform.translation.y -= *shake_y;
				*shake_counter -= 1;
			}
			if *shake_counter == 2 {
				*shake_counter -= 1;
			}

			if *shake_counter == 3 {
				*shake_x = vec.choose(&mut rng).unwrap() * shake_power;
				*shake_x = (*shake_x * 8.0).round() / 8.0;
				*shake_y = vec.choose(&mut rng).unwrap() * shake_power;
				*shake_y = (*shake_y * 8.0).round() / 8.0;
				transform.translation.x += *shake_x;
				transform.translation.y += *shake_y;
				*shake_counter -= 1;
			}
		}
	}
}

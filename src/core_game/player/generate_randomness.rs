use crate::core_game::player::player_structs::Player;
use crate::core_game::player::player_structs::RandomValues;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub fn generate_randomness(mut query: Query<&mut RandomValues, With<Player>>) {
	let mut rng = thread_rng();

	for mut rvalue in query.iter_mut() {
		rvalue.two = rng.gen_range(1..3);
		rvalue.three = rng.gen_range(1..4);
		rvalue.four = rng.gen_range(1..5);
		rvalue.five = rng.gen_range(1..6);
		rvalue.six = rng.gen_range(1..7);
		rvalue.seven = rng.gen_range(1..8);
		rvalue.eight = rng.gen_range(1..9);
		rvalue.nine = rng.gen_range(1..10);
		rvalue.ten = rng.gen_range(1..11);
		rvalue.eleven = rng.gen_range(1..12);
		rvalue.twelve = rng.gen_range(1..13);
	}
}

use bevy::prelude::*;
use bevy_rapier2d::{
	prelude::{Collider, InteractionGroups, QueryFilter, RapierContext, TOIStatus},
	rapier::prelude::Group,
};
use rand::{thread_rng, Rng};

use crate::core_game::creature::creature_structs::Creature;
use crate::core_game::creature::creature_structs::CreatureMoveState;
use crate::core_game::creature::creature_structs::CreatureState;
use crate::core_game::creature::creature_structs::CreatureStateVariables;

use crate::core_game::player::player_structs::Player;
use crate::core_game::player::player_structs::StealthMode;

pub fn set_creature_state(
	mut query: Query<
		(
			Entity,
			&Collider,
			&Transform,
			&mut CreatureState,
			&mut CreatureStateVariables,
		),
		With<Creature>,
	>,
	query_player: Query<(&Transform, &StealthMode), With<Player>>,
	rapier_context: Res<RapierContext>,
	mut chase_timer: Local<u32>,
) {
	let mut rng = thread_rng();

	let sight_range = 96.0;
	let attack_range = 0.0;
	let chase_max_duration: u32 = 150;

	for (e, collider, transform, mut state, mut var) in query.iter_mut() {
		let patrol_duration: u32 = rng.gen_range(100..=300);
		let idle_duration: u32 = rng.gen_range(50..=150);

		if state.new.0 == CreatureMoveState::Chase && state.old.0 != CreatureMoveState::Chase {
			var.attack_range_offset = rng.gen_range(0.0..=6.0);
		}

		let sight_sensor = rapier_context.cast_shape(
			transform.translation.truncate() - Vec2::new(sight_range, 0.0),
			0.0,
			Vec2::new(2.0 * sight_range, 0.0),
			collider,
			1.0,
			QueryFilter::default().groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2)),
		);

		let attack_sensor = rapier_context.cast_shape(
			transform.translation.truncate() - Vec2::new(attack_range, 0.0),
			0.0,
			Vec2::new(2.0 * attack_range - var.attack_range_offset, 0.0),
			collider,
			1.0,
			QueryFilter::default().groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2)),
		);

		// 'ISOLATED' STATE
		if true {
			var.isolated = false;
		} else {
			var.isolated = true;
		}

		for (transform_player, stealth) in query_player.iter() {
			if sight_sensor.is_none() || stealth.active {
				state.old.0 = state.new.0;
				state.new.0 = CreatureMoveState::Patrol;
			}

			// CHASE

			if transform.translation.x < transform_player.translation.x {
				var.chase_direction = 1.0;
			} else {
				var.chase_direction = -1.0;
			}

			if sight_sensor.is_some() && !stealth.active {
				*chase_timer = 60;
			}

			if *chase_timer > 0 {
				state.old.0 = state.new.0;
				state.new.0 = CreatureMoveState::Chase;
				*chase_timer -= 1;
			}
		}
	}
}

use bevy::prelude::*;
use bevy_rapier2d::{
	prelude::{Collider, InteractionGroups, QueryFilter, RapierContext, TOIStatus},
	rapier::prelude::Group,
};
use rand::{thread_rng, Rng};

use crate::core_game::creature::creature_structs::Creature;
use crate::core_game::creature::creature_structs::CreatureCasts;
use crate::core_game::creature::creature_structs::MoveSpeed;

use super::creature_structs::CreatureStateVariables;


pub fn creature_casts(
	mut query: Query<
		(
			&Collider,
			&Transform,
			&MoveSpeed,
			&mut CreatureCasts,
		),
		With<Creature>,
	>,
	rapier_context: Res<RapierContext>,
) {
	
	for (collider, transform, speed, mut cast) in query.iter_mut() {
		
		
		
		// INITIALIZE VARIABLES
		let mut rng = thread_rng();
		let sight_range = 120.0;
		let chase_range = 120.0;
		let help_range = 300.0;
		let attack_range = 0.0;
		let retreat_range = 58.0;
		let defence_range = 12.0;
		let raycast = 24.0;
		cast.sight_old = cast.sight_new;
		cast.sight_new = false;
    		cast.basic_right = false;
    		cast.basic_left = false;
    		cast.down_right = false;
    		cast.down_left = false;
    		cast.sight_range = false;
		cast.attack_range = false;
    		cast.retreat_range = false;
    		cast.defence_range = false;
    		cast.help_range = false;
    
		
		
		// SIGHT SENSOR
		let sight_sensor = rapier_context.cast_shape(
			transform.translation.truncate() - Vec2::new(sight_range, 0.0),
			0.0,
			Vec2::new(2.0 * sight_range, 0.0),
			collider,
			1.0,
			QueryFilter::default().groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2)),
		);
		if sight_sensor.is_some() {
		cast.sight_new = true;
		cast.sight_range = true
		}
		if cast.sight_new != cast.sight_old {
			cast.attack_offset = rng.gen_range(0.0..=6.0);
		}

		
		
		// ATTACK SENSOR
		let attack_sensor = rapier_context.cast_shape(
			transform.translation.truncate() - Vec2::new(attack_range + cast.attack_offset, 0.0),
			0.0,
			Vec2::new(2.0 * (attack_range + cast.attack_offset), 0.0),
			collider,
			1.0,
			QueryFilter::default()
			.groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2)),
		); 
		if attack_sensor.is_some() {
			cast.attack_range = true;
		}
		
		
		
		// CHASE SENSOR
		let chase_sensor = rapier_context.cast_shape(
			transform.translation.truncate() - Vec2::new(chase_range, 0.0),
			0.0,
			Vec2::new(2.0 * chase_range, 0.0),
			collider,
			1.0,
			QueryFilter::default()
			.groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2)),
		); 
		if chase_sensor.is_some() {
			cast.chase_range = true;
		}
		
		
		
		// HELP SENSOR
		let help_sensor = rapier_context.cast_shape(
			transform.translation.truncate() - Vec2::new(help_range, 0.0),
			0.0,
			Vec2::new(2.0 * help_range, 0.0),
			collider,
			1.0,
			QueryFilter::default()
			.groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2)),
		); 
		if help_sensor.is_some() {
			cast.help_range = true;
		}
		
		
		
		// RETREAT SENSOR
		let retreat_sensor = rapier_context.cast_shape(
			transform.translation.truncate() - Vec2::new(retreat_range, 0.0),
			0.0,
			Vec2::new(2.0 * retreat_range, 0.0),
			collider,
			1.0,
			QueryFilter::default()
			.groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2)),
		); 
		if retreat_sensor.is_some() {
			cast.retreat_range = true;
		}
		
		
		
		// DEFENCE SENSOR
		let defence_sensor = rapier_context.cast_shape(
			transform.translation.truncate() - Vec2::new(defence_range, 0.0),
			0.0,
			Vec2::new(2.0 * defence_range, 0.0),
			collider,
			1.0,
			QueryFilter::default()
			.groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2)),
		); 
		if defence_sensor.is_some() {
			cast.defence_range = true;
		}
		
		
		
		// DOWN RIGHT
		let hit_down_right = rapier_context.cast_shape(
			transform.translation.truncate() + Vec2::new(raycast, 0.0),
			0.0,
			Vec2::new(0.0, -1.0),
			collider,
			1.0,
			QueryFilter::default()
			.groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_1)),
		);
		if hit_down_right.is_some() {
			cast.down_right = true;
		}

		
		
		//DOWN LEFT
		let hit_down_left = rapier_context.cast_shape(
			transform.translation.truncate() + Vec2::new(-raycast, 0.0),
			0.0,
			Vec2::new(0.0, -1.0),
			collider,
			1.0,
			QueryFilter::default()
			.groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_1)),
		);
		if hit_down_left.is_some() {
			cast.down_left = true;
		}

		
		
		// BASIC RIGHT
		let collision_right = rapier_context.cast_shape(
			transform.translation.truncate(),
			0.0,
			Vec2::new(speed.x * 3.0, 0.0),
			collider,
			1.0,
			QueryFilter::default()
			.groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_1)),
		);
		if collision_right.is_some() {
			cast.basic_right = true;
		}
		
		
		
		// BASIC LEFT
		let collision_left = rapier_context.cast_shape(
			transform.translation.truncate(),
			0.0,
			Vec2::new(-speed.x * 3.0, 0.0),
			collider,
			1.0,
			QueryFilter::default()
			.groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_1)),
		);
		if collision_left.is_some() {
			cast.basic_left = true;
		}
		
	}
}
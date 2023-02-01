use bevy::prelude::*;
use bevy_rapier2d::{
	prelude::{Collider, InteractionGroups, QueryFilter, RapierContext, TOIStatus},
	rapier::prelude::Group,
};
use rand::{thread_rng, Rng};

use crate::core_game::creature::creature_structs::Creature;

pub fn creature_casts(
	mut query: Query<
		(
			&Collider,
			&Transform,
		),
		With<Creature>,
	>,
	mut attack_offset: Local<f32>,
	rapier_context: Res<RapierContext>,
) {
    let mut rng = thread_rng();
    let sight_range = 96.0;
    let attack_range = 0.0;
    let chase_max_duration: u32 = 150;
    
    *attack_offset = rng.gen_range(0.0..=6.0);
    
    
    for (collider, transform) in query.iter_mut() {
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
            Vec2::new(2.0 * attack_range - *attack_offset, 0.0),
            collider,
            1.0,
            QueryFilter::default().groups(InteractionGroups::new(Group::GROUP_3, Group::GROUP_2)),
        ); 
    }
}
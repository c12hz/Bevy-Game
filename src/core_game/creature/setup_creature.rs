use std::time::Duration;

use crate::core_game::creature::creature_structs::CreatureSpawnPoint;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn setup_creature(
	mut commands: Commands,
	query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
) {
	for (e, ldtk_entity_info) in query.iter() {
		if ldtk_entity_info.identifier != "Creature1" {
			continue;
		}

		let transform = Transform::from_xyz(
			ldtk_entity_info.px.x as f32,
			ldtk_entity_info.px.y as f32 - 21.0,
			8.0,
		);

		// SET UP PHYSICS ENTITY
		let spawn_point = commands
			.spawn(CreatureSpawnPoint {
				current: None,
				timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
				position: transform,
				min_free_range: 250.0,
			})
			.id();
		//
	}
}

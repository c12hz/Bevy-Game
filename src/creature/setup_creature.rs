use std::time::Duration;

use bevy::prelude::*;
use heron::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::creature::Creature;
use crate::creature::CreatureGraphics;
use crate::creature::MyCreatureAnimations;
use crate::creature::AnimationParams;
use crate::creature::Vel;
use crate::creature::MoveSpeed;
use crate::creature::CreatureMoveState;
use crate::creature::CreatureDirectionState;
use crate::creature::CreatureAnimationState;
use crate::creature::CreatureState;
use crate::creature::CreatureStateVariables;
use crate::creature::CreatureSpawnPoint;
use crate::creature::CreatureStats;
use crate::world::ColliderTypes;

use super::CreatureGraphicsEntity;
use super::CreatureUsefulVariables;




pub fn setup_creature(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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
            .spawn()
            .insert(CreatureSpawnPoint {
                 current: None,
                 timer: Timer::new(Duration::from_secs(2), false),
                 position: transform,
             })
             .id();
        //
    }

}

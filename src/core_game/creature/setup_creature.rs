use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::core_game::creature::creature_structs::Creature;
use crate::core_game::creature::creature_structs::CreatureGraphics;
use crate::core_game::creature::creature_structs::MyCreatureAnimations;
use crate::core_game::creature::creature_structs::AnimationParams;
use crate::core_game::creature::creature_structs::Vel;
use crate::core_game::creature::creature_structs::MoveSpeed;
use crate::core_game::creature::creature_structs::CreatureMoveState;
use crate::core_game::creature::creature_structs::CreatureDirectionState;
use crate::core_game::creature::creature_structs::CreatureAnimationState;
use crate::core_game::creature::creature_structs::CreatureState;
use crate::core_game::creature::creature_structs::CreatureStateVariables;
use crate::core_game::creature::creature_structs::CreatureSpawnPoint;
use crate::core_game::creature::creature_structs::CreatureStats;

use crate::core_game::creature::creature_structs::CreatureGraphicsEntity;
use crate::core_game::creature::creature_structs::CreatureUsefulVariables;




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
            .spawn(CreatureSpawnPoint {
                 current: None,
                 timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
                 position: transform,
             })
             .id();
        //
    }

}

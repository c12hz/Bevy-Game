use bevy::prelude::*;

use crate::core_game::player::player_structs::Camera;
use crate::core_game::player::player_structs::CameraVariables;
use bevy_ecs_ldtk::prelude::*;

pub fn setup_camera(
    mut commands: Commands,
    query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
) {

    for (_e, ldtk_entity_info) in query.iter() {
        if ldtk_entity_info.identifier != "Player" {
            continue;
        }

    // camera setup
        let camera_transform = Transform {
            translation: Vec3::new(ldtk_entity_info.px.x as f32,ldtk_entity_info.px.y as f32, 12.0),
            ..default()
        };
    
        commands.spawn_bundle(Camera2dBundle {
            projection: OrthographicProjection {
            scale: 1.0 / 8.0,
            ..default()
            },
            transform: camera_transform,
            ..default()
        })
        .insert(Camera)
        .insert(CameraVariables {
            new_ground_height: 0.0,
        }
    );
}
}

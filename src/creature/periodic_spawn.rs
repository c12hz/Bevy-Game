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
use crate::creature::CreatureStats;
use crate::world::ColliderTypes;
use crate::creature::CreatureSpawnPoint;

use super::CreatureGraphicsEntity;
use super::CreatureUsefulVariables;


pub fn periodic_spawn(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut query_spawnpoint: Query<&mut CreatureSpawnPoint>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query_creature: Query<Entity, With<Creature>>,
    query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
) {

    

        
        


        for mut spawnpoint in query_spawnpoint.iter_mut() {
            if let Some(e_creature) = spawnpoint.current {
                if query_creature.get(e_creature).is_err() {
                    spawnpoint.current = None;
                    
                }
            }
            if spawnpoint.current.is_none() {
                
                spawnpoint.timer.tick(time.delta());
                if spawnpoint.timer.finished() {

                    spawnpoint.timer.reset();
                    
                    
                    let e_creature = commands.spawn()
                        .insert_bundle(TransformBundle {
                            local: spawnpoint.position,
                            ..Default::default()
                        })
                        .insert(Vel {
                            x: 0.0,
                            y: 0.0,
                            dir: 0.0,
                        })
                        .insert(MoveSpeed {
                            x: 0.25,
                            y: 0.0,
                        })
                        .insert(Creature)
                        .insert(CreatureState {
                            old: (CreatureMoveState::Idle, CreatureDirectionState::None, CreatureAnimationState::Idle),
                            new: (CreatureMoveState::Idle, CreatureDirectionState::None, CreatureAnimationState::Idle),
                        })
                        .insert(CreatureStateVariables {
                            chase_direction: 1.0,
                            patrol_timer: 0,
                            idle_timer: 0,
                            reset_velocity: true,
                            attack_range_offset: 0.0,
                        })
                        .insert(CreatureUsefulVariables {
                            chase_delay: 0,
                            attack_delay: 0,
                        })
                        .insert(CreatureStats{
                            life: 192.0,
                        })
                        .insert(RigidBody::KinematicPositionBased)
                        .insert(CollisionShape::Cuboid { half_extends: Vec3::new(9.0, 5.0, 0.0), border_radius: None})
                        .insert(CollisionLayers::none().with_group(ColliderTypes::Enemy).with_masks(&[ColliderTypes::World, ColliderTypes::Player]))
                        .id();




                    

                    
                    spawnpoint.current = Some(e_creature);
                    if spawnpoint.current.is_some() {
                    }
                    // SET UP GRAPHICS ENTITY

                    let perfect_transitions = true;
                    let texture_handle = asset_server.load("newcreaturetest.png");
                    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 12.0), 1, 1);
                    let texture_atlas_handle = texture_atlases.add(texture_atlas);

                    commands.insert_resource(MyCreatureAnimations {
                        idle: AnimationParams {
                            atlas: texture_atlas_handle.clone(),
                            start: 0,
                            restart: 3,
                            end: 2,
                            perfect_transitions: true,
                        }
                    });


                    // spawn the entity
                    let e_graphics = commands
                        .spawn_bundle(SpriteSheetBundle {
                            texture_atlas: texture_atlas_handle.clone(),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                            visibility: Visibility { is_visible: true },
                            ..Default::default()
                        })
                        .insert(CreatureGraphics)
                        .insert(AnimationParams {
                            atlas: texture_atlas_handle.clone(),
                            start: 0,
                            restart: 3,
                            end: 2,
                            perfect_transitions,
                        })
                        .id();

                    commands.entity(e_creature)
                        .insert(CreatureGraphicsEntity(e_graphics));
                }
            }

        
        }
    }

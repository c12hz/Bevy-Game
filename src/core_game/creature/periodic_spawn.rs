use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group, RigidBody};

use crate::core_game::creature::creature_structs::AnimationParams;
use crate::core_game::creature::creature_structs::Creature;
use crate::core_game::creature::creature_structs::CreatureCasts;
use crate::core_game::creature::creature_structs::CreatureAnimationState;
use crate::core_game::creature::creature_structs::CreatureDirectionState;
use crate::core_game::creature::creature_structs::CreatureGraphics;
use crate::core_game::creature::creature_structs::CreatureMoveState;
use crate::core_game::creature::creature_structs::CreatureState;
use crate::core_game::creature::creature_structs::CreatureStateVariables;
use crate::core_game::creature::creature_structs::CreatureStats;
use crate::core_game::creature::creature_structs::MoveSpeed;
use crate::core_game::creature::creature_structs::MyCreatureAnimations;
use crate::core_game::creature::creature_structs::TimeDivisions;
use crate::core_game::creature::creature_structs::Vel;
use crate::core_game::player::player_structs::Player;

use crate::core_game::creature::creature_structs::CreatureSpawnPoint;

use crate::core_game::creature::creature_structs::CreatureGraphicsEntity;
use crate::core_game::creature::creature_structs::CreatureUsefulVariables;

pub fn periodic_spawn(
	time: Res<Time>,
	asset_server: Res<AssetServer>,
	mut commands: Commands,
	mut query_spawnpoint: Query<&mut CreatureSpawnPoint>,
	mut query_player: Query<&Transform, With<Player>>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	query_creature: Query<Entity, With<Creature>>,
	query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
) {
	for mut spawnpoint in query_spawnpoint.iter_mut() {
		for player_transform in query_player.iter() {
			let mut player_is_far_x = false;
			let mut player_is_far_y = false;
			if (spawnpoint.position.translation.x - player_transform.translation.x).abs()
				> spawnpoint.min_free_range
			{
				player_is_far_x = true;
			}
			if (spawnpoint.position.translation.y - player_transform.translation.y).abs()
				> spawnpoint.min_free_range
			{
				player_is_far_y = true;
			}
			if let Some(e_creature) = spawnpoint.current {
				if query_creature.get(e_creature).is_err() {
					spawnpoint.current = None;
				}
			}
			if spawnpoint.current.is_none() && (player_is_far_x || player_is_far_y) {
				spawnpoint.timer.tick(time.delta());
				if spawnpoint.timer.finished() {
					spawnpoint.timer.reset();

					let e_creature = commands
						.spawn((
							TransformBundle {
								local: spawnpoint.position,
								..Default::default()
							},
							Vel {
								x: 0.0,
								y: 0.0,
								dir: 0.0,
							},
							MoveSpeed { x: 0.125, y: 0.0 },
							(
								Creature,
								CreatureState {
									old: (
										CreatureMoveState::Idle,
										CreatureDirectionState::None,
										CreatureAnimationState::Idle,
									),
									new: (
										CreatureMoveState::Idle,
										CreatureDirectionState::None,
										CreatureAnimationState::Idle,
									),
								},
								CreatureStateVariables {
									chase_direction: 1.0,
									chase_timer: 0,
									patrol_timer: 0,
									idle_timer: 0,
									retreat_timer: 0.0,
									retreating_attack_timer: 0,
									reset_velocity: true,
									attack_range_offset: 0.0,
									isolated: true,
									sprite_direction: 1.0,
									switch: false,
									switch2: false,
									distance_from_player: 0.0
								},
							),
							TimeDivisions {
								two: 0,
								three: 0,
								four: 0,
								five: 0,
								six: 0,
								seven: 0,
								eight: 0,
								nine: 0,
								ten: 0,
								eleven: 0,
								twelve: 0,
								thirteen: 0,
								fourteen: 0,
								fifteen: 0,
								reset: false,
							},
							CreatureUsefulVariables {
								chase_delay: 0,
								attack_delay: 0,
							},
							CreatureStats { life: 500.0 },
							CreatureCasts {
								basic_right: false,
								basic_left: false,
								down_right: false,
								down_left: false,
								sight_range: false,
								sight_new: false,
								sight_old: false,
								chase_range: false,
								attack_range: false,
								attack_offset: 0.0,
								retreat_range: false,
								defence_range: false,
								help_range: false,
							},
							(
								RigidBody::KinematicPositionBased,
								Collider::cuboid(9.0, 5.0),
								CollisionGroups::new(
									Group::GROUP_3,
									Group::GROUP_1 | Group::GROUP_2,
								),
							),
						))
						.id();

					spawnpoint.current = Some(e_creature);
					if spawnpoint.current.is_some() {}
					// SET UP GRAPHICS ENTITY

					let perfect_transitions = true;
					
					let texture_handle_idle = asset_server.load("animations/creature/IcePaukIdle.png");
					let texture_atlas_idle = TextureAtlas::from_grid(texture_handle_idle, Vec2::new(90.0, 90.0), 17, 1, None, None, );
					let texture_atlas_handle_idle = texture_atlases.add(texture_atlas_idle);
					
					let texture_handle_walkf = asset_server.load("animations/creature/IcePaukWalk.png");
					let texture_atlas_walkf = TextureAtlas::from_grid(texture_handle_walkf, Vec2::new(90.0, 90.0), 10, 1, None, None, );
					let texture_atlas_handle_walkf = texture_atlases.add(texture_atlas_walkf);
					
					let texture_handle_atk = asset_server.load("animations/creature/IcePaukOffensiveAttack.png");
					let texture_atlas_atk = TextureAtlas::from_grid(texture_handle_atk, Vec2::new(90.0, 90.0), 11, 1, None, None, );
					let texture_atlas_handle_atk = texture_atlases.add(texture_atlas_atk);
					
					let texture_handle_retreat = asset_server.load("animations/creature/IcePaukRetreat.png");
					let texture_atlas_retreat = TextureAtlas::from_grid(texture_handle_retreat, Vec2::new(90.0, 90.0), 10, 1, None, None, );
					let texture_atlas_handle_retreat = texture_atlases.add(texture_atlas_retreat);
					
					let texture_handle_ranged = asset_server.load("animations/creature/IcePaukRangedAttack.png");
					let texture_atlas_ranged = TextureAtlas::from_grid(texture_handle_ranged, Vec2::new(90.0, 90.0), 15, 1, None, None, );
					let texture_atlas_handle_ranged = texture_atlases.add(texture_atlas_ranged);
					
					let texture_handle_defence = asset_server.load("animations/creature/IcePaukDefence.png");
					let texture_atlas_defence = TextureAtlas::from_grid(texture_handle_defence, Vec2::new(90.0, 90.0), 5, 1, None, None, );
					let texture_atlas_handle_defence = texture_atlases.add(texture_atlas_defence);
					

					commands.insert_resource(MyCreatureAnimations {
						idle: AnimationParams {
							atlas: texture_atlas_handle_idle.clone(),
							start: 0,
							restart: 0,
							end: 17,
							looping: true,
							perfect_transitions: true,
						},
						walkf: AnimationParams {
							atlas: texture_atlas_handle_walkf.clone(),
							start: 0,
							restart: 0,
							end: 10,
							looping: true,
							perfect_transitions: true,
						},
						atk: AnimationParams {
							atlas: texture_atlas_handle_atk.clone(),
							start: 0,
							restart: 0,
							end: 11,
							looping: true,
							perfect_transitions: true,
						},
						retreat: AnimationParams {
							atlas: texture_atlas_handle_retreat.clone(),
							start: 0,
							restart: 0,
							end: 10,
							looping: true,
							perfect_transitions: true,
						},
						ranged: AnimationParams {
							atlas: texture_atlas_handle_ranged.clone(),
							start: 0,
							restart: 0,
							end: 15,
							looping: true,
							perfect_transitions: true,
						},
						defence: AnimationParams {
							atlas: texture_atlas_handle_defence.clone(),
							start: 0,
							restart: 0,
							end: 5,
							looping: false,
							perfect_transitions: true,
						},
					});

					// spawn the entity
					let e_graphics = commands
						.spawn((
							(SpriteSheetBundle {
								texture_atlas: texture_atlas_handle_idle.clone(),
								transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
								visibility: Visibility { is_visible: true },
								..Default::default()
							}),
							(CreatureGraphics),
							(AnimationParams {
								atlas: texture_atlas_handle_idle.clone(),
								start: 0,
								restart: 0,
								end: 0,
								looping: true,
								perfect_transitions,
							}),
							(TimeDivisions {
								two: 0,
								three: 0,
								four: 0,
								five: 0,
								six: 0,
								seven: 0,
								eight: 0,
								nine: 0,
								ten: 0,
								eleven: 0,
								twelve: 0,
								thirteen: 0,
								fourteen: 0,
								fifteen: 0,
								reset: false,
							}),
						))
						.id();

					commands
						.entity(e_creature)
						.insert(CreatureGraphicsEntity(e_graphics));
				}
			}
		}
	}
}

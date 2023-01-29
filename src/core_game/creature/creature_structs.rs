use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Marker that goes on the creature main entity
#[derive(Component)]
pub struct Creature;
/// Marker that goes on the creature sprite entity
#[derive(Component)]
pub struct CreatureGraphics;

/// Extra component for the main entity, to store the id of the associated graphics entity
#[derive(Component)]
pub struct CreatureGraphicsEntity(pub Entity);

#[derive(Component, Clone)]
pub struct Vel {
	pub x: f32,
	pub y: f32,
	pub dir: f32,
}

#[derive(Component, Clone)]
pub struct MoveSpeed {
	pub x: f32,
	pub y: f32,
}

#[derive(Component)]
pub struct CreatureSpawnPoint {
	pub current: Option<Entity>,
	pub timer: Timer,
	pub position: Transform,
	pub min_free_range: f32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum CreatureMoveState {
	Idle,
	Patrol,
	Chase,
	Attack,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum CreatureDirectionState {
	Left,
	Right,
	None,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum CreatureAnimationState {
	Idle,
	Walk,
	Attack,
}

#[derive(Component, Clone)]
pub struct AnimationParams {
	pub atlas: Handle<TextureAtlas>,
	pub start: usize,
	pub restart: usize,
	pub end: usize,
	pub perfect_transitions: bool,
}

#[derive(Resource)]
pub struct MyCreatureAnimations {
	pub idle: AnimationParams,
	pub walkf: AnimationParams,
	pub atk: AnimationParams,
}

#[derive(Component, Clone)]
pub struct TimeDivisions {
	pub two: u32,
	pub three: u32,
	pub four: u32,
	pub five: u32,
	pub six: u32,
	pub seven: u32,
	pub eight: u32,
	pub nine: u32,
	pub ten: u32,
	pub eleven: u32,
	pub twelve: u32,
	pub thirteen: u32,
	pub fourteen: u32,
	pub fifteen: u32,
	pub reset: bool,
}

#[derive(Component, Debug)]
pub struct CreatureState {
	pub old: (
		CreatureMoveState,
		CreatureDirectionState,
		CreatureAnimationState,
	),
	pub new: (
		CreatureMoveState,
		CreatureDirectionState,
		CreatureAnimationState,
	),
}

#[derive(Component)]
pub struct CreatureStateVariables {
	pub chase_direction: f32,
	pub patrol_timer: u32,
	pub idle_timer: u32,
	pub reset_velocity: bool,
	pub attack_range_offset: f32,
	pub isolated: bool,
	pub sprite_direction: f32,
}

#[derive(Component, Clone)]
pub struct CreatureUsefulVariables {
	pub chase_delay: u32,
	pub attack_delay: u32,
}

#[derive(Component, Clone)]
pub struct CreatureStats {
	pub life: f32,
}

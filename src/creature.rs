use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod setup_creature;
pub mod periodic_spawn;
pub mod transfer_data_creature;
pub mod creature_get_damage;
pub mod set_creature_state;
pub mod apply_creature_state;
pub mod creature_movement;
pub mod creature_death;


/// Marker that goes on the creature main entity
#[derive(Component)]
pub struct Creature;
/// Marker that goes on the creature sprite entity
#[derive(Component)]
pub struct CreatureGraphics;

/// Extra component for the main entity, to store the id of the associated graphics entity
#[derive(Component)]
pub struct CreatureGraphicsEntity(Entity);



#[derive(Component, Clone)]
pub struct Vel {
    x: f32,
    y: f32,
    dir: f32,
}

#[derive(Component, Clone)]
pub struct MoveSpeed {
    x: f32,
    y: f32,
}


#[derive(Component)]
pub struct CreatureSpawnPoint {
    current: Option<Entity>,
    timer: Timer,
    position: Transform,
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
    atlas: Handle<TextureAtlas>,
    start: usize,
    restart: usize,
    end: usize,
    perfect_transitions: bool,
}

pub struct MyCreatureAnimations {
    idle: AnimationParams,
}





#[derive(Component, Debug)]
pub struct CreatureState {
    old: (CreatureMoveState, CreatureDirectionState, CreatureAnimationState),
    new: (CreatureMoveState, CreatureDirectionState, CreatureAnimationState),
}


#[derive(Component)]
pub struct CreatureStateVariables {
    chase_direction: f32,
    patrol_timer: u32,
    idle_timer: u32,
    reset_velocity: bool,
    attack_range_offset: f32,
}


#[derive(Component, Clone)]
pub struct CreatureUsefulVariables {
    chase_delay: u32,
    attack_delay: u32,
}

#[derive(Component, Clone)]
pub struct CreatureStats {
    life: f32,
}

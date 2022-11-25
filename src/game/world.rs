use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;


pub mod setup_world;
pub mod spawn_wall_collision;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
}

use bevy::{
    prelude::*,
};
use bevy_ecs_ldtk::prelude::*;

pub mod world_structs;
pub mod setup_world;
pub mod spawn_wall_collision;

use crate::core_game::player;
<<<<<<< Updated upstream
=======
use crate::core_game::creature;
>>>>>>> Stashed changes

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_world::setup_world.label("setup_world"))
            .add_system(player::setup_player::setup_player.after("setup_world"))
<<<<<<< Updated upstream
            //.add_system(setup_creature.label("setup_creature").after("setup_world"))
            //.add_system(periodic_spawn.after("setup_creature"))
=======
            .add_system(creature::setup_creature::setup_creature.label("setup_creature").after("setup_world"))
            .add_system(creature::periodic_spawn::periodic_spawn.after("setup_creature"))
>>>>>>> Stashed changes
            .add_system(player::setup_camera::setup_camera.after("setup_world"))
            .add_system(spawn_wall_collision::spawn_wall_collision)
            .add_system(player::get_player_input::get_player_input)
            .add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_int_cell::<world_structs::WallBundle>(1);
    }
}
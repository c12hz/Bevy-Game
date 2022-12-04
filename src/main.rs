use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
//use iyes_loopless::prelude::*;
//use std::time::Duration;
use bevy::window::PresentMode;
use bevy::render::texture::ImageSettings;
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};

mod core;
mod core_game;

/* 
mod world;
mod player;
mod creature;


use crate::world::setup_world::*;
use crate::world::spawn_wall_collision::*;
use crate::world::WallBundle;


use crate::player::get_player_input::*;
use crate::player::switch_animation::*;
use crate::player::movement_and_collisions::*;
use crate::player::time_divisions::*;
use crate::player::animate::*;
use crate::player::setup_player::*;
use crate::player::setup_camera::*;
use crate::player::move_camera::*;
use crate::player::teleport_to_spawn::*;
use crate::player::transfer_data::*;
use crate::player::set_player_state::*;
use crate::player::set_animation_state::*;
use crate::player::apply_player_state::*;
use crate::player::audio_test::*;
use crate::player::screen_shake::*;
use crate::player::player_deal_damage::*;
use crate::creature::creature_get_damage::*;
use crate::creature::set_creature_state::*;
use crate::creature::apply_creature_state::*;
use crate::creature::creature_movement::*;

use crate::creature::setup_creature::*;
use crate::creature::periodic_spawn::*;
use crate::creature::transfer_data_creature::*;
use crate::creature::creature_death::*;

*/

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(
            WindowDescriptor {
                present_mode: PresentMode::Fifo,
                title: core::GAME_NAME.to_string(),
                resizable: true,
                width: 1920.0,
                height: 1080.0,
                ..Default::default()
            }
        )
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(core::setup::SetupPlugin)
        .add_plugin(core_game::world::WorldPlugin)
        .add_plugin(core_game::player::PlayerPlugin)
        .add_plugin(core_game::creature::CreaturePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<core_game::player::player_structs::PlayerAbilities>()
        .register_inspectable::<core_game::player::player_structs::PlayerWeapons>()
        .run();
}

/*
    let mut fixed_first = SystemStage::parallel();
    fixed_first
    .add_system(set_player_state.label("set_state"))
        .add_system(apply_player_state.label("apply_state").after("set_state"))
            .add_system(movement_and_collisions.label("move").after("apply_state"))
                .add_system(teleport_to_spawn.after("move"))
                .add_system(transfer_data.after("move"))
                .add_system(move_camera.label("move_camera").after("move"))
                    .add_system(screen_shake.after("move_camera"))
            .add_system(set_animation_state.label("set_anim").after("apply_state"))           
                .add_system(switch_animation.label("switch_anim").after("set_anim"))
                    .add_system(time_divisions.label("time").after("set_anim"))
                        .add_system(animate.after("time").label("animate"))
                            .add_system(player_deal_damage.after("animate").label("deal_damage"))
                        //.add_system(audio_test.after("time"))
    .add_system(set_creature_state.label("set_c_state"))
        .add_system(apply_creature_state.label("apply_c_state").after("set_c_state"))
            .add_system(creature_movement.label("move").after("apply_c_state"))
                .add_system(creature_get_damage.label("get_damage").after("deal_damage").after("move"))
                .add_system(transfer_data_creature.after("move"))
                    .add_system(creature_death.after("get_damage"));
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(
            WindowDescriptor {
                present_mode: PresentMode::Mailbox,
                title: "BevyGame".into(),
                resizable: true,
                width: 1920.0,
                height: 1080.0,
                ..Default::default()
            }
        )
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LdtkPlugin)
        //.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(LevelSelection::Index(0))
        .add_stage_before(
            CoreStage::Update,
            "my_fixed_update",
            FixedTimestepStage::new(Duration::from_nanos(16666667))
                .with_stage(fixed_first)
        )
        .add_startup_system(setup_world.label("setup_world"))
        .add_system(setup_player.after("setup_world"))
        .add_system(setup_creature.label("setup_creature").after("setup_world"))
        .add_system(periodic_spawn.after("setup_creature"))
        .add_system(setup_camera.after("setup_world"))
        .add_system(spawn_wall_collision)
        .add_system(get_player_input)
        .register_ldtk_int_cell::<WallBundle>(1)
 */
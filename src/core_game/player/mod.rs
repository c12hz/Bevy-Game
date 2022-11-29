use bevy::{
    utils::Duration,
    prelude::*,
};
use iyes_loopless::prelude::FixedTimestepStage;

pub mod player_structs;
mod animate;
mod apply_player_state;
mod audio_test;
pub mod get_player_input;
mod move_camera;
mod movement_and_collisions;
mod player_deal_damage;
mod screen_shake;
mod set_animation_state;
mod set_player_state;
pub mod setup_player;
pub mod setup_camera;
mod switch_animation;
mod teleport_to_spawn;
mod time_divisions;
mod transfer_data;



pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        let mut fixed_first = SystemStage::parallel();
        fixed_first
        .add_system(set_player_state::set_player_state.label("set_state"))
            .add_system(apply_player_state::apply_player_state.label("apply_state").after("set_state"))
                .add_system(movement_and_collisions::movement_and_collisions.label("move").after("apply_state"))
                    .add_system(teleport_to_spawn::teleport_to_spawn.after("move"))
                    .add_system(transfer_data::transfer_data.after("move"))
                    .add_system(move_camera::move_camera.label("move_camera").after("move"))
                        .add_system(screen_shake::screen_shake.after("move_camera"))
                .add_system(set_animation_state::set_animation_state.label("set_anim").after("apply_state"))           
                    .add_system(switch_animation::switch_animation.label("switch_anim").after("set_anim"))
                        .add_system(time_divisions::time_divisions.label("time").after("set_anim"))
                            .add_system(animate::animate.after("time").label("animate"))
                                .add_system(player_deal_damage::player_deal_damage.after("animate").label("deal_damage"));
                            //.add_system(audio_test.after("time"))

        app.add_stage_before(
            CoreStage::Update,
            "my_fixed_update",
            FixedTimestepStage::new(Duration::from_nanos(16666667))
                .with_stage(fixed_first)
        );
    
    }
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

*/
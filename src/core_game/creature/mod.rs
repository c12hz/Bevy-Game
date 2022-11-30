<<<<<<< Updated upstream
=======
use bevy::{
    utils::Duration,
    prelude::*,
};

use iyes_loopless::prelude::FixedTimestepStage;

pub mod creature_get_damage;
pub mod set_creature_state;
pub mod apply_creature_state;
pub mod creature_movement;

pub mod setup_creature;
pub mod periodic_spawn;
pub mod transfer_data_creature;
pub mod creature_death;
pub mod creature_time_divisions;
pub mod creature_switch_animation;
pub mod animate_creature;
pub mod creature_structs;


>>>>>>> Stashed changes
pub struct CreaturePlugin;

impl Plugin for CreaturePlugin{
    fn build(&self, app: &mut App) {

<<<<<<< Updated upstream
=======
        let mut fixed_second = SystemStage::parallel();
        fixed_second
        .add_system(set_creature_state::set_creature_state.label("set_c_state"))
        .add_system(apply_creature_state::apply_creature_state.label("apply_c_state").after("set_c_state"))
            .add_system(creature_movement::creature_movement.label("move").after("apply_c_state"))
                .add_system(creature_get_damage::creature_get_damage.label("get_damage").after("deal_damage").after("move"))
                .add_system(transfer_data_creature::transfer_data_creature.after("move"))
                    .add_system(creature_death::creature_death.after("get_damage"))
                        .add_system(creature_switch_animation::creature_switch_animation.after("move").label("c_switch_anim"))
                            .add_system(creature_time_divisions::creature_time_divisions.label("c_time").after("c_switch_anim"))
                                .add_system(animate_creature::animate_creature);

        app.add_stage_before(
            CoreStage::Update,
            "my_fixed_update2",
            FixedTimestepStage::new(Duration::from_nanos(16666667))
                .with_stage(fixed_second)
        );
    

>>>>>>> Stashed changes
    }
}
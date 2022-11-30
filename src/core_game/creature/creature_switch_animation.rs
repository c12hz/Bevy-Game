use bevy::{prelude::*};


use crate::core_game::creature::creature_structs::MyCreatureAnimations;
use crate::core_game::creature::creature_structs::CreatureState;

use crate::core_game::creature::creature_structs::Creature;
use crate::core_game::creature::creature_structs::CreatureGraphics;
use crate::core_game::creature::creature_structs::CreatureMoveState;
use crate::core_game::creature::creature_structs::CreatureGraphicsEntity;

pub fn creature_switch_animation(
    anims: Option<Res<MyCreatureAnimations>>,
    graphics_query: Query<Entity, With<CreatureGraphics>>,
    state_query: Query<(&CreatureState, &CreatureGraphicsEntity), With<Creature>>,
    mut commands: Commands,
) {
    if let Some(anims) = anims {
        for (state, cg) in state_query.iter() {
        
        

            if let Ok(e) = graphics_query.get(cg.0) {

                if state.new.0 == CreatureMoveState::Patrol {
                    commands.entity(e).insert(anims.walkf.clone());
                }

                if state.new.0 == CreatureMoveState::Idle {
                    commands.entity(e).insert(anims.idle.clone());
                }

                if state.new.0 == CreatureMoveState::Attack {
                    commands.entity(e).insert(anims.atk.clone());
                }

                if state.new.0 == CreatureMoveState::Chase {
                    commands.entity(e).insert(anims.walkf.clone());
                }


            }
        }
    }   
}

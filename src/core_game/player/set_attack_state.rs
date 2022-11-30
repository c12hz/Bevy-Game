use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{Collider, InteractionGroups, QueryFilter, RapierContext},
    rapier::prelude::Group,
};


use crate::core_game::player::player_structs::PlayerState;
use crate::core_game::player::player_structs::PlayerMoveState;
use crate::core_game::player::player_structs::PlayerDirectionState;
use crate::core_game::player::player_structs::PlayerAttackState;
use crate::core_game::player::player_structs::PlayerStateVariables;
use crate::core_game::player::player_structs::Player;
use crate::core_game::player::player_structs::StealthMode;
use crate::core_game::player::player_structs::PlayerDamage;
use crate::core_game::player::player_structs::PlayerInput;
use crate::core_game::player::player_structs::Ability;
use crate::core_game::player::player_structs::PlayerAbilities;
use crate::core_game::player::player_structs::PlayerWeaponMelee;
use crate::core_game::player::player_structs::PlayerWeaponRanged;
use crate::core_game::player::player_structs::PlayerWeapons;



pub fn set_attack_state (
    mut query: Query<(&Collider, &Transform, &PlayerDamage, &mut PlayerState, &mut PlayerStateVariables, &PlayerInput, &PlayerAbilities, &PlayerWeapons), With<Player>>,
    rapier_context: Res<RapierContext>,
    mut timer_mbh:  Local<u32>,
    mut timer_mbs:  Local<u32>,
    mut timer_rbbf: Local<u32>,
    mut timer_rbbu: Local<u32>,
    mut timer_rbgf: Local<u32>,
    mut timer_rbgu: Local<u32>,
    mut timer_df:   Local<u32>,
    mut timer_dd45: Local<u32>,
) {
    



    for (collider, transform, damage, mut state, mut var, input, ability, weapon) in query.iter_mut() {

        let mut skill = 0;
        let mut abil = ability.ability1;
        let mut attack_timers_active = false;
        let mut up = false;

        if input.pressing_skill1 {
            skill = 1;
            abil = ability.ability1
        }
        if input.pressing_skill2 {
            skill = 2;
            abil = ability.ability2
        }
        if input.pressing_skill3 {
            skill = 3;
            abil = ability.ability3
        }

        if input.pressing_up {
            up = true;
        }

        let frame_length:u32 = 5;

        let mbh:u32 =  5;
        let mbs:u32 =  3;
        let rbb:u32 =  4;
        let rbg:u32 =  3;
        let df:u32  =  2;
        let dd45:u32 = 10;

        if *timer_mbh | *timer_mbs | *timer_rbbf | *timer_rbbu | *timer_rbgf | *timer_rbgu | *timer_df | *timer_dd45 != 0 {
            attack_timers_active = true;
        }



        // SET STATE TIMERS
        if skill != 0 && !attack_timers_active {
            if abil == Ability::MeleeBasic {
                if weapon.melee == PlayerWeaponMelee::Hammer {
                    *timer_mbh = mbh * frame_length;
                }
                if weapon.melee == PlayerWeaponMelee::Sword {
                    *timer_mbs = mbs * frame_length;
                }
            }
            if abil == Ability::RangedBasic && !up {
                if weapon.ranged == PlayerWeaponRanged::Bow {
                    *timer_rbbf = rbb * frame_length;
                }
                if weapon.ranged == PlayerWeaponRanged::Guns {
                    *timer_rbgf = rbg * frame_length;
                }
            }

            if abil == Ability::RangedBasic && up {
                if weapon.ranged == PlayerWeaponRanged::Bow {
                    *timer_rbbu = rbb * frame_length;
                }
                if weapon.ranged == PlayerWeaponRanged::Guns {
                    *timer_rbgu = rbg * frame_length;
                }
            }

            if abil == Ability::Whirlwind && (state.new.0 == PlayerMoveState::Idle || state.new.0 == PlayerMoveState::Run) {
                if weapon.melee == PlayerWeaponMelee::Hammer {
                    state.old.3 = state.new.3;
                    state.new.3 = PlayerAttackState::WhirlwindHammer;
                }
                if weapon.melee == PlayerWeaponMelee::Sword {
                    state.old.3 = state.new.3;
                    state.new.3 = PlayerAttackState::WhirlwindSword;
                }
            }

            if abil == Ability::DashForward {
                *timer_df = df * frame_length;
            }

            if abil == Ability::DashDown45 {
                *timer_dd45 = dd45 * frame_length;
            }
        }


        // SET STATES BASED ON TIMERS

        if *timer_mbh > 0 {
            state.old.3 = state.new.3;
            state.new.3 = PlayerAttackState::MeleeBasicHammer;
        }

        if *timer_mbs > 0 {
            state.old.3 = state.new.3;
            state.new.3 = PlayerAttackState::MeleeBasicSword;
        }

        if *timer_rbbf > 0 {
            state.old.3 = state.new.3;
            state.new.3 = PlayerAttackState::RangedBasicBowForward;
        }

        if *timer_rbbu > 0 {
            state.old.3 = state.new.3;
            state.new.3 = PlayerAttackState::RangedBasicBowUp;
        }

        if *timer_rbgf > 0 {
            state.old.3 = state.new.3;
            state.new.3 = PlayerAttackState::RangedBasicGunsForward;
        }

        if *timer_rbgu > 0 {
            state.old.3 = state.new.3;
            state.new.3 = PlayerAttackState::RangedBasicGunsUp;
        }

        if *timer_df > 0 {
            state.old.3 = state.new.3;
            state.new.3 = PlayerAttackState::DashForward;
        }

        if *timer_dd45 > 0 {
            state.old.3 = state.new.3;
            state.new.3 = PlayerAttackState::DashDown45;
        }

        if !attack_timers_active {
            state.old.3 = state.new.3;
            state.new.3 = PlayerAttackState::None;
        }






        // RESET TIMERS

        if *timer_mbh > 0 {
            *timer_mbh -= 1;
        }

        if *timer_mbs > 0 {
            *timer_mbs -= 1;
        }

        if *timer_rbbf > 0 {
            *timer_rbbf -= 1;
        }

        if *timer_rbbu > 0 {
            *timer_rbbu -= 1;
        }

        if *timer_rbgf > 0 {
            *timer_rbgf -= 1;
        }

        if *timer_rbgu > 0 {
            *timer_rbgu -= 1;
        }

        if *timer_df > 0 {
            *timer_df -= 1;
        }

        if *timer_dd45 > 0 {
            *timer_dd45 -= 1;
        }
    }
}
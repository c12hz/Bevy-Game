use bevy::prelude::*;

use crate::core_game::player::player_structs::TimeDivisions;
use rand::{thread_rng, Rng};

use super::player_structs::{Player, PlayerDamage, PlayerWeapons, PlayerWeaponMelee, PlayerWeaponRanged, PlayerState, PlayerAttackState, PlayerAnimationState, PlayerAbilities};


// careful with volume when you use this function!
// it plays a loud click once every five time steps (every animation frame)
// I use it to test if the time steps work well, and currently tbh they don't..
// ideally the clicks should be evenly spaced at 12hz
// currently there's a lot of stuttering
// not sure how to fix this, might be an issue with bevy itself, or with Iyes Loopless, or maybe I'm just doing something wrong
// I don't mind some tiny bit of stuttering, but currenly it's too much, where even playing footstep sounds
// synced to animation frames will sound off..
// this could also be an issue with my specific PC setup


pub fn audio_test (
    query: Query<(&PlayerDamage, &PlayerState, &TimeDivisions), With<Player>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    let mut rng = thread_rng();
    let choices = [1, 2, 3, 4];

    for (damage, state, time) in query.iter() {
        let random = rng.gen_range(1..5);
        let mut hit = asset_server.load("sound/HammerImpactIce4New.ogg");
        let mut whoosh = asset_server.load("sound/HammerWhoosh1.ogg");
        let mut footstep = asset_server.load("sound/Footstep1.ogg");
        let mut jumpup = asset_server.load("sound/JumpUp3.ogg");
        let mut falldown = asset_server.load("sound/FallDown.ogg");
        //let mut click = asset_server.load("sound/click.ogg");
        

        if random == 1 {
            whoosh = asset_server.load("sound/HammerWhoosh2.ogg");
        }
        if random == 2 {
            whoosh = asset_server.load("sound/HammerWhoosh2.ogg");
        }
        if random == 3 {
            whoosh = asset_server.load("sound/HammerWhoosh2.ogg");
        }
        if random == 4 {
            whoosh = asset_server.load("sound/HammerWhoosh2.ogg");
        }

        if random == 1 {
            hit = asset_server.load("sound/HammerImpactIce4New.ogg");
        }
        if random == 2 {
            hit = asset_server.load("sound/HammerImpactIce4New.ogg");
        }
        if random == 3 {
            hit = asset_server.load("sound/HammerImpactIce4New.ogg");
        }
        if random == 4 {
            hit = asset_server.load("sound/HammerImpactIce4New.ogg");
        }

        if random == 1 {
            footstep = asset_server.load("sound/Footstep1.ogg");
        }
        if random == 2 {
            footstep = asset_server.load("sound/Footstep2.ogg");
        }
        if random == 3 {
            footstep = asset_server.load("sound/Footstep3.ogg");
        }
        if random == 4 {
            footstep = asset_server.load("sound/Footstep4.ogg");
        }



        if state.new.3 == PlayerAttackState::MeleeBasicHammer && state.old.3 != PlayerAttackState::MeleeBasicHammer {
            audio.play_with_settings(
                whoosh,
                PlaybackSettings::ONCE.with_volume(0.2),
            );
        }

        if damage.applied {
            audio.play_with_settings(
                hit,
                PlaybackSettings::ONCE.with_volume(0.6),
            );
        }

        if state.new.2 == PlayerAnimationState::Run && time.five == 1 && time.four == 1 {
            audio.play_with_settings(
                footstep,
                PlaybackSettings::ONCE.with_volume(0.6),
            );
        }

        if state.new.2 == PlayerAnimationState::Jump && state.old.2 != PlayerAnimationState::Jump  {
            audio.play(jumpup);
        }

        if (state.new.2 == PlayerAnimationState::Idle || state.new.2 == PlayerAnimationState::Run) && state.old.2 == PlayerAnimationState::Fall {
            audio.play_with_settings(
                falldown,
                PlaybackSettings::ONCE.with_volume(1.2),
            );
        }

        if time.five == 1 && time.four == 1 {
            //audio.play(click);

        }
    }
}

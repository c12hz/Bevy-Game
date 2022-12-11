use bevy::prelude::*;

use crate::core_game::player::player_structs::TimeDivisions;
use rand::{thread_rng, Rng};

use super::player_structs::{PlayerDamage, PlayerWeapons, PlayerWeaponMelee, PlayerWeaponRanged, PlayerState, PlayerAttackState};


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
    query: Query<(&PlayerDamage, &PlayerState)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    let mut rng = thread_rng();
    let choices = [1, 2, 3, 4];

    for (damage, state) in query.iter() {
        let random = rng.gen_range(1..5);
        let mut hit = asset_server.load("sound/HammerImpactIce4New.ogg");
        let mut whoosh = asset_server.load("sound/HammerWhoosh1.ogg");
        //let mut footstep = asset_server.load("sound/Footstep1.ogg");
        

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
            //footstep = asset_server.load("sound/Footstep1.ogg");
        }
        if random == 2 {
            //footstep = asset_server.load("sound/Footstep2.ogg");
        }
        if random == 3 {
            //footstep = asset_server.load("sound/Footstep3.ogg");
        }
        if random == 4 {
            //footstep = asset_server.load("sound/Footstep4.ogg");
        }



        if state.new.3 == PlayerAttackState::MeleeBasicHammer && state.old.3 != PlayerAttackState::MeleeBasicHammer {
            audio.play(whoosh);
        }

        if damage.applied {
            audio.play(hit);
        }
    }
}

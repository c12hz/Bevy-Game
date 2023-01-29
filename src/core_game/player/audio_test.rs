use bevy::prelude::*;

use crate::core_game::player::player_structs::MyPlayerSounds;
use crate::core_game::player::player_structs::RandomValues;
use crate::core_game::player::player_structs::TimeDivisions;

use super::player_structs::{
	Player, PlayerAbilities, PlayerAnimationState, PlayerAttackState, PlayerDamage, PlayerState,
	PlayerWeaponMelee, PlayerWeaponRanged, PlayerWeapons,
};

// careful with volume when you use this function!
// it plays a loud click once every five time steps (every animation frame)
// I use it to test if the time steps work well, and currently tbh they don't..
// ideally the clicks should be evenly spaced at 12hz
// currently there's a lot of stuttering
// not sure how to fix this, might be an issue with bevy itself, or with Iyes Loopless, or maybe I'm just doing something wrong
// I don't mind some tiny bit of stuttering, but currenly it's too much, where even playing footstep sounds
// synced to animation frames will sound off..
// this could also be an issue with my specific PC setup

pub fn audio_test(
	query: Query<
		(
			&PlayerDamage,
			&PlayerState,
			&TimeDivisions,
			&RandomValues,
			Entity,
		),
		With<Player>,
	>,
	mut clock1: Local<bool>,
	mut clock2: Local<bool>,
	audio: Res<Audio>,
	sounds: Option<Res<MyPlayerSounds>>,
) {
	if let Some(sounds) = sounds {
		for (damage, state, time, rvalue, e) in query.iter() {
			let mut iceimpct = sounds.iceimpct1.handle.clone();
			let mut hmrimpct = sounds.hmrimpct1.handle.clone();
			let mut basshit = sounds.basshit1.handle.clone();
			let mut footstep = sounds.step1.handle.clone();

			if rvalue.two == 1 && *clock1 {
				iceimpct = sounds.iceimpct1.handle.clone();
			}
			if rvalue.two == 2 && *clock1 {
				iceimpct = sounds.iceimpct2.handle.clone();
			}
			if rvalue.two == 1 && !*clock1 {
				iceimpct = sounds.iceimpct3.handle.clone();
			}
			if rvalue.two == 2 && !*clock1 {
				iceimpct = sounds.iceimpct4.handle.clone();
			}

			if rvalue.five == 1 {
				hmrimpct = sounds.hmrimpct1.handle.clone();
			}
			if rvalue.five == 2 {
				hmrimpct = sounds.hmrimpct2.handle.clone();
			}
			if rvalue.five == 3 {
				hmrimpct = sounds.hmrimpct3.handle.clone();
			}
			if rvalue.five == 4 {
				hmrimpct = sounds.hmrimpct4.handle.clone();
			}
			if rvalue.five == 5 {
				hmrimpct = sounds.hmrimpct5.handle.clone();
			}

			if rvalue.two == 1 {
				basshit = sounds.basshit1.handle.clone();
			}
			if rvalue.two == 2 {
				basshit = sounds.basshit2.handle.clone();
			}

			if rvalue.six == 1 && *clock2 {
				footstep = sounds.step1.handle.clone();
			}
			if rvalue.six == 2 && *clock2 {
				footstep = sounds.step2.handle.clone();
			}
			if rvalue.six == 3 && *clock2 {
				footstep = sounds.step3.handle.clone();
			}
			if rvalue.six == 4 && *clock2 {
				footstep = sounds.step4.handle.clone();
			}
			if rvalue.six == 5 && *clock2 {
				footstep = sounds.step5.handle.clone();
			}
			if rvalue.six == 6 && *clock2 {
				footstep = sounds.step6.handle.clone();
			}
			if rvalue.seven == 1 && !*clock2 {
				footstep = sounds.step7.handle.clone();
			}
			if rvalue.seven == 2 && !*clock2 {
				footstep = sounds.step8.handle.clone();
			}
			if rvalue.seven == 3 && !*clock2 {
				footstep = sounds.step9.handle.clone();
			}
			if rvalue.seven == 4 && !*clock2 {
				footstep = sounds.step10.handle.clone();
			}
			if rvalue.seven == 5 && !*clock2 {
				footstep = sounds.step11.handle.clone();
			}
			if rvalue.seven == 6 && !*clock2 {
				footstep = sounds.step12.handle.clone();
			}
			if rvalue.seven == 7 && !*clock2 {
				footstep = sounds.step13.handle.clone();
			}

			if state.new.3 == PlayerAttackState::MeleeBasicHammer
				&& state.old.3 != PlayerAttackState::MeleeBasicHammer
			{}

			if damage.applied {
				*clock1 = !*clock1;
				audio.play_with_settings(iceimpct, PlaybackSettings::ONCE.with_volume(0.4));
				audio.play_with_settings(basshit, PlaybackSettings::ONCE.with_volume(0.8));
				if state.new.3 == PlayerAttackState::MeleeBasicHammer {
					audio.play_with_settings(hmrimpct, PlaybackSettings::ONCE.with_volume(0.4));
				}
			}

			if state.new.2 == PlayerAnimationState::Run && time.five == 1 && time.four == 1 {
				*clock2 = !*clock2;
				audio.play_with_settings(footstep, PlaybackSettings::ONCE.with_volume(0.5));
			}

			if state.new.2 == PlayerAnimationState::Jump
				&& state.old.2 != PlayerAnimationState::Jump
			{}

			if (state.new.2 == PlayerAnimationState::Idle
				|| state.new.2 == PlayerAnimationState::Run)
				&& state.old.2 == PlayerAnimationState::Fall
			{}

			if time.five == 1 && time.four == 1 {
				//audio.play(click);
			}
		}
	}
	//let mut click = asset_server.load("sound/click.ogg");
}

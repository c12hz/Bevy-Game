use crate::core_game::creature::creature_structs::AnimationParams;
use crate::core_game::creature::creature_structs::CreatureGraphics;
use crate::core_game::creature::creature_structs::TimeDivisions;
use bevy::prelude::*;

// this is perhaps a quite unusual way to go about things,
// but this game runs in fixed-timestep of 60fps, and so the various timings
// are not defined in milisecond but by numbers of frames
// for instance all animations in this game run at 12fps, which is 1 every 5 time-steps
// this function creates a multitude of different time divisions that can be used for various timings
// the time divisions provided by this specific function are reset whenever animation changes,
// so they are best suited for visual applications
// for time divisions related to physics stuff like gravity/jump length/etc I'm using custom frame counters dedicated to those

pub fn creature_time_divisions(
	mut query: Query<
		(&mut TimeDivisions, &Handle<TextureAtlas>, &AnimationParams),
		With<CreatureGraphics>,
	>
) {
	for (mut time, handle, params) in query.iter_mut() {
		time.reset = params.atlas != *handle; // resets the timer when animation changes

		if time.reset == true {
			time.two = 0;
			time.three = 0;
			time.four = 0;
			time.five = 0;
			time.six = 0;
			time.seven = 0;
			time.eight = 0;
			time.nine = 0;
			time.ten = 0;
			time.eleven = 0;
			time.twelve = 0;
			time.thirteen = 0;
			time.fourteen = 0;
			time.fifteen = 0;
		}

		time.two += 1;
		time.three += 1;
		time.four += 1;
		time.five += 1;
		time.six += 1;
		time.seven += 1;
		time.eight += 1;
		time.nine += 1;
		time.ten += 1;
		time.eleven += 1;
		time.twelve += 1;
		time.thirteen += 1;
		time.fourteen += 1;
		time.fifteen += 1;

		if time.two >= 2 {
			time.two = 0;
		}

		if time.three >= 3 {
			time.three = 0;
		}

		if time.four >= 4 {
			time.four = 0;
		}

		if time.five >= 5 {
			time.five = 0;
		}

		if time.six >= 6 {
			time.six = 0;
		}

		if time.seven >= 7 {
			time.seven = 0;
		}

		if time.eight >= 8 {
			time.eight = 0;
		}

		if time.nine >= 9 {
			time.nine = 0;
		}

		if time.ten >= 10 {
			time.ten = 0;
		}

		if time.eleven >= 11 {
			time.eleven = 0;
		}

		if time.twelve >= 12 {
			time.twelve = 0;
		}

		if time.thirteen >= 13 {
			time.thirteen = 0;
		}

		if time.fourteen >= 14 {
			time.fourteen = 0;
		}

		if time.fifteen >= 15 {
			time.fifteen = 0;
		}
	}
}

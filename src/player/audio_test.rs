use bevy::prelude::*;

use crate::player::TimeDivisions;


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
    query: Query<&TimeDivisions>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    for time in query.iter() {
        let music = asset_server.load("sound/click.ogg");

        if time.five == 1 {
            audio.play(music);
        }
    }
}

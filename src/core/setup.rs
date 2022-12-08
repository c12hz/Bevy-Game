
use bevy::{
    prelude::*
};

use iyes_loopless::{
    prelude::*,
};

use crate::core::*;

/*
    resources used:
    https://github.com/bevyengine/bevy/blob/latest/examples/window/window_settings.rs

*/

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loopless_state(states::AppState::Loading)
            .add_plugin(fps_plugin::FPSPlugin);
    }   
}

use bevy::prelude::*;

use iyes_loopless::prelude::*;

use crate::core::states;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
	fn build(
		&self,
		app: &mut App,
	) {
		app.add_startup_system(create_resources);
	}
}

pub fn create_resources(
	mut commands: Commands,
	_asset_server: Res<AssetServer>,
) {
	commands.insert_resource(NextState(states::AppState::Loaded));
}

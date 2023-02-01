use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

//test commit
pub fn setup_world(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(LdtkWorldBundle {
		ldtk_handle: asset_server.load("GameV3.ldtk"),
		..Default::default()
	});
}

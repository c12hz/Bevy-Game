use bevy::{
	diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
	prelude::*,
};
use iyes_loopless::prelude::*;

#[derive(Component)]
pub struct FrameCounterTag;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum FrameCounterState {
	Enabled,
	Disabled,
}

pub struct FPSPlugin;

impl Plugin for FPSPlugin {
	fn build(
		&self,
		app: &mut App,
	) {
		app.add_plugin(FrameTimeDiagnosticsPlugin)
			.add_loopless_state(FrameCounterState::Disabled)
			.add_enter_system(FrameCounterState::Enabled, create_fps_counter)
			.add_exit_system(FrameCounterState::Enabled, remove_fps_counter)
			.add_system(fps_counter_system.run_in_state(FrameCounterState::Enabled))
			.add_system(fps_enabler);
	}
}

fn create_fps_counter(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	let txt_style: TextStyle = TextStyle {
		font: asset_server.load("fonts/GravityBold.ttf"),
		font_size: 12.0,
		color: Color::rgb(0.9, 0., 0.),
	};

	commands
		.spawn_bundle(TextBundle {
			text: Text {
				sections: vec![TextSection {
					value: "FPS: ".to_string(),
					style: txt_style,
				}],
				..Default::default()
			},
			..Default::default()
		})
		.insert(FrameCounterTag);
}

fn remove_fps_counter(
	mut commands: Commands,
	mut query: Query<Entity, With<FrameCounterTag>>,
) {
	for entity in query.iter_mut() {
		commands.entity(entity).despawn();
	}
}

fn fps_counter_system(
	mut query: Query<&mut Text, With<FrameCounterTag>>,
	diagnostics: Res<Diagnostics>,
) {
	let fps_diags: Option<f64> = extract_fps(&diagnostics);

	if fps_diags.is_none() {
		return;
	}

	for mut txt in query.iter_mut() {
		let mut fps_str: String = "FPS: ".to_string();
		let frame_cnt = (fps_diags.unwrap() as i64).to_string();
		fps_str.push_str(frame_cnt.as_str());
		txt.sections[0].value = fps_str;
	}
}

fn fps_enabler(
	mut commands: Commands,
	frame_counter_state: Res<CurrentState<FrameCounterState>>,
	keys: Res<Input<KeyCode>>,
) {
	if keys.just_pressed(KeyCode::F5) == false {
		return;
	}

	let new_state: FrameCounterState = if frame_counter_state.0 == FrameCounterState::Enabled {
		FrameCounterState::Disabled
	} else {
		FrameCounterState::Enabled
	};

	commands.insert_resource(NextState(new_state));
}

fn extract_fps(diagnostics: &Res<Diagnostics>) -> Option<f64> {
	diagnostics
		.get(FrameTimeDiagnosticsPlugin::FPS)
		.and_then(|fps| fps.average())
}

use crate::core_game::creature::creature_structs::CreatureGraphics;
use bevy::prelude::*;

pub fn creature_reset_color(
	mut creature_graphics: Query<&mut TextureAtlasSprite, With<CreatureGraphics>>,
) {
	for mut sprite in creature_graphics.iter_mut() {
		sprite.color = Color::Rgba {
			red: 1.0,
			green: 1.0,
			blue: 1.0,
			alpha: 1.0,
		}
	}
}

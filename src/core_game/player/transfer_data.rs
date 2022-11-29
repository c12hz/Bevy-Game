use bevy::prelude::*;

use crate::core_game::player::player_structs::Player;
use crate::core_game::player::player_structs::PlayerGraphics;

pub fn transfer_data(
    qplayer: Query<&Transform, (With<Player>, Without<PlayerGraphics>)>,
    mut qgraphics: Query<(&mut Transform, &TextureAtlasSprite), (With<PlayerGraphics>, Without<Player>)>,
){
    
    // move transform values from physics entity to graphics entity
        // graphics entity is always 'quantized' to multiples of 0.25 (game scale unit)
    for transform in qplayer.iter() {
        for (mut trans, sprite) in qgraphics.iter_mut() {
            trans.translation = (transform.translation * 8.0).round() / 8.0;

            //the function below offsets the graphic entity's vertical position so that the head goes slightly above the collider box
            trans.translation.y = trans.translation.y + 3.0;
            if sprite.index == 3 {
                trans.translation.y = trans.translation.y + 0.25;
            }
            if sprite.index == 4 || sprite.index == 6 {
                trans.translation.y = trans.translation.y + 0.125;
            }
        }
    }   
}

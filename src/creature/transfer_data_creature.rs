use bevy::prelude::*;

use crate::creature::Creature;
use crate::creature::CreatureGraphics;

use super::CreatureGraphicsEntity;



pub fn transfer_data_creature(
    mut commands: Commands,
    qcreature: Query<(Entity, &Transform, &CreatureGraphicsEntity), (With<Creature>, Without<CreatureGraphics>)>,
    mut qgraphics: Query<&mut Transform, (Without<Creature>, With<CreatureGraphics>)>,
){
    
    // move transform values from physics entity to graphics entity
    // graphics entity is always 'quantized' to multiples of 0.25 (game scale unit)
    for (e, transform, cg) in qcreature.iter() {
        if let Ok(mut trans) = qgraphics.get_mut(cg.0) {
            trans.translation = (transform.translation * 4.0).round() / 4.0;

            //the function below offsets the graphic entity's vertical position so that the head goes slightly above the collider box
            trans.translation.y = trans.translation.y;
        } else {
            // there was a problem querying the graphics entity
            commands.entity(e).despawn_recursive();
        }
    }
}

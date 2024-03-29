use bevy::prelude::*;

use crate::creature::Creature;
use crate::creature::CreatureStats;
use crate::creature::CreatureGraphics;
use crate::creature::CreatureGraphicsEntity;



pub fn creature_death (
    mut commands: Commands,
    qcreature: Query<(Entity, &CreatureStats, &CreatureGraphicsEntity), (With<Creature>, Without<CreatureGraphics>)>,
    qgraphics: Query<Entity, (Without<Creature>, With<CreatureGraphics>)>,
){
    // graphics entity is always 'quantized' to multiples of 0.25 (game scale unit)
    for (e_physics, stats, cg) in qcreature.iter() {
        if let Ok(e_graphics) = qgraphics.get(cg.0) {
            if stats.life == 0.0 {
                commands.entity(e_physics).despawn_recursive();
                commands.entity(e_graphics).despawn_recursive();
            }
        }
    }
}



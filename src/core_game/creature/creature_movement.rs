use bevy::prelude::*;

use crate::core_game::creature::creature_structs::Creature;
use crate::core_game::creature::creature_structs::Vel;



pub fn creature_movement (
    mut query: Query<(Entity, &Vel, &mut Transform), With<Creature>>,
) {

    for (e, velocity, mut transform) in query.iter_mut() {
        
        transform.translation.x += velocity.x;
    }
}
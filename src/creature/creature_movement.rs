use bevy::prelude::*;

use crate::creature::Creature;
use crate::creature::Vel;



pub fn creature_movement (
    mut query: Query<(Entity, &Vel, &mut Transform), With<Creature>>,
) {

    for (e, velocity, mut transform) in query.iter_mut() {
        
        transform.translation.x += velocity.x;
    }
}
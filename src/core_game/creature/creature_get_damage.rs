use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::core_game::creature::creature_structs::Creature;
use crate::core_game::creature::creature_structs::CreatureGraphics;

use crate::core_game::player::player_structs::Player;
use crate::core_game::player::player_structs::PlayerDamage;

use crate::core_game::creature::creature_structs::CreatureGraphicsEntity;
use crate::core_game::creature::creature_structs::CreatureStats;

pub fn creature_get_damage (
    player: Query<(&PlayerDamage), With<Player>>,
    mut creature: Query<(&CreatureGraphicsEntity, &mut CreatureStats, &mut Transform),  With<Creature>>,
    mut creature_graphics: Query<(&mut TextureAtlasSprite), With<CreatureGraphics>>,
    mut timer: Local<u32>,
    mut pushback_timer: Local<u32>,

) {

    let pushback_velocity = 1.0;
    let mut rng = thread_rng();

    for damage in player.iter() {
        for target in damage.targets.iter() {
            if let Ok((e_graphics, mut stats, mut transform)) = creature.get_mut(*target) {
                
                if damage.applied && stats.life > 0.0 {
                    stats.life -= damage.value;
                    *pushback_timer = 10;
                }

                if stats.life < 0.0 {
                    stats.life = 0.0;
                }


                if *pushback_timer > 0 {
                    if rng.gen_range(0..9) > 6 {
                        transform.translation.x += pushback_velocity * damage.direction;
                    }
                    *pushback_timer -= 1;
                }



                if let Ok(mut sprite) = creature_graphics.get_mut(e_graphics.0) {
                    // do stuff with the graphics here

                    if damage.dealt {
                        *timer = 5;
                    }
                    if *timer > 0 {
                        sprite.color = Color::Rgba {
                            red: 2.0,
                            green: 2.0,
                            blue: 2.0,
                            alpha: 1.0,
                        }
                    }

                    if *timer > 0 {
                        *timer -= 1;
                    }
                }
            }
        }
    }
}


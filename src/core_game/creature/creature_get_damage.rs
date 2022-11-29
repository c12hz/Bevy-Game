use bevy::prelude::*;

use crate::creature::Creature;
use crate::creature::CreatureGraphics;

use crate::player::Player;
use crate::player::PlayerDamage;

use crate::creature::CreatureGraphicsEntity;
use crate::creature::CreatureStats;

pub fn creature_get_damage (
    player: Query<(&PlayerDamage), With<Player>>,
    mut creature: Query<(&CreatureGraphicsEntity, &mut CreatureStats),  With<Creature>>,
    mut creature_graphics: Query<(&mut TextureAtlasSprite), With<CreatureGraphics>>,
    mut timer: Local<u32>,

) {
    for damage in player.iter() {
        for target in damage.targets.iter() {
            if let Ok((e_graphics, mut stats)) = creature.get_mut(*target) {
                
                if damage.dealt && stats.life > 0.0 {
                    stats.life -= damage.value;
                }

                if stats.life < 0.0 {
                    stats.life = 0.0;
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
                    else {
                        sprite.color = Color::Rgba {
                            red: 1.0,
                            green: 1.0,
                            blue: 1.0,
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


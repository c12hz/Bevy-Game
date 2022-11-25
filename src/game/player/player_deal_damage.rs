use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{Collider, InteractionGroups, QueryFilter, RapierContext},
    rapier::prelude::Group,
};



use crate::player::PlayerState;
use crate::player::PlayerAnimationState;
use crate::player::Player;
use crate::player::PlayerGraphics;
use crate::player::PlayerDamage;
use crate::player::PlayerDamageStats;
use crate::player::DamageKind;




pub fn player_deal_damage (
    mut physical: Query<(&Collider, &PlayerState, &mut PlayerDamage, &PlayerDamageStats), With<Player>>,
    graphical: Query<(&Transform, &TextureAtlasSprite), With<PlayerGraphics>>,
    rapier_context: Res<RapierContext>,
    mut cooldown: Local<u32>,
) {


    for (collider, state, mut damage, stats) in physical.iter_mut() {
        for (transform, sprite) in graphical.iter() {

            let whirl_cast = 20.0;
            let whirl_offset = 12.0;
            let cooldown_timer = 5;

            damage.dealt = false;
            damage.applied = false;

       

            //WHIRLWIND DAMAGE
            
            if state.new.2 == PlayerAnimationState::Whirlwind && *cooldown == 0 {
                let mut targets_right = Vec::new();
                let mut targets_left = Vec::new();
                if sprite.index == 1 || sprite.index == 3 {
                    if (sprite.index == 1 && sprite.flip_x == false) || (sprite.index == 3 && sprite.flip_x == true) {
                        loop {
                            let hit_whirl_right = rapier_context.cast_shape(
                                Vec2::new(transform.translation.x, transform.translation.y) + Vec2::new(whirl_offset, 0.0),
                                0.0,
                                Vec2::new(whirl_cast - whirl_offset, 0.0),
                                collider,
                                1.0,
                                QueryFilter::default()
                                .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1))
                                .predicate(
                                &|e| {
                                    !targets_right.iter().any(|t| *t == e)
                                }),
                            );

                            if let Some((entity,toii)) = hit_whirl_right {
                                targets_right.push(entity);
                            } else {
                                break;
                            }
                        }

                        damage.dealt = true;
                        damage.kind = DamageKind::Whirlwind;
                        damage.kind_mult = stats.whirlwind_mult;
                        *cooldown = cooldown_timer;
                        damage.targets = targets_right;
                    }

                    if (sprite.index == 1 && sprite.flip_x == true) || (sprite.index == 3 && sprite.flip_x == false) {
                        loop {
                            let hit_whirl_left = rapier_context.cast_shape(
                                Vec2::new(transform.translation.x, transform.translation.y) + Vec2::new(-whirl_offset, 0.0),
                                0.0,
                                Vec2::new(-whirl_cast + whirl_offset, 0.0),
                                collider,
                                1.0,
                                QueryFilter::default()
                                .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1))
                                .predicate(
                                &|e| {
                                    !targets_left.iter().any(|t| *t == e)
                                }),
                            );

                            if let Some((entity, toii)) = hit_whirl_left {
                                targets_left.push(entity);
                            } else {
                                break;
                            }
                        }

                        damage.dealt = true;
                        damage.kind = DamageKind::Whirlwind;
                        damage.kind_mult = stats.whirlwind_mult;
                        *cooldown = cooldown_timer;
                        damage.targets = targets_left;
                    }
                }
            }

            // THIS COOLDOWN ENSURES DAMAGE CAN ONLY BE DEALT ONCE PER FRAME OF ANIMATION

            if *cooldown > 0 {
                *cooldown -= 1;
            }



            // CALCULATE FINAL DAMAGE VALUE

            damage.value = damage.weapon_dmg * damage.kind_mult;

            if damage.crit {
                damage.value *= 2.0;
            }


            // IF DAMAGE HIT TARGET SET DAMAGE.APPLIED TO TRUE
            if damage.targets.len() > 0 && damage.dealt {
                damage.applied = true;
            }
            else {
                damage.applied = false;
            }
        }
        
    }
}

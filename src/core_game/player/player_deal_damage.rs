use bevy::prelude::*;
use bevy_rapier2d::{
	prelude::{Collider, InteractionGroups, QueryFilter, RapierContext},
	rapier::prelude::Group,
};

use crate::core_game::player::player_structs::DamageKind;
use crate::core_game::player::player_structs::Player;
use crate::core_game::player::player_structs::PlayerAnimationState;
use crate::core_game::player::player_structs::PlayerDamage;
use crate::core_game::player::player_structs::PlayerDamageStats;
use crate::core_game::player::player_structs::PlayerGraphics;
use crate::core_game::player::player_structs::PlayerState;

pub fn player_deal_damage(
	mut physical: Query<
		(
			&Collider,
			&PlayerState,
			&mut PlayerDamage,
			&PlayerDamageStats,
		),
		With<Player>,
	>,
	graphical: Query<(&Transform, &TextureAtlasSprite), With<PlayerGraphics>>,
	rapier_context: Res<RapierContext>,
	mut cooldown: Local<u32>,
) {
	for (collider, state, mut damage, stats) in physical.iter_mut() {
		for (transform, sprite) in graphical.iter() {
			let whirl_cast = 20.0;
			let whirl_offset = 12.0;
			let mbh_cast = 10.0;
			let mbs_cast = 10.0;
			let rbg_cast = 512.0;
			let mut bsc_atk_offset = 12.0;
			let cooldown_timer = 6;
			let mut hit_frame: usize = 1000;
			let mut flip_value = 0.0;
			let hammer_hit_frame: usize = 1;
			let sword_hit_frame: usize = 1;
			let guns_hit_frame: usize = 0;

			damage.dealt = false;
			damage.applied = false;
			if sprite.flip_x {
				flip_value = -1.0
			} else {
				flip_value = 1.0
			}
			bsc_atk_offset *= flip_value;

			//MELEE BASIC HAMMER DAMAGE

			if state.new.2 == PlayerAnimationState::MeleeBasicHammer && *cooldown == 0 {
				hit_frame = hammer_hit_frame;

				if sprite.index == hit_frame {
					let mut targets = Vec::new();
					loop {
						let hit_mbh = rapier_context.cast_shape(
							transform.translation.truncate() + Vec2::new(bsc_atk_offset, 0.0),
							0.0,
							Vec2::new(mbh_cast * flip_value, 0.0),
							collider,
							1.0,
							QueryFilter::default()
								.groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_3))
								.predicate(&|e| !targets.iter().any(|t| *t == e)),
						);

						if let Some((entity, _toii)) = hit_mbh {
							targets.push(entity);
						} else {
							break;
						}
					}

					damage.dealt = true;
					damage.kind = DamageKind::Simple;
					damage.kind_mult = stats.simple_mult;
					*cooldown = cooldown_timer;
					damage.targets = targets;
					damage.direction = flip_value
				}
			}

			//MELEE BASIC SWORD DAMAGE

			if state.new.2 == PlayerAnimationState::MeleeBasicSword && *cooldown == 0 {
				hit_frame = sword_hit_frame;

				if sprite.index == hit_frame {
					let mut targets = Vec::new();
					loop {
						let hit_mbs = rapier_context.cast_shape(
							transform.translation.truncate() + Vec2::new(bsc_atk_offset, 0.0),
							0.0,
							Vec2::new(mbs_cast * flip_value, 0.0),
							collider,
							1.0,
							QueryFilter::default()
								.groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_3))
								.predicate(&|e| !targets.iter().any(|t| *t == e)),
						);

						if let Some((entity, _toii)) = hit_mbs {
							targets.push(entity);
						} else {
							break;
						}
					}

					damage.dealt = true;
					damage.kind = DamageKind::Simple;
					damage.kind_mult = stats.simple_mult;
					*cooldown = cooldown_timer;
					damage.targets = targets;
					damage.direction = flip_value
				}
			}

			//RANGED BASIC GUNS FORWARD DAMAGE

			if state.new.2 == PlayerAnimationState::RangedBasicGunsForward && *cooldown == 0 {
				hit_frame = guns_hit_frame;

				if sprite.index == hit_frame {
					let mut targets = Vec::new();
					let hit_rbg = rapier_context.cast_shape(
						transform.translation.truncate() + Vec2::new(bsc_atk_offset, 0.0),
						0.0,
						Vec2::new(rbg_cast * flip_value, 0.0),
						collider,
						1.0,
						QueryFilter::default()
							.groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_3))
					);

					if let Some((entity, _toii)) = hit_rbg {
						targets.push(entity);
					}

					damage.dealt = true;
					damage.kind = DamageKind::Simple;
					damage.kind_mult = stats.simple_mult;
					*cooldown = cooldown_timer;
					damage.targets = targets;
					damage.direction = flip_value
				}
			}

			//WHIRLWIND DAMAGE

			if state.new.2 == PlayerAnimationState::WhirlwindHammer && *cooldown == 0 {
				let mut targets_right = Vec::new();
				let mut targets_left = Vec::new();
				if sprite.index == 1 || sprite.index == 3 {
					if (sprite.index == 1 && sprite.flip_x == false)
						|| (sprite.index == 3 && sprite.flip_x == true)
					{
						loop {
							let hit_whirl_right = rapier_context.cast_shape(
								Vec2::new(transform.translation.x, transform.translation.y)
									+ Vec2::new(whirl_offset, 0.0),
								0.0,
								Vec2::new(whirl_cast - whirl_offset, 0.0),
								collider,
								1.0,
								QueryFilter::default()
									.groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_3))
									.predicate(&|e| !targets_right.iter().any(|t| *t == e)),
							);

							if let Some((entity, _toii)) = hit_whirl_right {
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
						damage.direction = 1.0;
					}

					if (sprite.index == 1 && sprite.flip_x == true)
						|| (sprite.index == 3 && sprite.flip_x == false)
					{
						loop {
							let hit_whirl_left = rapier_context.cast_shape(
								Vec2::new(transform.translation.x, transform.translation.y)
									+ Vec2::new(-whirl_offset, 0.0),
								0.0,
								Vec2::new(-whirl_cast + whirl_offset, 0.0),
								collider,
								1.0,
								QueryFilter::default()
									.groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_3))
									.predicate(&|e| !targets_left.iter().any(|t| *t == e)),
							);

							if let Some((entity, _toii)) = hit_whirl_left {
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
						damage.direction = -1.0;
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
			} else {
				damage.applied = false;
			}
		}
	}
}

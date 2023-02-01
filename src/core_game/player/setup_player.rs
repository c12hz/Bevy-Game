use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group, RigidBody};

use crate::core_game::player::player_structs::Ability;
use crate::core_game::player::player_structs::AnimationParams;
use crate::core_game::player::player_structs::DamageKind;
use crate::core_game::player::player_structs::Grav;
use crate::core_game::player::player_structs::MoveSpeed;
use crate::core_game::player::player_structs::MyPlayerAnimations;
use crate::core_game::player::player_structs::MyPlayerSounds;
use crate::core_game::player::player_structs::Player;
use crate::core_game::player::player_structs::PlayerAbilities;
use crate::core_game::player::player_structs::PlayerAnimationState;
use crate::core_game::player::player_structs::PlayerAttackState;
use crate::core_game::player::player_structs::PlayerDamage;
use crate::core_game::player::player_structs::PlayerDamageStats;
use crate::core_game::player::player_structs::PlayerDirectionState;
use crate::core_game::player::player_structs::PlayerGraphics;
use crate::core_game::player::player_structs::PlayerInput;
use crate::core_game::player::player_structs::PlayerMoveState;
use crate::core_game::player::player_structs::PlayerState;
use crate::core_game::player::player_structs::PlayerStateVariables;
use crate::core_game::player::player_structs::PlayerWeaponMelee;
use crate::core_game::player::player_structs::PlayerWeaponRanged;
use crate::core_game::player::player_structs::PlayerWeapons;
use crate::core_game::player::player_structs::RandomValues;
use crate::core_game::player::player_structs::StealthMode;
use crate::core_game::player::player_structs::TimeDivisions;
use crate::core_game::player::player_structs::Vel;
use crate::core_game::player::player_structs::WallKick;
use crate::core_game::player::player_structs::PlayerCasts;


use super::player_structs::SoundParams;

pub fn setup_player(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
) {
	for (e, ldtk_entity_info) in query.iter() {
		if ldtk_entity_info.identifier != "Player" {
			continue;
		}

		// there's a problem: the transform values that LDtk provides are incorrect (entities spawn way above where they should)
		// that's why currently I'm adjusting it manually
		let transform = Transform::from_xyz(
			ldtk_entity_info.px.x as f32,
			ldtk_entity_info.px.y as f32,
			9.0,
		);

		// SET UP PHYSICS ENTITY
		commands.spawn((
			TransformBundle {
				local: transform,
				..Default::default()
			},
			MoveSpeed { x: 1.25, y: 3.0 },
			Vel {
				x: 0.0,
				y: 0.0,
				dir: 0.0,
			},
			Grav {
				speed: 0.0,
				max_speed: 18.0,
				slide_speed: 3.0,
				strength: 0.5, // leave this at 0.5, the actual gravity strength is determined by how often gravity is applied, based on a frame counter.
				counter: 0,
			},
			(
				PlayerInput {
					pressing_jump: false,
					just_pressed_jump: false,
					pressing_left: false,
					just_pressed_left: false,
					pressing_right: false,
					just_pressed_right: false,
					pressing_dodge: false,
					just_pressed_dodge: false,
					pressing_skill1: false,
					just_pressed_skill1: false,
					pressing_skill2: false,
					just_pressed_skill2: false,
					pressing_skill3: false,
					just_pressed_skill3: false,
					pressing_skill4: false,
					just_pressed_skill4: false,
					pressing_up: false,
					just_pressed_up: false,
				},
				PlayerCasts {
					basic_up: false,
					basic_down: false,
					basic_left: false,
					basic_right: false,
					directional_x: false,
					big_left: false,
					big_right: false,
					wallslide_anim_up: false,
					wallslide_anim_down: false,
					enemy_penetration: false,
					nearby_enemies: 0,
				},
				Player,
				PlayerStateVariables {
					jump_frame_counter: 0,
					jumps_remaining: 2,
					runidle_counter: 0,
					idlewhirl_counter: 0,
					whirlidle_counter: 0,
					fallidle_counter: 0,
					walljump_counter: 0,
					dash_counter: 0,
					dash_cooldown: 0,
					dash_strike_counter: 0,
					dash_strike_cooldown: 0,
					actively_colliding: false,
					penetrating_enemy: false,
					sprite_flipped: false,
				},
				PlayerState {
					old: (
						PlayerMoveState::Idle,
						PlayerDirectionState::Right,
						PlayerAnimationState::Idle,
						PlayerAttackState::None,
					),
					new: (
						PlayerMoveState::Idle,
						PlayerDirectionState::Right,
						PlayerAnimationState::Idle,
						PlayerAttackState::None,
					),
				},
				PlayerDamage {
					dealt: false,
					applied: false,
					targets: Vec::new(),
					location: Vec3::new(0.0, 0.0, 0.0),
					kind: DamageKind::Simple,
					kind_mult: 1.0,
					weapon_dmg: 24.0,
					crit: false,
					value: 0.0,
					direction: 1.0,
				},
				PlayerDamageStats {
					hammer_damage: 60.0,
					sword_damage: 48.0,
					bow_damage: 36.0,
					guns_damage: 24.0,
					simple_mult: 1.0,
					whirlwind_mult: 1.2,
					dashstrike_mult: 3.0,
				},
			),
			WallKick {
				timer: 0,
				wall_direction: 0.0,
				full_wallslide: false,
			},
			StealthMode {
				active: false,
				duration: 300,
				counter: 0,
				speed_x: 2.25,
			},
			PlayerWeapons {
				melee: PlayerWeaponMelee::Hammer,
				ranged: PlayerWeaponRanged::Guns,
			},
			PlayerAbilities {
				ability1: Ability::MeleeBasic,
				ability2: Ability::RangedBasic,
				ability3: Ability::DashForward,
			},
			TimeDivisions {
				two: 0,
				three: 0,
				four: 0,
				five: 0,
				six: 0,
				seven: 0,
				eight: 0,
				nine: 0,
				ten: 0,
				eleven: 0,
				twelve: 0,
				thirteen: 0,
				fourteen: 0,
				fifteen: 0,
				reset: false,
			},
			RandomValues {
				one: 0,
				two: 0,
				three: 0,
				four: 0,
				five: 0,
				six: 0,
				seven: 0,
				eight: 0,
				nine: 0,
				ten: 0,
				eleven: 0,
				twelve: 0,
			},
			RigidBody::KinematicPositionBased,
			Collider::cuboid(2.0, 5.0),
			CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_3),
		));
		//.insert(CollisionShape::Cuboid { ..Default::default() });
		// ...

		// SET UP GRAPHICS ENTITY

		// load sprite sheets and all that

		let perfect_transitions = true;

		// PRELOAD SOUNDS
		let ice_impact_1: Handle<AudioSource> = asset_server.load("sound/HammerImpactIce1.ogg");
		let ice_impact_2: Handle<AudioSource> = asset_server.load("sound/HammerImpactIce2.ogg");
		let ice_impact_3: Handle<AudioSource> = asset_server.load("sound/HammerImpactIce3.ogg");
		let ice_impact_4: Handle<AudioSource> = asset_server.load("sound/HammerImpactIce4.ogg");

		let hammer_impact_1: Handle<AudioSource> = asset_server.load("sound/HammerImpact1.ogg");
		let hammer_impact_2: Handle<AudioSource> = asset_server.load("sound/HammerImpact2.ogg");
		let hammer_impact_3: Handle<AudioSource> = asset_server.load("sound/HammerImpact3.ogg");
		let hammer_impact_4: Handle<AudioSource> = asset_server.load("sound/HammerImpact4.ogg");
		let hammer_impact_5: Handle<AudioSource> = asset_server.load("sound/HammerImpact5.ogg");

		let bass_hit_1: Handle<AudioSource> = asset_server.load("sound/ImpactBass1.ogg");
		let bass_hit_2: Handle<AudioSource> = asset_server.load("sound/ImpactBass2.ogg");

		let footstep1: Handle<AudioSource> = asset_server.load("sound/Footstep1.ogg");
		let footstep2: Handle<AudioSource> = asset_server.load("sound/Footstep2.ogg");
		let footstep3: Handle<AudioSource> = asset_server.load("sound/Footstep3.ogg");
		let footstep4: Handle<AudioSource> = asset_server.load("sound/Footstep4.ogg");
		let footstep5: Handle<AudioSource> = asset_server.load("sound/Footstep5.ogg");
		let footstep6: Handle<AudioSource> = asset_server.load("sound/Footstep6.ogg");
		let footstep7: Handle<AudioSource> = asset_server.load("sound/Footstep7.ogg");
		let footstep8: Handle<AudioSource> = asset_server.load("sound/Footstep8.ogg");
		let footstep9: Handle<AudioSource> = asset_server.load("sound/Footstep9.ogg");
		let footstep10: Handle<AudioSource> = asset_server.load("sound/Footstep10.ogg");
		let footstep11: Handle<AudioSource> = asset_server.load("sound/Footstep11.ogg");
		let footstep12: Handle<AudioSource> = asset_server.load("sound/Footstep12.ogg");
		let footstep13: Handle<AudioSource> = asset_server.load("sound/Footstep13.ogg");

		// PRELOAD ANIMATIONS
		let texture_handle = asset_server.load("animations/newrun4c6.png");
		let texture_atlas =
			TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 7, 1, None, None);
		let texture_atlas_handle = texture_atlases.add(texture_atlas);

		let texture_handle_idle = asset_server.load("animations/PlayerFinal.png");
		let texture_atlas_idle =
			TextureAtlas::from_grid(texture_handle_idle, Vec2::new(16.0, 16.0), 1, 1, None, None);
		let texture_atlas_handle_idle = texture_atlases.add(texture_atlas_idle);

		let texture_handle_jump = asset_server.load("animations/jumpUp.png");
		let texture_atlas_jump =
			TextureAtlas::from_grid(texture_handle_jump, Vec2::new(16.0, 16.0), 3, 1, None, None);
		let texture_atlas_handle_jump = texture_atlases.add(texture_atlas_jump);

		let texture_handle_fall = asset_server.load("animations/fallnew.png");
		let texture_atlas_fall =
			TextureAtlas::from_grid(texture_handle_fall, Vec2::new(16.0, 16.0), 3, 1, None, None);
		let texture_atlas_handle_fall = texture_atlases.add(texture_atlas_fall);

		let texture_handle_jumpd = asset_server.load("animations/jumpnew2.png");
		let texture_atlas_jumpd = TextureAtlas::from_grid(
			texture_handle_jumpd,
			Vec2::new(16.0, 16.0),
			3,
			1,
			None,
			None,
		);
		let texture_atlas_handle_jumpd = texture_atlases.add(texture_atlas_jumpd);

		let texture_handle_falld = asset_server.load("animations/fall_directional.png");
		let texture_atlas_falld = TextureAtlas::from_grid(
			texture_handle_falld,
			Vec2::new(36.0, 24.0),
			4,
			1,
			None,
			None,
		);
		let texture_atlas_handle_falld = texture_atlases.add(texture_atlas_falld);

		let texture_handle_slide = asset_server.load("animations/PlayerFinal.png");
		let texture_atlas_slide = TextureAtlas::from_grid(
			texture_handle_slide,
			Vec2::new(16.0, 16.0),
			1,
			1,
			None,
			None,
		);
		let texture_atlas_handle_slide = texture_atlases.add(texture_atlas_slide);

		let texture_handle_whirl = asset_server.load("animations/whirlwind.png");
		let texture_atlas_whirl = TextureAtlas::from_grid(
			texture_handle_whirl,
			Vec2::new(52.0, 24.0),
			4,
			1,
			None,
			None,
		);
		let texture_atlas_handle_whirl = texture_atlases.add(texture_atlas_whirl);

		let texture_handle_runidle = asset_server.load("animations/PlayerFinal.png");
		let texture_atlas_runidle = TextureAtlas::from_grid(
			texture_handle_runidle,
			Vec2::new(16.0, 16.0),
			1,
			1,
			None,
			None,
		);
		let texture_atlas_handle_runidle = texture_atlases.add(texture_atlas_runidle);

		let texture_handle_idlewhirl = asset_server.load("animations/IdleWhirl.png");
		let texture_atlas_idlewhirl = TextureAtlas::from_grid(
			texture_handle_idlewhirl,
			Vec2::new(52.0, 24.0),
			2,
			1,
			None,
			None,
		);
		let texture_atlas_handle_idlewhirl = texture_atlases.add(texture_atlas_idlewhirl);

		let texture_handle_whirlidle = asset_server.load("animations/WhirlIdle.png");
		let texture_atlas_whirlidle = TextureAtlas::from_grid(
			texture_handle_whirlidle,
			Vec2::new(52.0, 29.0),
			2,
			1,
			None,
			None,
		);
		let texture_atlas_handle_whirlidle = texture_atlases.add(texture_atlas_whirlidle);

		let texture_handle_fallidle = asset_server.load("animations/fallidlenew.png");
		let texture_atlas_fallidle = TextureAtlas::from_grid(
			texture_handle_fallidle,
			Vec2::new(16.0, 16.0),
			2,
			1,
			None,
			None,
		);
		let texture_atlas_handle_fallidle = texture_atlases.add(texture_atlas_fallidle);

		let texture_handle_mbs1 = asset_server.load("animations/swordAttack1.png");
		let texture_atlas_mbs1 =
			TextureAtlas::from_grid(texture_handle_mbs1, Vec2::new(64.0, 24.0), 4, 1, None, None);
		let texture_atlas_handle_mbs1 = texture_atlases.add(texture_atlas_mbs1);

		let texture_handle_mbs2 = asset_server.load("animations/swordAttack2.png");
		let texture_atlas_mbs2 =
			TextureAtlas::from_grid(texture_handle_mbs2, Vec2::new(64.0, 24.0), 4, 1, None, None);
		let texture_atlas_handle_mbs2 = texture_atlases.add(texture_atlas_mbs2);

		let texture_handle_mbh1 = asset_server.load("animations/hammerAttack1.png");
		let texture_atlas_mbh1 =
			TextureAtlas::from_grid(texture_handle_mbh1, Vec2::new(64.0, 24.0), 6, 1, None, None);
		let texture_atlas_handle_mbh1 = texture_atlases.add(texture_atlas_mbh1);

		let texture_handle_mbh2 = asset_server.load("animations/hammerAttack2.png");
		let texture_atlas_mbh2 =
			TextureAtlas::from_grid(texture_handle_mbh2, Vec2::new(64.0, 24.0), 6, 1, None, None);
		let texture_atlas_handle_mbh2 = texture_atlases.add(texture_atlas_mbh2);

		let texture_handle_rbbf = asset_server.load("animations/rbbf.png");
		let texture_atlas_rbbf =
			TextureAtlas::from_grid(texture_handle_rbbf, Vec2::new(32.0, 32.0), 4, 1, None, None);
		let texture_atlas_handle_rbbf = texture_atlases.add(texture_atlas_rbbf);

		let texture_handle_rbbu = asset_server.load("animations/rbbu.png");
		let texture_atlas_rbbu =
			TextureAtlas::from_grid(texture_handle_rbbu, Vec2::new(32.0, 32.0), 4, 1, None, None);
		let texture_atlas_handle_rbbu = texture_atlases.add(texture_atlas_rbbu);

		let texture_handle_rbgf1 = asset_server.load("animations/rbgf1.png");
		let texture_atlas_rbgf1 = TextureAtlas::from_grid(
			texture_handle_rbgf1,
			Vec2::new(48.0, 32.0),
			3,
			1,
			None,
			None,
		);
		let texture_atlas_handle_rbgf1 = texture_atlases.add(texture_atlas_rbgf1);

		let texture_handle_rbgf2 = asset_server.load("animations/rbgf2.png");
		let texture_atlas_rbgf2 = TextureAtlas::from_grid(
			texture_handle_rbgf2,
			Vec2::new(48.0, 32.0),
			3,
			1,
			None,
			None,
		);
		let texture_atlas_handle_rbgf2 = texture_atlases.add(texture_atlas_rbgf2);

		let texture_handle_rbgu1 = asset_server.load("animations/rbgu1.png");
		let texture_atlas_rbgu1 = TextureAtlas::from_grid(
			texture_handle_rbgu1,
			Vec2::new(48.0, 32.0),
			3,
			1,
			None,
			None,
		);
		let texture_atlas_handle_rbgu1 = texture_atlases.add(texture_atlas_rbgu1);

		let texture_handle_rbgu2 = asset_server.load("animations/rbgu2.png");
		let texture_atlas_rbgu2 = TextureAtlas::from_grid(
			texture_handle_rbgu2,
			Vec2::new(48.0, 32.0),
			3,
			1,
			None,
			None,
		);
		let texture_atlas_handle_rbgu2 = texture_atlases.add(texture_atlas_rbgu2);

		commands.insert_resource(MyPlayerSounds {
			iceimpct1: SoundParams {
				handle: ice_impact_1.clone(),
				volume: 6.0,
				looping: false,
			},
			iceimpct2: SoundParams {
				handle: ice_impact_2.clone(),
				volume: 6.0,
				looping: false,
			},
			iceimpct3: SoundParams {
				handle: ice_impact_3.clone(),
				volume: 6.0,
				looping: false,
			},
			iceimpct4: SoundParams {
				handle: ice_impact_4.clone(),
				volume: 6.0,
				looping: false,
			},
			hmrimpct1: SoundParams {
				handle: hammer_impact_1.clone(),
				volume: 6.0,
				looping: false,
			},
			hmrimpct2: SoundParams {
				handle: hammer_impact_2.clone(),
				volume: 6.0,
				looping: false,
			},
			hmrimpct3: SoundParams {
				handle: hammer_impact_3.clone(),
				volume: 6.0,
				looping: false,
			},
			hmrimpct4: SoundParams {
				handle: hammer_impact_4.clone(),
				volume: 6.0,
				looping: false,
			},
			hmrimpct5: SoundParams {
				handle: hammer_impact_5.clone(),
				volume: 6.0,
				looping: false,
			},
			basshit1: SoundParams {
				handle: bass_hit_1.clone(),
				volume: 3.0,
				looping: false,
			},
			basshit2: SoundParams {
				handle: bass_hit_2.clone(),
				volume: 3.0,
				looping: false,
			},
			step1: SoundParams {
				handle: footstep1.clone(),
				volume: 8.0,
				looping: false,
			},
			step2: SoundParams {
				handle: footstep2.clone(),
				volume: 8.0,
				looping: false,
			},
			step3: SoundParams {
				handle: footstep3.clone(),
				volume: 8.0,
				looping: false,
			},
			step4: SoundParams {
				handle: footstep4.clone(),
				volume: 8.0,
				looping: false,
			},
			step5: SoundParams {
				handle: footstep5.clone(),
				volume: 8.0,
				looping: false,
			},
			step6: SoundParams {
				handle: footstep6.clone(),
				volume: 8.0,
				looping: false,
			},
			step7: SoundParams {
				handle: footstep7.clone(),
				volume: 8.0,
				looping: false,
			},
			step8: SoundParams {
				handle: footstep8.clone(),
				volume: 8.0,
				looping: false,
			},
			step9: SoundParams {
				handle: footstep9.clone(),
				volume: 8.0,
				looping: false,
			},
			step10: SoundParams {
				handle: footstep10.clone(),
				volume: 8.0,
				looping: false,
			},
			step11: SoundParams {
				handle: footstep11.clone(),
				volume: 8.0,
				looping: false,
			},
			step12: SoundParams {
				handle: footstep12.clone(),
				volume: 8.0,
				looping: false,
			},
			step13: SoundParams {
				handle: footstep13.clone(),
				volume: 8.0,
				looping: false,
			},
		});

		commands.insert_resource(MyPlayerAnimations {
			run: AnimationParams {
				atlas: texture_atlas_handle.clone(),
				start: 0,
				restart: 3,
				end: 7,
				perfect_transitions: false,
			},
			idle: AnimationParams {
				atlas: texture_atlas_handle_idle.clone(),
				start: 0,
				restart: 0,
				end: 1,
				perfect_transitions: false,
			},
			jump: AnimationParams {
				atlas: texture_atlas_handle_jump.clone(),
				start: 0,
				restart: 0,
				end: 3,
				perfect_transitions: false,
			},
			fall: AnimationParams {
				atlas: texture_atlas_handle_fall.clone(),
				start: 0,
				restart: 0,
				end: 3,
				perfect_transitions: false,
			},
			jumpd: AnimationParams {
				atlas: texture_atlas_handle_jumpd.clone(),
				start: 0,
				restart: 0,
				end: 3,
				perfect_transitions: false,
			},
			falld: AnimationParams {
				atlas: texture_atlas_handle_falld.clone(),
				start: 0,
				restart: 0,
				end: 4,
				perfect_transitions: false,
			},
			slide: AnimationParams {
				atlas: texture_atlas_handle_slide.clone(),
				start: 0,
				restart: 0,
				end: 1,
				perfect_transitions: false,
			},
			whirl: AnimationParams {
				atlas: texture_atlas_handle_whirl.clone(),
				start: 0,
				restart: 0,
				end: 4,
				perfect_transitions: false,
			},
			runidle: AnimationParams {
				atlas: texture_atlas_handle_runidle.clone(),
				start: 0,
				restart: 0,
				end: 1,
				perfect_transitions: false,
			},
			idlewhirl: AnimationParams {
				atlas: texture_atlas_handle_idlewhirl.clone(),
				start: 0,
				restart: 0,
				end: 2,
				perfect_transitions: false,
			},
			whirlidle: AnimationParams {
				atlas: texture_atlas_handle_whirlidle.clone(),
				start: 0,
				restart: 0,
				end: 2,
				perfect_transitions: false,
			},
			fallidle: AnimationParams {
				atlas: texture_atlas_handle_fallidle.clone(),
				start: 0,
				restart: 0,
				end: 2,
				perfect_transitions: false,
			},
			mbs1: AnimationParams {
				atlas: texture_atlas_handle_mbs1.clone(),
				start: 1,
				restart: 1,
				end: 4,
				perfect_transitions: false,
			},
			mbs2: AnimationParams {
				atlas: texture_atlas_handle_mbs2.clone(),
				start: 1,
				restart: 1,
				end: 4,
				perfect_transitions: false,
			},
			mbh1: AnimationParams {
				atlas: texture_atlas_handle_mbh1.clone(),
				start: 0,
				restart: 0,
				end: 5,
				perfect_transitions: false,
			},
			mbh2: AnimationParams {
				atlas: texture_atlas_handle_mbh2.clone(),
				start: 0,
				restart: 0,
				end: 5,
				perfect_transitions: false,
			},
			rbbf: AnimationParams {
				atlas: texture_atlas_handle_rbbf.clone(),
				start: 0,
				restart: 0,
				end: 4,
				perfect_transitions: false,
			},
			rbbu: AnimationParams {
				atlas: texture_atlas_handle_rbbu.clone(),
				start: 0,
				restart: 0,
				end: 4,
				perfect_transitions: false,
			},
			rbgf1: AnimationParams {
				atlas: texture_atlas_handle_rbgf1.clone(),
				start: 0,
				restart: 0,
				end: 3,
				perfect_transitions: false,
			},
			rbgf2: AnimationParams {
				atlas: texture_atlas_handle_rbgf2.clone(),
				start: 0,
				restart: 0,
				end: 3,
				perfect_transitions: false,
			},
			rbgu1: AnimationParams {
				atlas: texture_atlas_handle_rbgu1.clone(),
				start: 0,
				restart: 0,
				end: 3,
				perfect_transitions: false,
			},
			rbgu2: AnimationParams {
				atlas: texture_atlas_handle_rbgu2.clone(),
				start: 0,
				restart: 0,
				end: 3,
				perfect_transitions: false,
			},
		});

		// spawn the entity

		commands.spawn((
			SpriteSheetBundle {
				texture_atlas: texture_atlas_handle.clone(),
				transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
				visibility: Visibility { is_visible: true },
				..Default::default()
			},
			PlayerGraphics,
			SoundParams {
				handle: ice_impact_1.clone(),
				volume: 0.6,
				looping: false,
			},
			AnimationParams {
				atlas: texture_atlas_handle.clone(),
				start: 0,
				restart: 0,
				end: 8,
				perfect_transitions,
			},
			PlayerInput {
				pressing_jump: false,
				just_pressed_jump: false,
				pressing_left: false,
				just_pressed_left: false,
				pressing_right: false,
				just_pressed_right: false,
				pressing_dodge: false,
				just_pressed_dodge: false,
				pressing_skill1: false,
				just_pressed_skill1: false,
				pressing_skill2: false,
				just_pressed_skill2: false,
				pressing_skill3: false,
				just_pressed_skill3: false,
				pressing_skill4: false,
				just_pressed_skill4: false,
				pressing_up: false,
				just_pressed_up: false,
			},
			TimeDivisions {
				two: 0,
				three: 0,
				four: 0,
				five: 0,
				six: 0,
				seven: 0,
				eight: 0,
				nine: 0,
				ten: 0,
				eleven: 0,
				twelve: 0,
				thirteen: 0,
				fourteen: 0,
				fifteen: 0,
				reset: false,
			},
		));
	}
}

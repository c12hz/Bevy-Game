use bevy::prelude::*;
<<<<<<< Updated upstream


=======
use bevy_inspector_egui::Inspectable;
>>>>>>> Stashed changes

#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Camera;


#[derive(Component)]
pub struct PlayerGraphics;


#[derive(Component, Clone)]
pub struct Vel {
    pub x: f32,
    pub y: f32,
    pub dir: f32,
}

#[derive(Component, Clone)]
pub struct MoveSpeed {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Grav {
    pub speed: f32,
    pub max_speed: f32,
    pub slide_speed: f32,
    pub strength: f32,
    pub counter: i32,
}


#[derive(Component, Clone)]
pub struct CameraVariables {
    pub new_ground_height: f32,
}

#[derive(Component, Clone)]
pub struct PlayerInput {
    pub pressing_jump: bool,
    pub just_pressed_jump: bool,
    pub pressing_left: bool,
    pub just_pressed_left: bool,
    pub pressing_right: bool,
    pub just_pressed_right: bool,
    pub pressing_dodge: bool,
    pub just_pressed_dodge: bool,
    pub pressing_skill1: bool,
    pub just_pressed_skill1: bool,
    pub pressing_skill2: bool,
    pub just_pressed_skill2: bool,
    pub pressing_skill3: bool,
    pub just_pressed_skill3: bool,
    pub pressing_skill4: bool,
    pub just_pressed_skill4: bool,
<<<<<<< Updated upstream
}


#[derive(Component, Debug, Clone, Copy, PartialEq)]
=======
    pub pressing_up: bool,
    pub just_pressed_up: bool,
}


#[derive(Debug, Clone, Copy, PartialEq)]
>>>>>>> Stashed changes
pub enum PlayerMoveState {
    Idle,
    Run,
    Jump,
    Fall,
    WallSlide,
    Whirlwind,
<<<<<<< Updated upstream
    Dash,
    DashStrike,
=======
    DashForward,
    DashDown45,
>>>>>>> Stashed changes
}



<<<<<<< Updated upstream
#[derive(Component, Debug, Clone, Copy, PartialEq)]
=======
#[derive(Debug, Clone, Copy, PartialEq)]
>>>>>>> Stashed changes
pub enum PlayerDirectionState {
    Left,
    Right,
    None,
}


#[derive(Component, Clone)]
pub struct StealthMode {
    pub active: bool,
    pub duration: u32,
    pub counter: u32,
    pub speed_x: f32,
}


<<<<<<< Updated upstream
#[derive(Component, Debug, Clone, Copy, PartialEq)]
=======
#[derive(Debug, Clone, Copy, PartialEq)]
>>>>>>> Stashed changes
pub enum PlayerAnimationState {
    Idle,
    Run,
    Jump,
    Fall,
    WallSlide,
<<<<<<< Updated upstream
    Whirlwind,
=======
    WhirlwindHammer,
    WhirlwindSword,
>>>>>>> Stashed changes
    RunIdle,
    IdleWhirlwind,
    WhirlwindIdle,
    FallIdle,
<<<<<<< Updated upstream
    SwordHitBasic,
    HammerHitBasic,


=======
    MeleeBasicHammer,
    MeleeBasicSword,
    RangedBasicBowForward,
    RangedBasicBowUp,
    RangedBasicGunsForward,
    RangedBasicGunsUp,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerAttackState {
    MeleeBasicHammer,
    MeleeBasicSword,
    RangedBasicBowForward,
    RangedBasicBowUp,
    RangedBasicGunsForward,
    RangedBasicGunsUp,
    WhirlwindHammer,
    WhirlwindSword,
    DashForward,
    DashDown45,
    None,
>>>>>>> Stashed changes
}


#[derive(Component, Debug)]
pub struct PlayerState {
<<<<<<< Updated upstream
    pub old: (PlayerMoveState, PlayerDirectionState, PlayerAnimationState),
    pub new: (PlayerMoveState, PlayerDirectionState, PlayerAnimationState),
}




// change this to bevy's built in change detection
impl PlayerState {
    fn _changed(&self) -> bool {
        self.old.0 != self.new.0
    }
=======
    pub old: (PlayerMoveState, PlayerDirectionState, PlayerAnimationState, PlayerAttackState),
    pub new: (PlayerMoveState, PlayerDirectionState, PlayerAnimationState, PlayerAttackState),
>>>>>>> Stashed changes
}



// various variables used for determining player state.
// for example a frame counter variable to determine how long a jump state can last
#[derive(Component)]
pub struct PlayerStateVariables {
    pub jump_frame_counter: u32,
    pub jumps_remaining:  u32,
    pub runidle_counter: u32,
    pub idlewhirl_counter: u32,
    pub whirlidle_counter: u32,
    pub fallidle_counter: u32,
    pub walljump_counter: u32,
    pub dash_counter: u32,
    pub dash_cooldown: u32,
    pub dash_strike_counter: u32,
    pub dash_strike_cooldown: u32,
    pub actively_colliding: bool,
    pub penetrating_enemy: bool,
    pub sprite_flipped: bool,
}

#[derive(Component)]
pub struct WallKick {
    pub timer: u32,
    pub wall_direction: f32,
    pub full_wallslide: bool,
}



#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum DamageKind {
    Simple,
    Whirlwind,
    DashStrike,
}

<<<<<<< Updated upstream
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum DamageWeapon {
    Hammer,
    Sword,
    Bow,
    Guns,
}

=======


#[derive(Debug, Clone, Copy, PartialEq, Reflect, Inspectable)]
pub enum PlayerWeaponMelee {
    Hammer,
    Sword,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect, Inspectable)]
pub enum PlayerWeaponRanged {
    Bow,
    Guns,
    None,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect, Inspectable)]
pub struct PlayerWeapons {
    pub melee: PlayerWeaponMelee,
    pub ranged: PlayerWeaponRanged,
}



>>>>>>> Stashed changes
#[derive(Component, Debug, Clone)]
pub struct PlayerDamage {
    pub dealt: bool,
    pub applied: bool,
    pub targets: Vec<Entity>,
    pub location: Vec3,
    pub kind: DamageKind,
<<<<<<< Updated upstream
    pub weapon: DamageWeapon,
=======
>>>>>>> Stashed changes
    pub kind_mult: f32,
    pub weapon_dmg: f32,
    pub crit: bool,
    pub value: f32,
}

#[derive(Component, Clone)]
pub struct PlayerDamageStats {
    pub hammer_damage: f32,
    pub sword_damage: f32,
    pub bow_damage: f32,
    pub guns_damage: f32,
    pub simple_mult: f32,
    pub whirlwind_mult: f32,
    pub dashstrike_mult: f32,
}






#[derive(Component, Clone)]
pub struct AnimationParams {
    pub atlas: Handle<TextureAtlas>,
    pub start: usize,
    pub restart: usize,
    pub end: usize,
    pub perfect_transitions: bool,
}


// the jumpd and falld are for separate forward jump and fall animations that will be added soon
pub struct MyPlayerAnimations {
    pub run: AnimationParams,
    pub idle: AnimationParams,
    pub jump: AnimationParams,
    pub fall: AnimationParams,
    pub jumpd: AnimationParams,
    pub falld: AnimationParams,
    pub slide: AnimationParams,
    pub whirl: AnimationParams,
    pub runidle: AnimationParams,
    pub idlewhirl: AnimationParams,
    pub whirlidle: AnimationParams,
    pub fallidle: AnimationParams,
<<<<<<< Updated upstream
    pub swdatkbsc1: AnimationParams,
    pub swdatkbsc2: AnimationParams,
    pub hmratkbsc1: AnimationParams,
    pub hmratkbsc2: AnimationParams,
=======
    pub mbs1: AnimationParams,
    pub mbs2: AnimationParams,
    pub mbh1: AnimationParams,
    pub mbh2: AnimationParams,
    pub rbbf: AnimationParams,
    pub rbbu: AnimationParams,
    pub rbgf1: AnimationParams,
    pub rbgf2: AnimationParams,
    pub rbgu1: AnimationParams,
    pub rbgu2: AnimationParams,
>>>>>>> Stashed changes
}

#[derive(Component, Clone)]
pub struct TimeDivisions {
    pub two: u32,
    pub three: u32,
    pub four: u32,
    pub five: u32,
    pub six: u32,
    pub seven: u32,
    pub eight: u32,
    pub nine: u32,
    pub ten: u32,
    pub eleven: u32,
    pub twelve: u32,
    pub thirteen: u32,
    pub fourteen: u32,
    pub fifteen: u32,
    pub reset: bool,
}
<<<<<<< Updated upstream
=======


#[derive(Clone, Copy, Debug, Reflect, Inspectable, PartialEq)]
pub enum Ability {
    MeleeBasic,
    RangedBasic,
    Whirlwind,
    DashForward,
    DashDown45,
    Stealth,
    None,
}

#[derive(Component, Debug, Reflect, Inspectable)]
pub struct PlayerAbilities {
    pub ability1: Ability,
    pub ability2: Ability,
    pub ability3: Ability,
}
>>>>>>> Stashed changes

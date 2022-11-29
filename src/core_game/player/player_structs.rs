use bevy::prelude::*;



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
}


#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum PlayerMoveState {
    Idle,
    Run,
    Jump,
    Fall,
    WallSlide,
    Whirlwind,
    Dash,
    DashStrike,
}



#[derive(Component, Debug, Clone, Copy, PartialEq)]
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


#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum PlayerAnimationState {
    Idle,
    Run,
    Jump,
    Fall,
    WallSlide,
    Whirlwind,
    RunIdle,
    IdleWhirlwind,
    WhirlwindIdle,
    FallIdle,
    SwordHitBasic,
    HammerHitBasic,


}


#[derive(Component, Debug)]
pub struct PlayerState {
    pub old: (PlayerMoveState, PlayerDirectionState, PlayerAnimationState),
    pub new: (PlayerMoveState, PlayerDirectionState, PlayerAnimationState),
}




// change this to bevy's built in change detection
impl PlayerState {
    fn _changed(&self) -> bool {
        self.old.0 != self.new.0
    }
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

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum DamageWeapon {
    Hammer,
    Sword,
    Bow,
    Guns,
}

#[derive(Component, Debug, Clone)]
pub struct PlayerDamage {
    pub dealt: bool,
    pub applied: bool,
    pub targets: Vec<Entity>,
    pub location: Vec3,
    pub kind: DamageKind,
    pub weapon: DamageWeapon,
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
    pub swdatkbsc1: AnimationParams,
    pub swdatkbsc2: AnimationParams,
    pub hmratkbsc1: AnimationParams,
    pub hmratkbsc2: AnimationParams,
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

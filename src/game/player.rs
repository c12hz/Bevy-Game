use bevy::prelude::*;


pub mod setup_player;
pub mod get_player_input;
pub mod setup_camera;
pub mod teleport_to_spawn;
pub mod transfer_data;
pub mod move_camera;
pub mod set_player_state;
pub mod set_animation_state;
pub mod apply_player_state;
pub mod switch_animation;
pub mod animate;
pub mod time_divisions;
pub mod movement_and_collisions;
pub mod audio_test;
pub mod player_deal_damage;
pub mod screen_shake;




#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Camera;


#[derive(Component)]
pub struct PlayerGraphics;


#[derive(Component, Clone)]
pub struct Vel {
    x: f32,
    y: f32,
    dir: f32,
}

#[derive(Component, Clone)]
pub struct MoveSpeed {
    x: f32,
    y: f32,
}

#[derive(Component)]
pub struct Grav {
    speed: f32,
    max_speed: f32,
    slide_speed: f32,
    strength: f32,
    counter: i32,
}


#[derive(Component, Clone)]
pub struct CameraVariables {
    new_ground_height: f32,
}

#[derive(Component, Clone)]
pub struct PlayerInput {
    pressing_jump: bool,
    just_pressed_jump: bool,
    pressing_left: bool,
    just_pressed_left: bool,
    pressing_right: bool,
    just_pressed_right: bool,
    pressing_dodge: bool,
    just_pressed_dodge: bool,
    pressing_skill1: bool,
    just_pressed_skill1: bool,
    pressing_skill2: bool,
    just_pressed_skill2: bool,
    pressing_skill3: bool,
    just_pressed_skill3: bool,
    pressing_skill4: bool,
    just_pressed_skill4: bool,
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
    duration: u32,
    counter: u32,
    speed_x: f32,
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
    old: (PlayerMoveState, PlayerDirectionState, PlayerAnimationState),
    new: (PlayerMoveState, PlayerDirectionState, PlayerAnimationState),
}




// change this to bevy's built in change detection
impl PlayerState {
    fn changed(&self) -> bool {
        self.old.0 != self.new.0
    }
}



// various variables used for determining player state.
// for example a frame counter variable to determine how long a jump state can last
#[derive(Component)]
pub struct PlayerStateVariables {
    jump_frame_counter: u32,
    jumps_remaining:  u32,
    runidle_counter: u32,
    idlewhirl_counter: u32,
    whirlidle_counter: u32,
    fallidle_counter: u32,
    walljump_counter: u32,
    dash_counter: u32,
    dash_cooldown: u32,
    dash_strike_counter: u32,
    dash_strike_cooldown: u32,
    actively_colliding: bool,
    penetrating_enemy: bool,
    sprite_flipped: bool,
}

#[derive(Component)]
pub struct WallKick {
    timer: u32,
    wall_direction: f32,
    full_wallslide: bool,
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
    location: Vec3,
    kind: DamageKind,
    weapon: DamageWeapon,
    kind_mult: f32,
    weapon_dmg: f32,
    crit: bool,
    pub value: f32,
}

#[derive(Component, Clone)]
pub struct PlayerDamageStats {
    hammer_damage: f32,
    sword_damage: f32,
    bow_damage: f32,
    guns_damage: f32,
    simple_mult: f32,
    whirlwind_mult: f32,
    dashstrike_mult: f32,
}






#[derive(Component, Clone)]
pub struct AnimationParams {
    atlas: Handle<TextureAtlas>,
    start: usize,
    restart: usize,
    end: usize,
    perfect_transitions: bool,
}


// the jumpd and falld are for separate forward jump and fall animations that will be added soon
pub struct MyPlayerAnimations {
    run: AnimationParams,
    idle: AnimationParams,
    jump: AnimationParams,
    fall: AnimationParams,
    jumpd: AnimationParams,
    falld: AnimationParams,
    slide: AnimationParams,
    whirl: AnimationParams,
    runidle: AnimationParams,
    idlewhirl: AnimationParams,
    whirlidle: AnimationParams,
    fallidle: AnimationParams,
    swdatkbsc1: AnimationParams,
    swdatkbsc2: AnimationParams,
    hmratkbsc1: AnimationParams,
    hmratkbsc2: AnimationParams,
}

#[derive(Component, Clone)]
pub struct TimeDivisions {
    two: u32,
    three: u32,
    four: u32,
    five: u32,
    six: u32,
    seven: u32,
    eight: u32,
    nine: u32,
    ten: u32,
    eleven: u32,
    twelve: u32,
    thirteen: u32,
    fourteen: u32,
    fifteen: u32,
    reset: bool,
}

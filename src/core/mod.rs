pub mod setup;
pub mod states;
pub mod fps_plugin;
pub mod resources;

pub mod prelude {
    pub use crate::core::states;
}

pub const GAME_NAME: &str = "GAME";
pub const VERSION: f32 = 0.1;
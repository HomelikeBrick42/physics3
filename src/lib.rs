mod circle;
mod state;
mod vector2;

pub use circle::*;
pub use state::*;
pub use vector2::*;

#[macro_export]
macro_rules! number { ($($a:tt)+) => { ::fixed_macro::types::I64F64!($($a)+) } }
pub type Number = fixed::types::I64F64;

pub const WINDOW_WIDTH: usize = 640;
pub const WINDOW_HEIGHT: usize = 480;

pub const MAX_PHYSICS_ITERATIONS: usize = 128;
pub const TIME_STEP: Number = number!(0.01);
pub const MAX_TIME_STEPS_PER_FRAME: usize = 32;

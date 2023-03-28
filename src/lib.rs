mod circle;
mod state;
mod vector2;

pub use circle::*;
pub use state::*;
pub use vector2::*;

pub use fixed_macro::types::I64F64 as number;
pub type Number = fixed::types::I64F64;

pub const TIME_STEP: Number = number!(0.001);

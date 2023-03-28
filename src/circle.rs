use crate::{number, Number, Vector2};

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub mass: Number,
    pub radius: Number,
}

impl Circle {
    pub fn get_energy(&self) -> Number {
        number!(0.5) * self.mass * self.velocity.sqr_length()
    }
}

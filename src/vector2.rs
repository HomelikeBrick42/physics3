use fixed_sqrt::FixedSqrt;

use crate::Number;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector2 {
    pub x: Number,
    pub y: Number,
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2 {
        x: Number::ZERO,
        y: Number::ZERO,
    };

    pub fn dot(self, other: Vector2) -> Number {
        self.x * other.x + self.y * other.y
    }

    pub fn sqr_length(self) -> Number {
        self.dot(self)
    }

    pub fn length(self) -> Number {
        self.sqr_length().sqrt()
    }

    pub fn normalized(self) -> Vector2 {
        let length = self.length();
        if length > Number::ZERO {
            self / length
        } else {
            Vector2::ZERO
        }
    }
}

impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add<Number> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Number) -> Self::Output {
        Vector2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl std::ops::Add<Vector2> for Number {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self + rhs.x,
            y: self + rhs.y,
        }
    }
}

impl std::ops::AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        *self = *self + rhs;
    }
}

impl std::ops::AddAssign<Number> for Vector2 {
    fn add_assign(&mut self, rhs: Number) {
        *self = *self + rhs;
    }
}

impl std::iter::Sum for Vector2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = Vector2::ZERO;
        for value in iter {
            result += value;
        }
        result
    }
}

impl std::ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Sub<Number> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Number) -> Self::Output {
        Vector2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl std::ops::Sub<Vector2> for Number {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self - rhs.x,
            y: self - rhs.y,
        }
    }
}

impl std::ops::SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        *self = *self - rhs;
    }
}

impl std::ops::SubAssign<Number> for Vector2 {
    fn sub_assign(&mut self, rhs: Number) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl std::ops::Mul<Number> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Number) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<Vector2> for Number {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::MulAssign<Vector2> for Vector2 {
    fn mul_assign(&mut self, rhs: Vector2) {
        *self = *self * rhs;
    }
}

impl std::ops::MulAssign<Number> for Vector2 {
    fn mul_assign(&mut self, rhs: Number) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl std::ops::Div<Number> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Number) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Div<Vector2> for Number {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}

impl std::ops::DivAssign<Vector2> for Vector2 {
    fn div_assign(&mut self, rhs: Vector2) {
        *self = *self / rhs;
    }
}

impl std::ops::DivAssign<Number> for Vector2 {
    fn div_assign(&mut self, rhs: Number) {
        *self = *self / rhs;
    }
}

impl std::ops::Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

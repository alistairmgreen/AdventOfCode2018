use std::ops::{Add, AddAssign, Div, DivAssign, Sub, SubAssign, Mul, MulAssign};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}

impl Vector2D {
    pub fn abs_square(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Vector2D) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, other: Vector2D) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<i32> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: i32) -> Vector2D {
        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<i32> for Vector2D {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<i32> for Vector2D {
    type Output = Vector2D;

    fn div(self, rhs: i32) -> Vector2D {
        Vector2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<i32> for Vector2D {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
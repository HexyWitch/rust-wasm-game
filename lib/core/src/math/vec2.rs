use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::convert::Into;

#[derive(Clone, Copy, Debug)]
pub struct Vec2(pub f32, pub f32);

#[allow(dead_code)]
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2(x, y)
    }
    pub fn zero() -> Vec2 {
        Vec2(0.0, 0.0)
    }
    pub fn with_angle(angle: f32) -> Vec2 {
        Vec2(angle.cos(), angle.sin())
    }

    pub fn angle(&self) -> f32 {
        self.1.atan2(self.0)
    }
    pub fn mag(&self) -> f32 {
        (self.0.powf(2.0) + self.1.powf(2.0)).sqrt()
    }
    pub fn mag_squared(&self) -> f32 {
        self.0.powf(2.0) + self.1.powf(2.0)
    }
    pub fn normalized(&self) -> Vec2 {
        self.clone() / self.mag()
    }
}

impl Into<(f32, f32)> for Vec2 {
    fn into(self) -> (f32, f32) {
        (self.0, self.1)
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Self) -> Self {
        Vec2(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Self {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl Div for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: Self) -> Self {
        Vec2(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Self {
        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

use super::*;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
#[derive(Component,Default)]
#[require(Transform)]
pub struct InstantVelocity{
    velocity: Vec2
}
impl InstantVelocity{
    pub fn new(velocity: Vec2)->Self{
        Self{velocity}
    }
    pub fn consume(&mut self)->Vec2{
        let output = self.velocity;
        self.velocity = Vec2::ZERO;
        output
    }
}
impl Add<Vec2> for InstantVelocity{
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output {
        Self{velocity: self.velocity + rhs}
    }
}
impl AddAssign<Vec2> for InstantVelocity{
    fn add_assign(&mut self, rhs: Vec2) {
        self.velocity += rhs
    }
}
impl Sub<Vec2> for InstantVelocity{
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self{velocity: self.velocity - rhs}
    }
}
impl SubAssign<Vec2> for InstantVelocity{
    fn sub_assign(&mut self, rhs: Vec2) {
        self.velocity -= rhs
    }
}
impl Mul<Vec2> for InstantVelocity{
    type Output = Self;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Self{velocity: self.velocity * rhs}
    }
}
impl MulAssign<Vec2> for InstantVelocity{
    fn mul_assign(&mut self, rhs: Vec2){
        self.velocity *= rhs
    }
}
impl Mul<f32> for InstantVelocity{
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self{velocity: self.velocity * rhs}
    }
}
impl MulAssign<f32> for InstantVelocity{
    fn mul_assign(&mut self, rhs: f32){
        self.velocity *= rhs
    }
}


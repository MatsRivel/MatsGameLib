use super::*;
use crate::utils::vec2_derivations::vec2_wrapper;

vec2_wrapper!(InstandVelocity);
#[derive(Component,Default)]
#[require(Transform)]
pub struct InstantVelocity(Vec2);
impl InstantVelocity{
    pub fn new(velocity: Vec2)->Self{
        Self(velocity)
    }
    pub fn consume(&mut self)->Vec2{
        let output = self.0;
        self.0 = Vec2::ZERO;
        output
    }
}


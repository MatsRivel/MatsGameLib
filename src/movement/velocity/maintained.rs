
use crate::utils::vec2_derivations::vec2_wrapper;

use super::*;
vec2_wrapper!(MaintainedVelocity);
#[derive(Component,Default)]
#[require(Transform)]
pub struct MaintainedVelocity(Vec2);
impl MaintainedVelocity{
    pub fn new(velocity: Vec2)->Self{
        Self(velocity)
    }
    pub fn consume(&mut self)->Vec2{
        let output = self.0;
        self.0 = Vec2::ZERO;
        output
    }
}

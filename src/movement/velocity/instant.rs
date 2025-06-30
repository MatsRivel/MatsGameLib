use super::*;

#[derive(Component,Default,Vec2Ops)]
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


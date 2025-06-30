use super::*;
#[derive(Component,Default,Vec2Ops)]
#[require(Transform)]
pub struct MaintainedVelocity(Vec2);
impl MaintainedVelocity{
    pub fn new(velocity: Vec2)->Self{
        Self(velocity)
    }
    pub fn get(&mut self)->Vec2{
        self.0
    }
}

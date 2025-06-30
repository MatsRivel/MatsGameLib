use super::*;

#[derive(Component,Default,f32Ops)]
#[require(Transform)]
pub struct MaintainedAngularVelocity(f32);
impl MaintainedAngularVelocity{
    pub fn new(velocity: f32)->Self{
        Self(velocity)
    }
    pub fn consume(&mut self)->f32{
        let output = self.0;
        self.0 = 0.0;
        output
    }
}


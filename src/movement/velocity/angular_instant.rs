use super::*;
#[derive(Component,Default, Debug, Clone, Copy, f32Ops)]
#[require(Transform)]
pub struct InstantAngularVelocity(f32);
impl InstantAngularVelocity{
    pub fn new(velocity: f32)->Self{
        Self(velocity)
    }
    pub fn consume(&mut self)->f32{
        let output = self.0;
        self.0 = 0.0;
        output
    }
}


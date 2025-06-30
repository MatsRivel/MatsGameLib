
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

impl AddAssign<&mut MaintainedVelocity> for Transform{
    fn add_assign(&mut self, rhs: &mut MaintainedVelocity) {
        self.translation += rhs.get().extend(0.0);
    }
}

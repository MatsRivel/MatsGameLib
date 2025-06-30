use super::*;

#[derive(Component,Default, Debug, Clone, Copy, Vec2Ops, PartialEq)]
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

impl AddAssign<&mut InstantVelocity> for Transform{
    fn add_assign(&mut self, rhs: &mut InstantVelocity) {
        self.translation += rhs.consume().extend(0.0);
    }
}
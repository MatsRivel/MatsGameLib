
use super::*;
#[derive(Component,Default, Debug, Clone, Copy, PartialEq, Vec2Ops)]
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

impl_vec2_add_assign!(MaintainedVelocity,MaintainedVelocity);
impl_vec2_add!(MaintainedVelocity,MaintainedVelocity,MaintainedVelocity);

#[cfg(test)]
pub mod maintainted_velocity_tests{
    use super::*;
    #[test]
    pub fn check_add_assign(){
        let mut a = MaintainedVelocity::new(Vec2::ZERO);
        let b = MaintainedVelocity::new(Vec2::ONE);
        let expected = MaintainedVelocity::new(Vec2::ONE);
        a += b;
        assert_eq!(expected,a)
    }
    #[test]
    pub fn check_add(){
        let a = MaintainedVelocity::new(Vec2::ZERO);
        let b = MaintainedVelocity::new(Vec2::ONE);
        let expected = MaintainedVelocity::new(Vec2::ONE);
        assert_eq!(expected,a+b)
    }
}
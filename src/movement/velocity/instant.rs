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
impl Add<&mut InstantVelocity> for Transform{
    type Output = Transform;
    
    fn add(mut self, rhs: &mut InstantVelocity) -> Self::Output {
        self.translation += rhs.consume().extend(0.0);
        self
    }
}
impl_vec2_add_assign!(InstantVelocity,InstantVelocity);
impl_vec2_add!(InstantVelocity,InstantVelocity,InstantVelocity);

#[cfg(test)]
pub mod tests{
    use super::*;
    #[test]
    pub fn check_add_assign_self(){
        let mut a = InstantVelocity::new(Vec2::ZERO);
        let b = InstantVelocity::new(Vec2::ONE);
        let expected = InstantVelocity::new(Vec2::ONE);
        a += b;
        assert_eq!(expected,a)
    }
    #[test]
    pub fn check_add_self(){
        let a = InstantVelocity::new(Vec2::ZERO);
        let b = InstantVelocity::new(Vec2::ONE);
        let expected = InstantVelocity::new(Vec2::ONE);
        assert_eq!(expected,a+b)
    }
    #[test]
    pub fn check_add_assign_other(){
        let mut a = Transform::from_translation(Vec3::ZERO);
        let mut b = InstantVelocity::new(Vec2::ONE);
        let expected = Transform::from_translation(Vec3::new(1.0, 1.0, 0.0));
        a += &mut b;
        assert_eq!(expected,a)
    }
    #[test]
    pub fn check_add_other(){
        let a = Transform::from_translation(Vec3::ZERO);
        let mut b = InstantVelocity::new(Vec2::ONE);
        let expected = Transform::from_translation(Vec3::new(1.0, 1.0, 0.0));
        assert_eq!(expected,a+&mut b)
    }
}
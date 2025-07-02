use super::*;
#[derive(Component,Default, Debug, Clone, Copy, PartialEq, f32Ops)]
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

impl_f32_add_assign!(InstantAngularVelocity,InstantAngularVelocity);
impl_f32_add!(InstantAngularVelocity,InstantAngularVelocity,InstantAngularVelocity);

#[cfg(test)]
pub mod instant_angular_acceleration_tests{
    use super::*;
    #[test]
    pub fn check_add_assign(){
        let mut a = InstantAngularVelocity::new(0.0);
        let b = InstantAngularVelocity::new(1.0);
        let expected = InstantAngularVelocity::new(1.0);
        a += b;
        assert_eq!(expected,a)
    }
    #[test]
    pub fn check_add(){
        let a = InstantAngularVelocity::new(0.0);
        let b = InstantAngularVelocity::new(1.0);
        let expected = InstantAngularVelocity::new(1.0);
        assert_eq!(expected,a+b)
    }
}
use bevy::math::f32;

use super::*;
use crate::movement::velocity::{angular_instant::InstantAngularVelocity, angular_maintained::MaintainedAngularVelocity};
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, f32Ops)]
#[require(InstantAngularVelocity)]
pub struct InstantAngularAcceleration(f32);
impl InstantAngularAcceleration{
    pub fn new(acceleration: f32)->Self{
        Self(acceleration)
    }
    pub fn consume(&mut self)->f32{
        let output = self.0;
        self.0 = 0.0;
        output
    }
    #[allow(unused)]
    pub fn limit(&mut self, limit: f32){
        let acc = self.0;
        if acc > limit{
            self.0 = limit;
        }
    }
    pub fn clear(&mut self){
        self.0 = 0.0;
    }
}
// Macro impls that allow f!(A,B) -> AddAssign B to A, but not vice versa. Both must impl Deref<f32>
impl_f32_add_assign!(MaintainedAngularVelocity,InstantAngularAcceleration);
impl_f32_add_assign!(InstantAngularAcceleration,InstantAngularAcceleration);
// Macro impls that allow f!(A,B) -> Add B to A, but not vice versa. Both must impl Deref<f32>
impl_f32_add!(InstantAngularAcceleration,InstantAngularAcceleration,InstantAngularAcceleration);
impl_f32_add!(MaintainedAngularVelocity,InstantAngularAcceleration,MaintainedAngularVelocity);
#[cfg(test)]
pub mod tests{
    use super::*;
    #[test]
    pub fn check_add_assign_self(){
        let mut a = InstantAngularAcceleration::new(0.0);
        let b = InstantAngularAcceleration::new(1.0);
        let expected = InstantAngularAcceleration::new(1.0);
        a += b;
        assert_eq!(expected,a)
    }
        #[test]
    pub fn check_add_assign_other(){
        let mut a = MaintainedAngularVelocity::new(0.0);
        let b = InstantAngularAcceleration::new(1.0);
        let expected = MaintainedAngularVelocity::new(1.0);
        a += b;
        assert_eq!(expected,a)
    }
    #[test]
    pub fn check_add_self(){
        let a = InstantAngularAcceleration::new(0.0);
        let b = InstantAngularAcceleration::new(1.0);
        let expected = InstantAngularAcceleration::new(1.0);
        assert_eq!(expected,a+b)
    }
    pub fn check_add_other(){
        let a = MaintainedAngularVelocity::new(0.0);
        let b = InstantAngularAcceleration::new(1.0);
        let expected = MaintainedAngularVelocity::new(1.0);
        assert_eq!(expected,a+b)
    }
}
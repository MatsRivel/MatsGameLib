use bevy::math::f32;

use super::*;
use crate::movement::velocity::angular_instant::InstantAngularVelocity;
#[derive(Component, Default, Debug, Clone, Copy, f32Ops)]
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
impl_f32_add_assign!(InstantAngularVelocity,InstantAngularAcceleration);
impl_f32_add_assign!(InstantAngularAcceleration,InstantAngularAcceleration);
// Macro impls that allow f!(A,B) -> Add B to A, but not vice versa. Both must impl Deref<f32>
impl_f32_add!(InstantAngularVelocity,InstantAngularAcceleration,InstantAngularVelocity);
impl_f32_add!(InstantAngularVelocity,InstantAngularVelocity,InstantAngularVelocity);
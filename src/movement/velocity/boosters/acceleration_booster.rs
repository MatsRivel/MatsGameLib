use crate::movement::acceleration::instant::InstantAcceleration;

use super::*;

#[derive(Component,Debug,Clone,Copy,f32Deref)]
pub struct InstantAccelerationBooster(f32);
impl InstantAccelerationBooster{
    pub fn new(boost:f32)->Self{
        Self(boost)
    }
}

impl_vec2_f32_mul!(InstantAcceleration,InstantAccelerationBooster,InstantAcceleration);
impl_vec2_f32_mul_assign!(InstantAcceleration,InstantAccelerationBooster);

#[cfg(test)]
pub mod booster_tests{
    use super::*;
    #[test]
    pub fn mul_boost(){
        let v1 = InstantAcceleration::new(Vec2::ONE);
        let boost = InstantAccelerationBooster::new(2.0);
        let v2 = v1 * boost;
        let expected = Vec2::new(2.0,2.0);
        assert_eq!(expected,*v2);
    }

    #[test]
    pub fn mul_assign_boost(){
        let mut v = InstantAcceleration::new(Vec2::ONE);
        let boost = InstantAccelerationBooster::new(2.0);
        v *= boost;
        let expected = Vec2::new(2.0,2.0);
        assert_eq!(expected,*v);
    }
}
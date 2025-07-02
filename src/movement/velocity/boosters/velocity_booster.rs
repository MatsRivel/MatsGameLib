use crate::movement::velocity::instant::InstantVelocity;

use super::*;

#[derive(Component,Debug,Clone,Copy,f32Deref)]
pub struct InstantVelocityBooster(f32);
impl InstantVelocityBooster{
    pub fn new(boost:f32)->Self{
        Self(boost)
    }
}

impl_vec2_f32_mul!(InstantVelocity,InstantVelocityBooster,InstantVelocity);
impl_vec2_f32_mul_assign!(InstantVelocity,InstantVelocityBooster);

#[cfg(test)]
pub mod booster_tests{
    use super::*;
    #[test]
    pub fn mul_boost(){
        let v1 = InstantVelocity::new(Vec2::ONE);
        let boost = InstantVelocityBooster::new(2.0);
        let v2 = v1 * boost;
        let expected = Vec2::new(2.0,2.0);
        assert_eq!(expected,*v2);
    }

    #[test]
    pub fn mul_assign_boost(){
        let mut v = InstantVelocity::new(Vec2::ONE);
        let boost = InstantVelocityBooster::new(2.0);
        v *= boost;
        let expected = Vec2::new(2.0,2.0);
        assert_eq!(expected,*v);
    }
}
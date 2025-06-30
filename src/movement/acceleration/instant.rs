use super::*;
use crate::movement::velocity::instant::InstantVelocity;
#[derive(Component, Default, Debug, Clone, Copy, Vec2Ops, PartialEq)]
#[require(InstantVelocity)]
pub struct InstantAcceleration(Vec2);
impl InstantAcceleration{
    pub fn new(acceleration: Vec2)->Self{
        Self(acceleration)
    }
    pub fn consume(&mut self)->Vec2{
        let output = self.0;
        self.0 = Vec2::ZERO;
        output
    }
    #[allow(unused)]
    pub fn limit(&mut self, limit: f32){
        let acc = self.0.length();
        if acc > limit{
            if self.0 == Vec2::ZERO { return;}
            self.0 = self.0.normalize() * limit;
        }
    }
    pub fn clear(&mut self){
        self.0 += Vec2::ZERO;
    }
}
// Macro impls that allow f!(A,B) -> AddAssign B to A, but not vice versa. Both must impl Deref<Vec2>
impl_vec2_add_assign!(InstantVelocity,InstantAcceleration);
impl_vec2_add_assign!(InstantVelocity,InstantVelocity);
// Macro impls that allow f!(A,B) -> Add B to A, but not vice versa. Both must impl Deref<Vec2>
impl_vec2_add!(InstantVelocity,InstantAcceleration,InstantVelocity);
impl_vec2_add!(InstantVelocity,InstantVelocity,InstantVelocity);


#[cfg(test)]
pub mod tests{
    use super::*;
    #[test]
    pub fn check_add_assign_1(){
        let mut a = InstantVelocity::new(Vec2::ZERO);
        let b = InstantAcceleration::new(Vec2::ONE);
        let expected = InstantVelocity::new(Vec2::ONE);
        a += b;
        assert_eq!(expected,a)
    }
    #[test]
    pub fn check_add_assign_2(){
        let mut a = InstantVelocity::new(Vec2::ZERO);
        let b = InstantVelocity::new(Vec2::ONE);
        let expected = InstantVelocity::new(Vec2::ONE);
        a += b;
        assert_eq!(expected,a)
    }
    #[test]
    pub fn check_add_1(){
        let a = InstantVelocity::new(Vec2::ZERO);
        let b = InstantAcceleration::new(Vec2::ONE);
        let expected = InstantVelocity::new(Vec2::ONE);
        assert_eq!(expected,a+b)
    }
    #[test]
    pub fn check_add_2(){
        let a = InstantVelocity::new(Vec2::ZERO);
        let b = InstantVelocity::new(Vec2::ONE);
        let expected = InstantVelocity::new(Vec2::ONE);
        assert_eq!(expected,a+b)
    }
}
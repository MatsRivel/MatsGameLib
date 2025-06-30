use super::*;
#[derive(Component, Default, Debug, Clone, Copy, Vec2Ops, PartialEq)]
#[require(MaintainedVelocity)]
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
impl_vec2_add_assign!(MaintainedVelocity,InstantAcceleration);

// Macro impls that allow f!(A,B) -> Add B to A, but not vice versa. Both must impl Deref<Vec2>
impl_vec2_add!(MaintainedVelocity,InstantAcceleration,MaintainedVelocity);



#[cfg(test)]
pub mod tests{
    use super::*;
    #[test]
    pub fn check_add_assign(){
        let mut a = MaintainedVelocity::new(Vec2::ZERO);
        let b = InstantAcceleration::new(Vec2::ONE);
        let expected = MaintainedVelocity::new(Vec2::ONE);
        a += b;
        assert_eq!(expected,a)
    }

    #[test]
    pub fn check_add(){
        let a = MaintainedVelocity::new(Vec2::ZERO);
        let b = InstantAcceleration::new(Vec2::ONE);
        let expected = MaintainedVelocity::new(Vec2::ONE);
        assert_eq!(expected,a+b)
    }
}
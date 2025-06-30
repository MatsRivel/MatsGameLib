use super::*;

#[derive(Component,Default, Debug, Clone, Copy, PartialEq, f32Ops)]
#[require(Transform)]
pub struct MaintainedAngularVelocity(f32);
impl MaintainedAngularVelocity{
    pub fn new(velocity: f32)->Self{
        Self(velocity)
    }
    pub fn consume(&mut self)->f32{
        let output = self.0;
        self.0 = 0.0;
        output
    }
}
impl_f32_add_assign!(MaintainedAngularVelocity,MaintainedAngularVelocity);
impl_f32_add!(MaintainedAngularVelocity,MaintainedAngularVelocity,MaintainedAngularVelocity);
#[cfg(test)]
pub mod tests{
    use super::*;
    #[test]
    pub fn check_add_assign(){
        let mut a = MaintainedAngularVelocity::new(0.0);
        let b = MaintainedAngularVelocity::new(1.0);
        let expected = MaintainedAngularVelocity::new(1.0);
        a += b;
        assert_eq!(expected,a)
    }
    #[test]
    pub fn check_add(){
        let a = MaintainedAngularVelocity::new(0.0);
        let b = MaintainedAngularVelocity::new(1.0);
        let expected = MaintainedAngularVelocity::new(1.0);
        assert_eq!(expected,a+b)
    }
}
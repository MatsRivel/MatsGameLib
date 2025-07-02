use super::*;

#[derive(Default,Debug,Clone,Copy,PartialEq, PartialOrd)]
pub struct Damage(f32);
impl Damage{
    pub fn new(damage:f32)->Self{
        Self(damage)
    }
    pub fn consume(self)->f32{
        self.0
    }
}
impl Deref for Damage{
    type Target=f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<f32> for Damage{
    fn from(value: f32) -> Self {
        Self(value)
    }
}
impl_f32_add_assign!(Damage,Damage);
impl_f32_add!(Damage,Damage,Damage);

#[cfg(test)]
pub mod damage_tests{
    use super::*;
    #[test]
    pub fn add_damage(){
        let a = Damage::new(1.0);
        let b = Damage::new(2.0);
        let c = a+b;
        let expected = 3.0;
        assert_eq!(expected,*c);
    }
    #[test]
    pub fn add_assign_damage(){
        let mut a = Damage::new(1.0);
        a += Damage::new(2.0);
        let expected = 3.0;
        assert_eq!(expected,*a);
    }
}
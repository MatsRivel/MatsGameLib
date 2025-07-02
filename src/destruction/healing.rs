use super::*;

#[derive(Default,Debug,Clone,Copy,PartialEq, PartialOrd)]
pub struct Healing(f32);
impl Healing{
    pub fn new(healing: f32)->Self{
        Self(healing)
    }
}
impl Deref for Healing{
    type Target=f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<f32> for Healing{
    fn from(value: f32) -> Self {
        Self(value)
    }
}
impl_f32_add_assign!(Healing,Healing);
impl_f32_add!(Healing,Healing,Healing);

#[cfg(test)]
pub mod healing_tests{
    use super::*;
    #[test]
    pub fn add_healing(){
        let a = Healing::new(1.0);
        let b = Healing::new(2.0);
        let c = a+b;
        let expected = 3.0;
        assert_eq!(expected,*c);
    }
    #[test]
    pub fn add_assign_healing(){
        let mut a = Healing::new(1.0);
        a += Healing::new(2.0);
        let expected = 3.0;
        assert_eq!(expected,*a);
    }
}
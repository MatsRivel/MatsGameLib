use super::*;

#[derive(Default,Debug,Clone,Copy,PartialEq, PartialOrd)]
pub struct Damage(f32);
impl Damage{
    pub fn new(damage:f32)->Self{
        Self(-damage)
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
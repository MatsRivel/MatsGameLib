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


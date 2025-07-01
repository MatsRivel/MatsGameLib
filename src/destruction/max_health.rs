use super::*;

#[derive(Debug,Clone,Copy,PartialEq, PartialOrd)]
pub struct MaxHealth(f32);
impl MaxHealth{
    pub fn new(health:f32)->Self{
        Self(health)
    }
    pub fn get(&self)->f32{
        self.0
    }
}
impl Deref for MaxHealth{
    type Target=f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Default for MaxHealth{
    fn default() -> Self {
        Self(100.0)
    }
}
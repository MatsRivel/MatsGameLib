use super::*;

#[derive(Debug,Clone,Copy,PartialEq, PartialOrd)]
pub struct CurrentHealth(f32);
impl CurrentHealth{
    pub fn limit(&mut self, upper_bound: MaxHealth){
        self.0 = self.0.min(upper_bound.get());
    }
    pub fn new(health:f32)->Self{
        Self(health)
    }
}
impl Default for CurrentHealth{
    fn default() -> Self {
        Self(100.0)
    }
}
impl Deref for CurrentHealth{
    type Target=f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<f32> for CurrentHealth{
    fn from(value: f32) -> Self {
        Self(value)
    }
}
impl_f32_add_assign!(CurrentHealth,Healing);
impl_f32_add!(CurrentHealth,Healing,CurrentHealth);

impl Add<Damage> for CurrentHealth{
    type Output = CurrentHealth;

    fn add(mut self, rhs: Damage) -> Self::Output {
        self += rhs;
        self
    }
}
impl AddAssign<Damage> for CurrentHealth{
    fn add_assign(&mut self, rhs: Damage) {
        self.0 -= rhs.consume();
    }
}
use super::*;
#[derive(Component)]
pub struct Health{
    max_health: MaxHealth,
    current_health: CurrentHealth
}
impl Health{
    pub fn new(health:f32)->Self{
        Self { max_health: MaxHealth::new(health), current_health: CurrentHealth::new(health) }
    }
    pub fn new_injured(max_health:f32,current_health:f32)->Self{
        Self { max_health: MaxHealth::new(max_health), current_health: CurrentHealth::new(current_health) }
    }

    pub fn is_alive(&self)->bool{
        self.current_health != CurrentHealth::new(0.0)
    }
}
impl Default for Health{
    fn default() -> Self {
        Self::new(100.0)
    }
}
impl AddAssign<Healing> for Health{
    fn add_assign(&mut self, rhs: Healing) {
        self.current_health += rhs;
        self.current_health.limit(self.max_health);

    }
}
impl Add<Healing> for Health{
    type Output = Health;

    fn add(mut self, rhs: Healing) -> Self::Output {
        self.current_health += rhs;
        self.current_health.limit(self.max_health);
        self
    }
}

impl AddAssign<Damage> for Health{
    fn add_assign(&mut self, rhs: Damage) {
        self.current_health += rhs;

    }
}
impl Add<Damage> for Health{
    type Output = Health;

    fn add(mut self, rhs: Damage) -> Self::Output {
        self.current_health += rhs;
        self
    }
}

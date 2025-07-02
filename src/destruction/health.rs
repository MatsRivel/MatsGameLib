use super::*;
#[derive(Component,Clone,Copy,Debug, PartialEq)]
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
        // self.current_health.limit(self.max_health);

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

#[cfg(test)]
pub mod health_tests{
    pub use super::*;
    pub mod adding_damage{
        pub use super::*;
        #[test]
        pub fn add_damage(){
            let hp = Health::new(100.0);
            let damage = Damage::new(1.0);
            let hp_after_dmg = hp + damage;
            let expected_hp: f32 = 99.0;
            
            assert_eq!(expected_hp, *hp_after_dmg.current_health )
        }
        #[test]
        pub fn add_assign_damage(){
            let mut hp = Health::new(100.0);
            let damage = Damage::new(1.0);
            hp += damage;
            let expected_hp: f32 = 99.0;

            assert_eq!(expected_hp, *hp.current_health )
        }
    }
    pub mod adding_healing{
        pub use super::*;
        #[test]
        pub fn add_healing(){
            let hp = Health::new_injured(100.0, 99.0);
            let heal = Healing::new(1.0);
            let hp_after_dmg = hp + heal;
            let expected_hp: f32 = 100.0;

            assert_eq!(expected_hp, *hp_after_dmg.current_health )
        }
        #[test]
        pub fn add_assign_healing(){
            let mut hp = Health::new_injured(100.0, 99.0);
            let healing = Healing::new(1.0);
            hp += healing;
            let expected_hp: f32 = 100.0;

            assert_eq!(expected_hp, *hp.current_health )
        }
        #[test]
        pub fn add_and_assign_healing_equivalency(){
            let mut hp1 = Health::new_injured(100.0, 99.0);
            let healing = Healing::new(1.0);
            hp1 += healing;

            let hp2_base = Health::new_injured(100.0, 99.0);
            let hp2 = hp2_base + healing;

            assert_eq!(hp1, hp2);
        }
    }
}
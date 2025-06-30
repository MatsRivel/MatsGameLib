use crate::movement::velocity::instant::InstantVelocity;
use crate::utils::vec2_derivations::vec2_wrapper;
use super::*;

vec2_wrapper!(InstantAcceleration);
#[derive(Component, Default, Debug, Clone, Copy)]
#[require(InstantVelocity)]
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

pub fn apply_linear_acceleration(time: Res<Time>, mut query: Query<(&mut InstantAcceleration, &mut InstantAcceleration)>){
    for (mut vel, mut acc) in query.iter_mut(){
        *acc *= time.delta_secs();
        *vel += *acc;
        acc.clear();
    }
}
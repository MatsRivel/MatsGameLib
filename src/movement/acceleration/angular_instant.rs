use bevy::math::f32;

use super::*;
use crate::movement::velocity::instant::InstantAngularVelocity;
#[derive(Component, Default, Debug, Clone, Copy,f32Ops)]
#[require(InstantAngularVelocity)]
pub struct InstantAngularAcceleration(f32);
impl InstantAngularAcceleration{
    pub fn new(acceleration: f32)->Self{
        Self(acceleration)
    }
    pub fn consume(&mut self)->f32{
        let output = self.0;
        self.0 = 0.0;
        output
    }
    #[allow(unused)]
    pub fn limit(&mut self, limit: f32){
        let acc = self.0;
        if acc > limit{
            self.0 = limit;
        }
    }
    pub fn clear(&mut self){
        self.0 = 0.0;
    }
}

pub fn apply_linear_acceleration(time: Res<Time>, mut query: Query<(&mut InstantAngularVelocity, &mut InstantAngularAcceleration)>){
    for (mut vel, mut acc) in query.iter_mut(){
        *acc *= time.delta_secs();
        *vel += **acc;
        acc.clear();
    }
}
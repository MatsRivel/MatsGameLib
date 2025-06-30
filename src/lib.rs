use bevy::prelude::*;
use std::ops::{Add,AddAssign,Mul,MulAssign,Sub,SubAssign, Deref};
use ops_derive::{Vec2Ops,f32Ops};
pub mod entities;
pub mod camera;
pub mod movement;
pub mod utils;
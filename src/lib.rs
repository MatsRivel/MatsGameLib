#![feature(trivial_bounds)]
use bevy::prelude::*;
use std::ops::{Add,AddAssign,Mul,MulAssign,Sub,SubAssign, Deref};
use ops_derive::*;
pub mod entities;
pub mod camera;
pub mod movement;
pub mod utils;
pub mod destruction;
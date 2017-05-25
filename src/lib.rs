#![cfg_attr(bench, feature(test))]
#![feature(conservative_impl_trait)]

#[macro_use] extern crate bitflags;
extern crate itertools;
extern crate ordermap;
extern crate rand;

pub mod aura;
pub mod crusader;
pub mod dps;
pub mod formation;
pub mod formation_search;
pub mod gear;
pub mod talent;
pub mod user_data;

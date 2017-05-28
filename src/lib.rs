#![cfg_attr(test, feature(test))]
#![feature(conservative_impl_trait)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate serde_derive;
extern crate clap;
extern crate itertools;
extern crate ordermap;
extern crate rand;
extern crate serde;
extern crate toml;

pub mod aura;
pub mod cli;
pub mod crusader;
pub mod dps;
pub mod formation;
pub mod formation_search;
pub mod gear;
pub mod talent;
pub mod user_data;

use std::fs::File;
use std::io::prelude::*;

use user_data::*;

pub fn create_user_data() -> UserData {
    let mut user_data_toml = String::new();
    File::open("user_data.toml").unwrap().read_to_string(&mut user_data_toml).unwrap();
    match user_data::from_toml(&user_data_toml) {
        Ok(d) => d,
        Err(e) => match e.line_col() {
            Some((line, _)) => panic!("Error parsing user data on line {}: {}", line, e),
            None => panic!("Error parsing user data: {}", e),
        },
    }
}

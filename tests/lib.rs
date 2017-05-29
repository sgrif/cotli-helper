#![deny(warnings)]

extern crate cotli_helper;

pub mod support;

macro_rules! assert_formation_dps {
    ($expected:expr, $formation:expr) => {
        assert_dps_eq!($expected, $formation.total_dps(&Default::default()));
    }
}

macro_rules! assert_dps_eq {
    ($expected:expr, $dps:expr) => {
        assert_eq!($expected, &$dps.to_string());
    }
}

mod crusaders;

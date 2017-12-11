#![feature(test)]

extern crate serde;
extern crate serde_json;
extern crate measurements;
extern crate dof_calc;
extern crate test;

use measurements::{Length, Aperture};
use dof_calc::*;
use test::Bencher;

#[bench]
fn bench_work(b: &mut Bencher) {
    b.iter(|| Output::calc(Input::new_from_raw(Length::from_millimeters(50.0),
                                    Length::from_meters(0.6),
                                    Aperture::from_aperture_number(2.0),
                                    Length::from_micrometers(20.0))));
}


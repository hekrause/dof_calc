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
                                    Length::from_micrometers(20.0))))
}

#[bench]
fn bench_json_input_serialized(b: &mut Bencher) {
    let input = Input::new_from_raw(Length::from_millimeters(50.0),
                                    Length::from_meters(0.6),
                                    Aperture::from_aperture_number(2.0),
                                    Length::from_micrometers(20.0));

    b.iter(|| serde_json::to_string(&input).unwrap())
}

#[bench]
fn bench_json_input_deserialized(b: &mut Bencher) {
    let input = Input::new_from_raw(Length::from_millimeters(50.0),
                                    Length::from_meters(0.6),
                                    Aperture::from_aperture_number(2.0),
                                    Length::from_micrometers(20.0));

    let serialized: String = serde_json::to_string(&input).unwrap();

    b.iter(|| Some::<Input>(serde_json::from_str(&serialized).unwrap()))
}

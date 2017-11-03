
extern crate measurements;
extern crate dof_calc;

use measurements::{Length, Aperture};
use dof_calc::*;

#[test]
fn work() {
    let input = Input::new(Length::from_millimeters(50.0),
                           Length::from_meters(0.6),
                           Aperture::from_aperture_number(2.0),
                           Length::from_micrometers(20.0));
    let output = Output::calc(input);

    println!("{:#?}", output);
}

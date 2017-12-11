extern crate serde;
extern crate serde_json;
extern crate measurements;
extern crate dof_calc;

use measurements::{Length, Aperture};
use dof_calc::*;

#[test]
fn work() {
    let input = Input::new_from_raw(Length::from_millimeters(50.0),
                                    Length::from_meters(0.6),
                                    Aperture::from_aperture_number(2.0),
                                    Length::from_micrometers(20.0));
    let _output = Output::calc(input);
}

/*
#[test]
fn test_format_value() {
    let len = Length::from_kilometers(0.01);
    let new = Output::format_value(len);
    println!("{:#?}", new);
}

#[test]
fn json_input() {
    let input = Input::new_from_raw(Length::from_millimeters(50.0),
                                    Length::from_meters(0.6),
                                    Aperture::from_aperture_number(2.0),
                                    Length::from_micrometers(20.0));

    let serialized = serde_json::to_string(&input).unwrap();
    assert_eq!(serialized,
               "{\
               \"focal_length\":{\"meters\":0.05},\
               \"focus_distance\":{\"meters\":0.6},\
               \"f_number\":{\"aperture_number\":2.0},\
               \"circle_of_confusion\":{\"meters\":0.00002}\
               }");

    let deserialized: Input = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized,
               Input::new_from_raw(Length::from_millimeters(50.0),
                                   Length::from_meters(0.6),
                                   Aperture::from_aperture_number(2.0),
                                   Length::from_micrometers(20.0)));
}

#[test]
fn json_output() {
    let input = Input::new_from_raw(Length::from_millimeters(50.0),
                                    Length::from_meters(0.6),
                                    Aperture::from_aperture_number(2.0),
                                    Length::from_micrometers(20.0));
    let output = Output::calc(input);

    let serialized = serde_json::to_string(&output).unwrap();
    assert_eq!(serialized, "{\
                            \"hyperfocal_distance\":{\"meters\":62.550000000000007},\
                            \"hyperfocal_near_limit\":{\"meters\":31.275000000000003},\
                            \"near_distance\":{\"meters\":0.5947660586835845},\
                            \"far_distance\":{\"meters\":0.6053268765133172},\
                            \"depth_of_field\":{\"meters\":0.010560817829732727},\
                            \"in_front_of_object\":{\"meters\":0.005233941316415525},\
                            \"behind_object\":{\"meters\":0.0053268765133172028}\
                            }");

    let deserialized: Output = serde_json::from_str(&serialized).unwrap();
}
*/

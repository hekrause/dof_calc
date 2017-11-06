
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate measurements;

use measurements::Measurement;
use measurements::{Length, Aperture};
use std::fmt;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Input {
    focal_length: Length,
    focus_distance: Length,
    f_number: Aperture,
    circle_of_confusion: Length,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Output {
    hyperfocal_distance: Length,
    hyperfocal_near_limit: Length,
    near_distance: Length,
    far_distance: Length,
    depth_of_field: Length,
    in_front_of_object: Length,
    behind_object: Length,
}

impl Input {
    pub fn new_from_raw(f_len: Length, f_dis: Length, f_num: Aperture, coc: Length) -> Self {
        Input {
            focal_length: f_len,
            focus_distance: f_dis,
            f_number: f_num,
            circle_of_confusion: coc,
        }
    }
}

impl Output {
    pub fn new(h_dis: Length,
               h_near_lim: Length,
               near_dis: Length,
               far_dis: Length,
               dof: Length,
               front_obj: Length,
               behind_obj: Length)
               -> Self {
        Output {
            hyperfocal_distance: h_dis,
            hyperfocal_near_limit: h_near_lim,
            near_distance: near_dis,
            far_distance: far_dis,
            depth_of_field: dof,
            in_front_of_object: front_obj,
            behind_object: behind_obj,
        }
    }

    pub fn calc(input_params: Input) -> Output {
        let h_dis = Output::calc_hyperfocal_distance(&input_params.focal_length,
                                                     &input_params.f_number,
                                                     &input_params.circle_of_confusion);
        let h_near_lim = Output::calc_hyperfocal_near_limit(&h_dis);
        let near_dis = Output::calc_near_distance(&h_dis,
                                                  &input_params.focal_length,
                                                  &input_params.focus_distance);
        let far_dis = Output::calc_far_distance(&h_dis,
                                                &input_params.focal_length,
                                                &input_params.focus_distance);
        let dof = Output::calc_dof(&near_dis, &far_dis);
        let front_obj = Output::calc_in_front_object(&input_params.focus_distance, &near_dis);
        let behind_obj = Output::calc_behind_object(&input_params.focus_distance, &far_dis);

        Output::new(h_dis,
                    h_near_lim,
                    near_dis,
                    far_dis,
                    dof,
                    front_obj,
                    behind_obj)
    }

    fn calc_hyperfocal_distance(f_len: &Length, f_num: &Aperture, coc: &Length) -> Length {
        Length::from_base_units(((f_len.as_base_units() * f_len.as_base_units()) /
                                 (f_num.as_base_units() * coc.as_base_units()) +
                                 f_len.as_base_units()))
    }

    fn calc_hyperfocal_near_limit(h_near_lim: &Length) -> Length {
        Length::from_base_units(h_near_lim.as_base_units() / 2.0)
    }

    fn calc_near_distance(h_dis: &Length, f_len: &Length, f_dis: &Length) -> Length {
        Length::from_base_units((f_dis.as_base_units() *
                                 (h_dis.as_base_units() - f_len.as_base_units())) /
                                (h_dis.as_base_units() + f_dis.as_base_units() -
                                 (2.0 * f_len.as_base_units())))
    }

    fn calc_far_distance(h_dis: &Length, f_len: &Length, f_dis: &Length) -> Length {
        Length::from_base_units((f_dis.as_base_units() *
                                 (h_dis.as_base_units() - f_len.as_base_units())) /
                                (h_dis.as_base_units() - f_dis.as_base_units()))
    }

    fn calc_dof(near_dis: &Length, far_dis: &Length) -> Length {
        Length::from_base_units(far_dis.as_base_units() - near_dis.as_base_units())
    }

    fn calc_in_front_object(f_dis: &Length, near_dis: &Length) -> Length {
        Length::from_base_units(f_dis.as_base_units() - near_dis.as_base_units())
    }

    fn calc_behind_object(f_dis: &Length, far_dis: &Length) -> Length {
        Length::from_base_units(far_dis.as_base_units() - f_dis.as_base_units())
    }
}

use crate::{ray::Ray, color::Color, world::Record, point::Point};

use std::rc::Rc;

use super::Material;

pub struct Diffuse {
    color : Color<f64>,
}

impl Diffuse {
    pub fn new(color : Color<f64>) -> Diffuse {
        Diffuse {
            color,
        }
    }
}

impl Material for Diffuse {
    fn scatter(&self, r_in : Ray, hit : Record) -> (Ray, &Color<f64>){
        let mut target = hit.normal + Point::random_unit_in_sphere(1.);
        if target.near_zero() {
            target = target.unit();
        }
        (Ray::new(hit.p, target, r_in.get_depth()+1),
        &self.color)
    }
}
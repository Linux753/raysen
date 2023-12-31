use crate::{ray::Ray, world::Record, color::Color};

use std::rc::Rc;

pub mod diffuse;
pub mod metal;
pub mod dielectric;

pub trait Material {
    fn scatter(&self, r_in : Ray, hit : Record) -> (Ray, &Color<f64>);
}


pub enum Texture {
    Diffuse(diffuse::Diffuse),
    Metal(metal::Metal),
    Dielectric(dielectric::Dielectric),
}

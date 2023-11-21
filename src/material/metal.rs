use crate::{color::Color, ray::Ray, point::Point};

use super::Material;



pub struct Metal {
    albedo : Color<f64>,
    fuzzyness : f64,
}

impl Metal {
    pub fn new(albedo : Color<f64>, fuzzyness : f64) -> Self {
        Metal { 
            albedo,
            fuzzyness,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in : crate::ray::Ray, hit : crate::world::Record) -> (crate::ray::Ray, &Color<f64>) {
        let dir = *(r_in.dir()) - (hit.normal*(hit.normal&*(r_in.dir()))*2.);
        if self.fuzzyness==0. {
            return (Ray::new(hit.p, dir, r_in.get_depth()+1),
                    &self.albedo);
        }
        let ray = Ray::new(hit.p, dir + Point::random_in_sphere(self.fuzzyness), r_in.get_depth()+1);
        //Test if we didn't launched the fuzzed ray into the object
        if *(ray.dir())&hit.normal > 0. {
            (ray,
            &self.albedo)
        }
        else {
            (ray,
            &Color {r: 0., g: 0., b: 0.})
        }
    }
}
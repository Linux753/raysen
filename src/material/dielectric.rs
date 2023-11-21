use crate::{ray::Ray, world::Record, color::Color, point::Point};

use super::Material;


pub struct Dielectric {
    eta : f64,
    albedo : Color<f64>,
}

impl Dielectric {
    pub fn new(eta : f64) -> Self {
        Dielectric { 
            eta,
            albedo : Color {r:1., g:1., b:1.},
        }
    }

    //Calculate the reflected direction (as a mettalic mzterial without fuzziness)
    fn reflect(&self, r_in : Ray, hit :  Record) -> Ray {
        let dir = *(r_in.dir()) - (hit.normal*(hit.normal&*(r_in.dir()))*2.);
        Ray::new(hit.p, dir, r_in.get_depth()+1)
    }

    //Calculate the refracted direction
    fn refract(&self, inc_unit : Point<f64>, normal : Point<f64>, eta_quotient : f64, cos_theta : f64) -> Point<f64> {
        let dir_out_perp = (inc_unit + normal*cos_theta)*eta_quotient;
        let dir_out_parallel = normal * (-1. * (1.-(dir_out_perp.norm_squared())).sqrt());
        dir_out_parallel + dir_out_perp
    }

    //calculate the reflectance coefficient using the µSchlick Approximation
    //The reflectance is between 0 and 1
    fn reflectance(&self, cosine : f64, ref_idx : f64) -> f64 {
        let r0 : f64 = (1.-ref_idx)/(1.+ref_idx);
        let r0 = r0 * r0;
        r0 + (1.-r0)*((1.-cosine).powi(5))
    }
}

impl Material for Dielectric {
    
    fn scatter(&self, r_in : crate::ray::Ray, hit : crate::world::Record) -> (crate::ray::Ray, &crate::color::Color<f64>) {
        let inc_unit = r_in.dir().unit();
        let eta_quotient = if hit.front_face { 1./self.eta } else { self.eta/1.};
        let cos_theta = (-1. * (inc_unit&hit.normal)).min(1.);
        let sin_theta = (1.-cos_theta*cos_theta).sqrt();

        //If total reflection or reflectance (applying Schlick Approximation)
        if (sin_theta*eta_quotient > 1.)||(self.reflectance(cos_theta, eta_quotient) > rand::random() )  {
            (self.reflect(r_in, hit),
            &self.albedo)
        }
        else {
            (Ray::new(hit.p, self.refract(inc_unit, hit.normal, eta_quotient, cos_theta), r_in.get_depth()+1),
            &self.albedo)
        }
    }
}
pub mod sphere;
use sphere::Sphere;
use crate::point::Point;

use super::Ray;

#[derive(Clone)]
pub struct Record {
    pub t : f64,
    pub p : Point<f64>,
    pub normal : Point<f64>,
    pub front_face : bool,
}

impl Record {
    pub fn new(ray : &Ray, t : f64, p : Point<f64>, outward_normal : Point<f64>) -> Record {
        let front_face = *(ray.dir())&outward_normal < 0.0;
        let normal = if front_face {outward_normal} else {outward_normal*(-1f64)};
        Record {
            t,
            p,
            normal,
            front_face,
        }
    }
    
    pub fn to_ray(self, depth : u32) -> Ray {
        Ray::new(self.p, self.normal, depth)
    }
}

pub trait Hitable {
    fn hit(&self, r : &Ray, t_min : f64, t_max : f64) -> Option<f64>; 
    fn get_records(&self, r : &Ray, t : f64) -> Record;
}

pub enum Surface {
    Sphere(Sphere)
}

pub struct World {
    pub objects : Vec<Surface>,
}

impl World {
    pub fn new() -> World {
        World {
            objects : Vec::new()
        }
    }
    pub fn add_sphere(&mut self, center : Point<f64>, radius : f64){
        self.objects.push(Surface::Sphere(Sphere::new(center,radius)));
    }
}



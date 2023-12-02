

use std::sync::Arc;

use crate::{point::Point, ray::Ray, material::Texture};
use super::{Hitable, Record, aabb::Aabb};

pub struct Sphere {
    center : Point<f64>,
    radius : f64
}

impl Sphere {
    pub fn new(center : Point<f64>, radius : f64) -> Sphere {
        return Sphere {
            center,
            radius
        };
    }

    pub fn new_with_aabb(center : Point<f64>, radius : f64, texture : Arc<Texture>) -> Aabb {
        Aabb::new_one(super::Surface::Sphere(Sphere::new(center, radius)), texture)
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn get_center(&self) -> Point<f64> {
        self.center
    }
}

impl Hitable for  Sphere {
    fn hit(&self, r : &Ray, t_min : f64, t_max : f64) -> Option<f64> {
        let oc = *(r.orig())-self.center;
        let a : f64 = *(r.dir())&*(r.dir());
        let half_b : f64 = *(r.dir())&(oc);
        let c : f64 = ((oc)&(oc))-(self.radius*self.radius);
        let delta : f64 = half_b*half_b - a*c;
        if delta<0.0 {
            return None;
        }
        
        //We have a>0 so we can just compare -b-sqrt(delta) with -b+sqrt(delta)
        let sqrt_delta = delta.sqrt();
        let t1 = (-half_b - sqrt_delta)/a;
        let t2 = (-half_b + sqrt_delta)/a;
        if t1 >= t_min && t1 <= t_max {
            Some(t1)
        }
        else if t2>=t_min && t2<=t_max {
            Some(t2)
        }
        else{
            return None;
        }
    }
    fn get_records(&self, r : &Ray, t : f64) -> Record {
        let p = r.at(t);
        let outward_normal = (p-self.center)/self.radius;

        Record::new(r, t, p, outward_normal)
    }

    fn get_bb(&self) -> (Point<f64>, Point<f64>) {
        let radius_vec : Point<f64> = Point { x: self.radius, y: self.radius, z: self.radius };
        (self.center-radius_vec,
        self.center+radius_vec)
    }
}
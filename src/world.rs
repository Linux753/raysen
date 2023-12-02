pub mod sphere;
pub mod aabb;

use std::sync::Arc;

use sphere::Sphere;
use crate::{point::Point, material::{Texture, diffuse::{self, Diffuse}}, color::Color};
use self::aabb::Aabb;

use super::Ray;

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
    fn get_bb(&self) -> (Point<f64>, Point<f64>);
}

pub enum Surface {
    Sphere(Sphere),
    AABB(Aabb),
}

pub struct World {
    pub default_texture : Arc<Texture>,
    pub objects : Vec<(Surface, Arc<Texture>)>,
}

impl World {
    pub fn new() -> World {
        World {
            objects : Vec::new(),
            default_texture : Arc::new(Texture::Diffuse(Diffuse::new(Color::<f64> {r:1.0, g:1.0, b:1.0}))),
        }
    }

    pub fn new_from_vec(objects : Vec<(Surface, Arc<Texture>)>) -> World {
        World {
            objects,
            default_texture : Arc::new(Texture::Diffuse(Diffuse::new(Color::<f64> {r:1.0, g:1.0, b:1.0}))),
        }
    }

    pub fn add_sphere(&mut self, center : Point<f64>, radius : f64, texture : Arc<Texture>){
        self.objects.push((Surface::Sphere(Sphere::new(center, radius)), texture));
    }

    pub fn add_sphere_without_collision(&mut self, center : Point<f64>, radius : f64, texture : Arc<Texture>) -> bool {
        for (object, _) in &self.objects {
            match object {
                Surface::Sphere(sphere) => {
                    if (center-sphere.get_center()).norm() < (radius+sphere.get_radius()) {
                        return false;
                    }
                },
                _ => ()
            }
        }

        self.add_sphere(center, radius, texture);

        return true;
    }
}



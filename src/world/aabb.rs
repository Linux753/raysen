use std::{sync::Arc, mem::swap};

use crate::{point::Point, material::Texture};

use super::{World, Surface, Hitable};



pub struct Aabb {
    min : Point<f64>,
    max : Point<f64>,
    subworld : World
}

impl Aabb {
    pub fn new_one(surface : Surface, texture : Arc<Texture>) -> Aabb {
        let (min, max) = match &surface {
            Surface::Sphere(sphere) => sphere.get_bb(),
            Surface::AABB(_) => panic!("Should not encapsulate AABB in AABB"),
        };
        Aabb {
            min,
            max,
            subworld: World::new_from_vec(vec![(surface, texture)]),
        }
    }

    pub fn hit<'a, 'b>(&'a self, r : &'b crate::ray::Ray, mut t_min : f64, mut t_max : f64) -> Option<(f64, &'a Surface, Arc<Texture>)> {
        let inv_d = 1.0/r.dir().x;
        let mut t0 = (self.min.x - r.orig().x) * inv_d;
        let mut t1 = (self.max.x - r.orig().x) * inv_d;
    
        if inv_d<0.0 {
            swap(&mut t0, &mut t1);
        }
        t_min = t0.max(t_min);
        t_max = t1.min(t_max);

        if t_max < t_min {
            return None;
        }

        let inv_d = 1.0/r.dir().y;
        let mut t0 = (self.min.y - r.orig().y) * inv_d;
        let mut t1 = (self.max.y - r.orig().y) * inv_d;
    
        if inv_d<0.0 {
            swap(&mut t0, &mut t1);
        }
        t_min = t0.max(t_min);
        t_max = t1.min(t_max);

        if t_max < t_min {
            return None;
        }

        let inv_d = 1.0/r.dir().z;
        let mut t0 = (self.min.z - r.orig().z) * inv_d;
        let mut t1: f64 = (self.max.z - r.orig().z) * inv_d;
    
        if inv_d<0.0 {
            swap(&mut t0, &mut t1);
        }
        t_min = t0.max(t_min);
        t_max = t1.min(t_max);

        if t_max < t_min {
            return None;
        }

        return r.object_hitted(&self.subworld);
    }
}
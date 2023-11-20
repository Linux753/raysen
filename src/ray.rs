use std::rc::Rc;

use crate::material::{Texture, Material};
use crate::point::Point;
use crate::color::Color;
use crate::world::{World, Hitable, Surface};
pub struct Ray{
    orig : Point<f64>,
    dir : Point<f64>,
    depth : u32,
}

impl Ray {
    pub fn new(orig : Point<f64>, dir : Point<f64>, depth : u32) -> Ray {
        Ray {
            orig,
            dir,
            depth,
        }
    }

    pub fn get_depth(&self) -> u32 {
        self.depth
    }
    
    pub fn dir(&self) -> &Point<f64> {
        &self.dir
    }

    pub fn orig(&self) -> &Point<f64> {
        &self.orig
    }
    
    
    pub fn at(&self, t : f64) -> Point<f64> {
        self.orig + self.dir*t
    }

    pub fn color(self, world : &World) -> Color<f64> {
        if self.depth > 50 {
            return Color {r: 0., g: 0., b: 0.};
        }
        let t_max = f64::MAX;
        let t_min = 0.001;
        let mut best_t : Option<(f64, &Surface, Rc<Texture>)> = None;

        for (surface, texture) in &world.objects {
            match &surface {
                &Surface::Sphere(sphere) => {
                    if let Some(t) = sphere.hit(&self, t_min, t_max) {
                        best_t = match best_t {
                            None => Some((t, surface, Rc::clone(texture))),
                            Some((best_t, best_surf, best_text)) => {
                                if best_t>t {
                                    Some((t, surface, Rc::clone(texture)))
                                }
                                else{
                                    Some((best_t, best_surf, best_text))
                                }
                            }
                        };
                    }
                }
            }
        }
        match best_t {
            None => {
                let unit_dir = self.dir.unit();
                let a : f64 = (unit_dir.y+1.0)*0.5;
                Color {r: 1.0, g: 1.0, b: 1.0}*(1.0-a) + a*Color { r: 0.5, g: 0.7, b: 1.0 }
            }
            Some((t, surface, texture)) => {
                let best_record = match surface {
                    Surface::Sphere(sphere) => sphere.get_records(&self, t)
                };

                let (ray, color) = match texture.as_ref() {
                    Texture::Diffuse(diffuse) => diffuse.scatter(self, best_record),
                };
                ray.color(world)*color
                //let dir_col = (best_record.normal+Point { x: 1.0, y: 1.0, z: 1.0 })*0.5;
                //return Color {r: dir_col.x, g:dir_col.y, b:dir_col.z};
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation(){
        let a = Ray::new(Point { x: 3.0, y: 2.0, z: 1.0 }, Point { x: 1.0, y: 1.0, z: 0.0 }, 0);
        let b = Ray::new(Point { x: 5.0, y: 0.5, z: 0.3 }, Point { x: 2.0, y: 1.0, z: 3.0 }, 0);

        let c = a.orig&b.orig;
        let d = a.orig-b.orig;


        assert_ne!(c, 2.0);
        assert_ne!(d.x, a.orig.x);

        let e : Point<i32> = Point { x: 2, y: 4, z: 5 };
        let f : Point<i32> = Point { x: 1, y: 6, z: 0 };
        
        let g = e^f;
        let h = f&e;
        let i = f+e;
        
        assert_eq!(h, 26);
    }
}


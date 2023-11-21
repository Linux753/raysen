pub mod point;
pub mod color;
pub mod ray;
pub mod world;
pub mod material;

use rand::prelude::*;
use std::rc::Rc;

use point::Point;
use ray::Ray;
use material::Texture;

use crate::{world::World, color::Color};

pub struct Camera {
    center : Point<f64>,
    dir : Point<f64>,
    focal_lenght : f64,

    viewport_width : f64,
    viewport_height : f64,
    pixel_delta_u : Point<f64>,
    pixel_delta_v : Point<f64>,
    pixel00_loc : Point<f64>,

    pub image_height : u32,
    pub image_width : u32,
}

impl Camera{
    pub fn new(image_width : u32, aspect_ratio : f64) -> Camera {
        let image_height : u32 = ((image_width as f64)/aspect_ratio) as u32;

        let viewport_height = 2.0;
        let viewport_width = viewport_height*((image_width as f64)/(image_height as f64));

        let focal_lenght = 2.;
        let center : Point<f64> = Point { x: 0.0, y: 0.0, z: 0.0 };
        let dir = Point {x:0.0, y:0.0, z:-focal_lenght};
        
        let viewport_u = Point {x:viewport_width, y:0.0, z:0.0};
        let viewport_v = Point {x:0.0, y:-viewport_height, z:0.0};

        let pixel_delta_u = viewport_u/(image_width as f64);
        let pixel_delta_v = viewport_v/(image_height as f64);

        let pixel_up_left = center + dir - viewport_u/2.0 - viewport_v/2.0;
        let pixel00_loc = pixel_up_left + pixel_delta_u*0.5 + pixel_delta_v*0.5;



        Camera { 
            center, 
            dir,
            focal_lenght,
            viewport_width,
            viewport_height,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            image_height,
            image_width,
        }   

    }

    pub fn pixel_ray(&self, i : u32, j:u32) -> Ray {
        let mut rng = rand::thread_rng();
        let alpha : f64 = rng.gen::<f64>() - 0.5;
        let beta : f64 = rng.gen::<f64>() - 0.5;
        let pixel_center = self.pixel00_loc + self.pixel_delta_u*i + self.pixel_delta_u*alpha + self.pixel_delta_v*j + self.pixel_delta_v*beta;
        let dir = pixel_center - self.center;
        Ray::new(pixel_center, dir, 0)
    }
}


pub fn run() {
    //Init the ppm format
    let img_width : u32 = 1000 ;
    let max : u32 = 255;

    let camera : Camera = Camera::new(img_width, 16.0/9.0);

    println!("P3");
    println!("{} {}", camera.image_width, camera.image_height);
    println!("{max}");

    let sample_per_pixel = 100;

    let mut world = World::new();

    let bleu_dif = Rc::new(Texture::Diffuse(material::diffuse::Diffuse::new(Color { r: 0.3, g: 0.05, b: 0.4 })));
    let gris_dif = Rc::new(Texture::Diffuse(material::diffuse::Diffuse::new(Color { r: 0.5, g: 0.5, b: 0.5 })));
    let jaune_dif = Rc::new(Texture::Diffuse(material::diffuse::Diffuse::new(Color {r:0.4, g: 0.6, b: 0.1})));
    let bleu_met = Rc::new(Texture::Metal(material::metal::Metal::new(Color { r: 0.75, g: 0.75, b: 0.95 }, 0.0)));
    let rouge_met = Rc::new(Texture::Metal(material::metal::Metal::new(Color { r: 0.95, g: 0.1, b: 0.05 }, 0.15)));
    let verre = Rc::new(Texture::Dielectric(material::dielectric::Dielectric::new(1.5)));

    world.add_sphere(Point { x: 0.0, y: -0.5, z: -3.1 }, 0.5, Rc::clone(&verre));
    world.add_sphere(Point { x: -1.0, y: -0.7, z: -3.0 }, 0.2, Rc::clone(&bleu_met));
    world.add_sphere(Point { x: -0.0, y: -0.8, z: -4.3 }, 0.2, Rc::clone(&rouge_met));
    world.add_sphere(Point { x: 1.0, y: -0.5, z: -3.0 }, 0.5, Rc::clone(&bleu_dif));
    world.add_sphere(Point { x: -2.0, y: 0.0, z: -3.0 }, 1., Rc::clone(&jaune_dif));
    world.add_sphere(Point { x: 0.0, y: -101., z: -3. }, 100., Rc::clone(&gris_dif));

    for _ in 0..100 {
        let point = Point::random_in_sphere(1.);
        eprintln!("x: {}, y : {}, z: {}" , point.x, point.y, point.z);
    }

    for j in 0..camera.image_height {
        eprintln!("Rendering line {}", j+1);
        for i in 0..img_width {
            let mut color = Color {r : 0., g : 0., b:0.};
            for _ in 0..sample_per_pixel {
                color = color + camera.pixel_ray(i, j).color(&world);
            }
            color.write(sample_per_pixel);
        }
    }
}
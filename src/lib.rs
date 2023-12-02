pub mod point;
pub mod color;
pub mod ray;
pub mod world;
pub mod material;

use rand::prelude::*;
use std::{vec, thread, sync::{mpsc, Arc}};

use point::Point;
use ray::Ray;
use material::Texture;

use crate::{world::World, color::Color};

pub struct Camera {
    center : Point<f64>,
    dir : Point<f64>,
    lens_radius : f64,
    focus_dist : f64,

    w : Point<f64>, //Looking direction vector (unit)
    u : Point<f64>, //Horizontal direction vector (unit)
    v : Point<f64>, //Vertical direction vector (unit)

    viewport_width : f64,
    viewport_height : f64,
    pixel_delta_u : Point<f64>,
    pixel_delta_v : Point<f64>,
    pixel00_loc : Point<f64>,

    pub image_height : u32,
    pub image_width : u32,
}

impl Camera{
    pub fn new(origin : Point<f64>, lookat : Point<f64> , image_width : u32, aspect_ratio : f64, vfov : f64, aperture : f64, focus_dist : f64) -> Camera {
        let image_height : u32 = ((image_width as f64)/aspect_ratio) as u32;


        let theta = vfov.to_radians();
        let h = (theta/2.).tan()*focus_dist;
        let viewport_height = 2.0*h;
        let viewport_width = viewport_height*((image_width as f64)/(image_height as f64));
        
        let center : Point<f64> = origin;
        let look_dir : Point<f64> = (lookat-center).unit();
        let up : Point<f64> = Point { x: 0., y: 1., z: 0. };
        let vup = (up + look_dir*(-1.*(up&look_dir))).unit();
        let uup = vup^look_dir;
        
        let viewport_u = uup * viewport_width;
        let viewport_v = vup * -viewport_height;

        let pixel_delta_u = viewport_u/(image_width as f64);
        let pixel_delta_v = viewport_v/(image_height as f64);

        let pixel_up_left = center + look_dir*focus_dist - viewport_u/2.0 - viewport_v/2.0;
        let pixel00_loc = pixel_up_left + pixel_delta_u*0.5 + pixel_delta_v*0.5;



        Camera { 
            center, 
            dir : look_dir,
            lens_radius : aperture/2.0,
            focus_dist,

            w : look_dir,
            v : vup,
            u : uup,

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
        //Antialiasing random on the position of the pixel center
        let mut rng = rand::thread_rng();
        let alpha : f64 = rng.gen::<f64>() - 0.5;
        let beta : f64 = rng.gen::<f64>() - 0.5;
        
        //Focus distance : random on the position of the pixel origin
        let rd = Point::random_in_circle(self.lens_radius);
        let offset = self.u * rd.x + self.v * rd.y;

        let ray_origin = self.center + offset;
        let dir = self.pixel00_loc + self.pixel_delta_u*i + self.pixel_delta_u*alpha  + self.pixel_delta_v*j + self.pixel_delta_v*beta - ray_origin;
        Ray::new(ray_origin, dir, 0)
    }
}

pub fn scene1(img_width : u32) -> (Camera, World) {
    let camera : Camera = Camera::new(Point { x: 1., y: 0.5, z: 2. },Point { x: 0.0, y: -0.5, z: -3.0 } , img_width, 16.0/9.0, 45., 0.2, 5.0);

    let mut world = World::new();

    let bleu_dif = Arc::new(Texture::Diffuse(material::diffuse::Diffuse::new(Color { r: 0.3, g: 0.05, b: 0.4 })));
    let gris_dif: Arc<Texture> = Arc::new(Texture::Diffuse(material::diffuse::Diffuse::new(Color { r: 0.5, g: 0.5, b: 0.5 })));
    let jaune_dif = Arc::new(Texture::Diffuse(material::diffuse::Diffuse::new(Color {r:0.4, g: 0.6, b: 0.1})));
    let bleu_met = Arc::new(Texture::Metal(material::metal::Metal::new(Color { r: 0.75, g: 0.75, b: 0.95 }, 0.0)));
    let rouge_met = Arc::new(Texture::Metal(material::metal::Metal::new(Color { r: 0.95, g: 0.1, b: 0.05 }, 0.15)));
    let verre = Arc::new(Texture::Dielectric(material::dielectric::Dielectric::new(1.5)));

    world.add_sphere(Point { x: 0.0, y: -0.5, z: -3.1 }, 0.5, Arc::clone(&verre));
    world.add_sphere(Point { x: 0.0, y: -0.5, z: -3.1 }, -0.4, Arc::clone(&verre));
    world.add_sphere(Point { x: -1.0, y: -0.8, z: -3.0 }, 0.2, Arc::clone(&bleu_met));
    world.add_sphere(Point { x: -0.0, y: -0.8, z: -4.3 }, 0.2, Arc::clone(&rouge_met));
    world.add_sphere(Point { x: 1.0, y: -0.5, z: -3.0 }, 0.5, Arc::clone(&bleu_dif));
    world.add_sphere(Point { x: -2.0, y: 0.0, z: -4.5 }, 1., Arc::clone(&jaune_dif));
    world.add_sphere(Point { x: 0.0, y: -1001., z: -3. }, 1000., Arc::clone(&gris_dif));

    (camera, world)
}

pub fn scene2(img_width : u32) -> (Camera, World) {
    let camera : Camera = Camera::new(Point { x: -1., y: 1.5, z: 2. },Point { x: -1.0, y: 0.75, z: -3.0 } , img_width, 16.0/9.0, 30., 0.08, 5.90);

    let mut world = World::new();

    let bleu_dif = Arc::new(Texture::Diffuse(material::diffuse::Diffuse::new(Color { r: 0.3, g: 0.05, b: 0.4 })));
    let gris_dif = Arc::new(Texture::Diffuse(material::diffuse::Diffuse::new(Color { r: 0.5, g: 0.5, b: 0.5 })));
    let jaune_dif = Arc::new(Texture::Diffuse(material::diffuse::Diffuse::new(Color {r:0.4, g: 0.6, b: 0.1})));
    let bleu_met = Arc::new(Texture::Metal(material::metal::Metal::new(Color { r: 0.75, g: 0.75, b: 0.95 }, 0.0)));
    let rouge_met = Arc::new(Texture::Metal(material::metal::Metal::new(Color { r: 0.95, g: 0.1, b: 0.05 }, 0.05)));
    let verre = Arc::new(Texture::Dielectric(material::dielectric::Dielectric::new(1.5)));

    world.add_sphere(Point { x: 0.0, y: -1000., z: -3. }, 1000., Arc::clone(&gris_dif));
    world.add_sphere(Point { x: -2.0, y: 0.75, z: -2.0 }, 0.75, Arc::clone(&bleu_met));
    world.add_sphere(Point { x: -1.5, y: 0.75, z: -3.75 }, 0.75, Arc::clone(&verre));
    world.add_sphere(Point { x: -1.5, y: 0.75, z: -3.75 }, -0.65, Arc::clone(&verre));
    world.add_sphere(Point { x: -1.0, y: 0.75, z: -5.50 }, 0.75, Arc::clone(&bleu_dif));

    let vec_texture = vec![bleu_dif, gris_dif, jaune_dif, bleu_met, rouge_met, verre];
    let mut rng = rand::thread_rng();
    for _ in 0..25 {
        world.add_sphere_without_collision(
            Point { x: rng.gen_range(-5.0..3.0), y: 0.2, z: rng.gen_range(-13.0..3.0) },
            0.20,
            Arc::new(Texture::Dielectric(material::dielectric::Dielectric::new(rng.gen_range(1.5..2.4))))
        );
    }

    for _ in 0..110 {
        world.add_sphere_without_collision(
            Point { x: rng.gen_range(-5.0..3.0), y: 0.2, z: rng.gen_range(-13.0..3.0) },
            0.20,
            Arc::new(Texture::Diffuse(material::diffuse::Diffuse::new(Color::random()))),
        );
    }

    for _ in 0..60 {
        world.add_sphere_without_collision(
            Point { x: rng.gen_range(-5.0..3.0), y: 0.2, z: rng.gen_range(-13.0..3.0) },
            0.20,
            Arc::new(Texture::Metal(material::metal::Metal::new(Color::random(), 0.0))),
        );
    }

    for _ in 0..60 {
        world.add_sphere_without_collision(
            Point { x: rng.gen_range(-5.0..3.0), y: 0.2, z: rng.gen_range(-13.0..3.0) },
            0.20,
            Arc::new(Texture::Metal(material::metal::Metal::new(Color::random(), rand::random()))),
        );
    }

    (camera, world)
}

fn add_color_vec(dst : &mut Vec<Color<f64>>, src : Vec<Color<f64>>){
    for i in 0..src.len() {
        let color_dst = dst.get_mut(i).expect("The received vector is too long.");
        let color_src  = src.get(i).expect("There should be an element bc we iter on it's lenght");
        color_dst.r += color_src.r;
        color_dst.g += color_src.g;
        color_dst.b += color_src.b
    }
}

pub fn run() {
    //Init the ppm format
    let img_width : u32 = 1920 ;
    let max : u32 = 255;

    let nb_thread = 8;

    let (camera, world) = scene2(img_width);
    let camera = Arc::new(camera);
    let world = Arc::new(world);
    
    println!("P3");
    println!("{} {}", camera.image_width, camera.image_height);
    println!("{max}");

    let sample_per_pixel = 100;
    let sample_per_pixel_per_thread = (sample_per_pixel as f64/nb_thread as f64).ceil() as u32;
    
    let mut handles = vec![];
    let mut rxs = vec![];

    for _ in 0..nb_thread {
        let (tx, rx) = mpsc::channel();
        let camera_thread = Arc::clone(&camera);
        let world_thread = Arc::clone(&world);
        let handle = thread::spawn( move || {
            for j in 0..camera_thread.as_ref().image_height {
                let mut colors = vec![];
                for i in 0..img_width {
                    let mut color = Color {r : 0., g : 0., b:0.};
                    for _ in 0..sample_per_pixel_per_thread {
                        color = color + camera_thread.as_ref().pixel_ray(i, j).color(&world_thread.as_ref());
                    }
                    colors.push(color);
                }
                tx.send(colors).unwrap();
            }
        });
        rxs.push(rx);
        handles.push(handle);
    }

    for j in 0..camera.image_height {
        eprintln!("Rendering line {}", j+1);
        let mut color_tot: Vec<Color<f64>> = Vec::with_capacity(img_width as usize);
        color_tot.resize_with(img_width as usize, || Color { r: 0.0, g: 0.0, b: 0.0 });
        for rx in &rxs {
            let colors_thread = rx.recv().unwrap();
            add_color_vec(&mut color_tot, colors_thread);
        }

        for color in color_tot {
            color.write(sample_per_pixel_per_thread*nb_thread);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

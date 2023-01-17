extern crate rand;
extern crate rayon;
extern crate indicatif;

mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;
mod material;
mod ppm;
mod plane;

use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};

use indicatif::{ProgressBar, ProgressStyle};
use crate::ppm::gen_ppm;

use image::{RgbImage, ImageBuffer, Rgb, imageops};
use material::scatter;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::*;
use crate::sphere::Sphere;
use crate::plane::{Plane, Rectangle, Aabb};
use crate::camera::Camera;
use crate::material::Material;

fn main() {

    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = 300;
    const SAMPLES: i32 = 8000;

    

    // let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    // list.push(Box::new(Sphere::sphere(
    //     Vec3::new(0.0, 0.0, -1.0),
    //      0.5,
    //      Material::Lambertian { albedo: Vec3::random()})
    //     ));

    // list.push(Box::new(Sphere::sphere(
    //     Vec3::new(-1.0, 0.0, -1.0),
    //      0.5,
    //      Material::Lambertian { albedo: Vec3::random()})
    //     ));

    // list.push(Box::new(Sphere::sphere(
    //     Vec3::new(1.0, 0.0, -1.0),
    //      0.5,
    //      Material::Metal { albedo: Vec3::random(), fuzz: 0.2 })
    //     ));


    // list.push(Box::new(Sphere::sphere(
    //     Vec3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     Material::Metal { albedo: Vec3::random(), fuzz: 1.0 })
    // ));


    // let world = HittableList::new(list);

    let world: HittableList = cornell_box();
    
    //Standard X-axis
    let standard_cam = Camera::new(
        Vec3::new(40.0, 20.0, 0.0),
        Vec3::new(0.0, 20.0, 00.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64,
    );
    //Standard X-axis
    let cam = Camera::new(
        Vec3::new(40.0, 20.0, 0.0),
        Vec3::new(0.0, 20.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64,
    );


    // let cam: Camera = Camera::new(Vec3::new(13.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 0.0),
    // Vec3::new(0.0, 1.0, 0.0), 20.0, (IMAGE_WIDTH as f64)/(IMAGE_HEIGHT as f64), 0.1, 10.0);

    let bar = ProgressBar::new(IMAGE_HEIGHT.into());
    bar.set_style(ProgressStyle::default_bar().template("[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]").ok().unwrap());

    let filename = "data/new_image.png".to_string();

    let scene: Vec<Vec<Vec3>> = (0..IMAGE_HEIGHT).into_par_iter().map(|y_rev| {
        let y: f64 = IMAGE_HEIGHT as f64 - y_rev as f64 - 1.0;
        let row: Vec<Vec3> = (0..IMAGE_WIDTH).into_par_iter().map(|x| {
            let mut color_vector: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES {
                let u: f64 = (x as f64 + rand::random::<f64>()) / IMAGE_WIDTH as f64;
                let v: f64 = (y as f64 + rand::random::<f64>()) / IMAGE_HEIGHT as f64;
                let r: Ray = cam.get_ray(u, v);
                color_vector = color_vector + color(&r, &world, 10);
            }
            color_vector = color_vector/SAMPLES as f64;
            color_vector = 255.99*Vec3::new(color_vector.x().sqrt(), color_vector.y().sqrt(), color_vector.z().sqrt());
            color_vector.colorize();
            color_vector
        }).collect();
        bar.inc(1);
        row
    }).collect();

    bar.finish();

    gen_ppm(scene, filename);
}

fn color(r: &Ray, world: &HittableList, depth: i64) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.001, std::f64::MAX) {
        let mut scattered = Ray::ray(Vec3::default(), Vec3::default());
        let mut attentuation = Vec3::default();

        if depth < 50 && scatter(&rec.material, r, &rec, &mut attentuation, &mut scattered) {
            return attentuation * color(&scattered, world, depth+1);
        }
        else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    }
    else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::default();
    let mut rng_double = rand::thread_rng();

    loop {
        p = 2.0 * Vec3::new(rng_double.gen::<f64>(), rng_double.gen::<f64>(), rng_double.gen::<f64>()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    list.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) })
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen::<f64>();
            let center: Vec3 = Vec3::new(a as f64 + 0.9 * rng.gen::<f64>(),
                0.2, b as f64 + 0.9 * rng.gen::<f64>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {  // diffuse
                    list.push(Box::new(Sphere::sphere(
                        center,
                        0.2,
                        Material::Lambertian { albedo: Vec3::random() })
                    ));
                } else {  //metal
                    list.push(Box::new(Sphere::sphere(
                        center,
                        0.2,
                        Material::Metal {
                            albedo: Vec3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>())),
                            fuzz: 0.5 * rng.gen::<f64>() })
                    ));
                }
            }

            list.push(Box::new(Sphere::sphere(
                Vec3::new(-4.0, 1.0, 0.0),
                1.0,
                Material::Lambertian { albedo: Vec3::new(0.4, 0.4, 0.1) })
            ));

            list.push(Box::new(Sphere::sphere(
                Vec3::new(4.0, 1.0, 0.0),
                1.0,
                Material::Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 })
            ));
        }
    }
    let world = HittableList::new(list);
    world
}

fn plane_scene() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    // list.push(Box::new(Sphere::sphere(
    //     Vec3::new(0.0, -1000.0, 0.0),
    //     1000.0,
    //     Material::Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) })
    // ));

            list.push(Box::new(Sphere::sphere(
                Vec3::new(0.0, 0.0, -1.0),
                0.5,
                Material::Lambertian { albedo: Vec3::new(0.8, 0.8, 0.3) })
            ));

            list.push(Box::new(Sphere::sphere(
                Vec3::new(0.0, -100.5, -1.0),
                100.0,
                Material::Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) })
            ));


            list.push(Box::new(Plane::plane(
                Vec3::new(0.0, 1.0, 0.0),
                2.0,
                Material::Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) })
            ));
            let world = HittableList::new(list);
            world
}

fn cornell_box() -> HittableList {
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    // Set up the objects in the scene
    let red = Material::Lambertian { albedo: Vec3::new(0.65, 0.05, 0.05) };
    let white = Material::Lambertian { albedo: Vec3::new(0.73, 0.73, 0.73) };
    let green = Material::Lambertian { albedo:Vec3::new(0.12, 0.45, 0.15) };
    let light = Material::Light { emittance: Vec3::new(40.0, 20.0, 10.0) };

    // Add the floor
    list.push(Box::new(Plane::plane(
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        white,
    )));

    // Add the roof
    list.push(Box::new(Plane::plane(
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        white,
    )));

    // Add the backwall
    list.push(Box::new(Plane::plane(
        Vec3::new(1.0, 0.0, 0.0),
        0.0,
        white,
    )));
    // Add the leftwall
    list.push(Box::new(Plane::plane(
        Vec3::new(0.0, 0.0, 1.0),
        20.0,
        white,
    )));

    // Add the rightwall
    list.push(Box::new(Plane::plane(
        Vec3::new(0.0, 0.0, 1.0),
        -20.0,
        white,
    )));

    //center sphere
    list.push(Box::new(Sphere::sphere(
        Vec3::new(10.0, 10.0, 0.0),
        5.0,
        red
    )));
    //light source sphere
    // list.push(Box::new(Sphere::sphere(
    //     Vec3::new(20.0, 40.0, 20.0),
    //     5.0,
    //     light
    // )));

    // list.push(Box::new(Plane::plane(
    //     Vec3::new(1.0, 0.0, 0.0),
    //     1.0,
    //     green
    // )));

    // list.push(Box::new(Plane::plane(
    //     Vec3::new(0.0, 1.0, 0.0),
    //     1.0,
    //     white
    // )));

    // // Add the floor
    // let floor = Rectangle {
    //     p0: Vec3::new(-100.0, 0.0, -100.0),
    //     p1: Vec3::new(-100.0, 0.0, 100.0),
    //     p2: Vec3::new(100.0, 0.0, 100.0),
    //     material: white.clone(),
    //     aabb: Aabb::new(Vec3::new(-100.0, 0.0, -100.0), Vec3::new(100.0, 0.0, 100.0)),
    // };
    // list.push(Box::new(floor));

    // // Add the ceiling
    // let ceiling = Rectangle {
    //     p0: Vec3::new(-100.0, 100.0, 100.0),
    //     p1: Vec3::new(-100.0, 100.0, -100.0),
    //     p2: Vec3::new(100.0, 100.0, -100.0),
    //     material: white.clone(),
    //     aabb: Aabb::new(Vec3::new(-100.0, 100.0, -100.0), Vec3::new(100.0, 100.0, 100.0)),
    // };
    // list.push(Box::new(ceiling));

    // // Add the walls
    // let left_wall = Rectangle {
    //     p0: Vec3::new(-100.0, 0.0, 100.0),
    //     p1: Vec3::new(-100.0, 0.0, -100.0),
    //     p2: Vec3::new(-100.0, 100.0, -100.0),
    //     material: green.clone(),
    //     aabb: Aabb::new(Vec3::new(-100.0, 0.0, -100.0), Vec3::new(-100.0, 100.0, 100.0)),
    // };
    // list.push(Box::new(left_wall));

    // let right_wall = Rectangle {
    //     p0: Vec3::new(100.0, 0.0, -100.0),
    //     p1: Vec3::new(100.0, 0.0, 100.0),
    //     p2: Vec3::new(100.0, 100.0, 100.0),
    //     material: red.clone(),
    //     aabb: Aabb::new(Vec3::new(100.0, 0.0, 100.0), Vec3::new(100.0, 100.0, 100.0)),
    // };
    // list.push(Box::new(right_wall));

    let world = HittableList::new(list);
    world
}

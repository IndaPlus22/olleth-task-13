use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::random_in_unit_sphere;

extern crate rand;

use std::f64::consts::PI;

use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        v_fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = v_fov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let w = Vec3::unit_vector(&(look_from - look_at));
        let u = Vec3::unit_vector(&Vec3::cross(&v_up, &w));
        let v = Vec3::cross(&w, &u);
        let lower_left_corner = look_from
            - half_width * u
            - half_height * v
            - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;
        Self {
            origin: look_from,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: 0.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_sphere();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::ray(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

// impl Camera {
//     fn random_in_unit_disk() -> Vec3 {
//         let mut rng = rand::thread_rng();
//         let mut p: Vec3;
//         while {
//             p = 2.0*Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
//             Vec3::dot(&p, &p) >= 1.0
//         } {}
//         p
//     }

//     pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, 
//         aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
//             let lens_radius: f64 = aperture/2.0;
    
//             let theta: f64 = vfov*PI/180.0;
//             let half_height: f64 = (theta/2.0).tan();
//             let half_width: f64 = aspect * half_height;
    
//             let origin: Vec3 = lookfrom;
            
//             let w: Vec3 = Vec3::unit_vector(&(lookfrom - lookat));
//             let u: Vec3 = Vec3::unit_vector(&Vec3::cross(&vup, &w));
//             let v: Vec3 = Vec3::cross(&w, &u);
            
//             let lower_left_corner = origin - half_width*focus_dist*u - half_height*focus_dist*v - focus_dist*w;
//             let horizontal: Vec3 = 2.0*half_width*focus_dist*u;
//             let vertical: Vec3 = 2.0*half_height*focus_dist*v;
            
//             Camera{lower_left_corner: lower_left_corner,
//                 horizontal: horizontal,
//                 vertical: vertical,
//                 origin: origin,
//                 lens_radius: lens_radius,
//                 u:u,
//                 v:v,
//                 w:w,
//                 }
//     }

//     pub fn get_ray(&self, u: f64, v: f64) -> Ray {
//         let rd: Vec3 = self.lens_radius * random_in_unit_sphere();
//         let offset: Vec3 = self.u * rd.x() + self.v * rd.y();
//         Ray::ray(self.origin + offset,
//                  self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin - offset)
//     }
// }
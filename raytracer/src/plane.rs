use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

pub struct Plane {
    pub normal: Vec3,
    pub distance: f64,
    pub material: Material,
}

impl Plane {
    pub fn plane(normal: Vec3, distance: f64, material: Material) -> Plane {
        Plane {
            normal,
            distance,
            material,
        }
    } 
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (Vec3::dot(&-ray.origin(), &self.normal) + self.distance) / Vec3::dot(&ray.direction(), &self.normal);
        if t < t_min || t > t_max {
            return None;
        }
        let p = ray.point_at_parameter(t);
        Some(HitRecord {
            t,
            p,
            normal: self.normal,
            material: self.material,
        })
    }
}

pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {  
    pub fn new(min: Vec3, max: Vec3 ) -> Aabb {
        Aabb {
            min,
            max,
        }
    }
    fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let t0 = min_max::min((self.min.e[a] - ray.origin().e[a]) / ray.direction().e[a], (self.max.e[a] - ray.origin().e[a]) / ray.direction().e[a]);
            let t1 = min_max::max((self.min.e[a] - ray.origin().e[a]) / ray.direction().e[a], (self.max.e[a] - ray.origin().e[a]) / ray.direction().e[a]);
            t_min = min_max::max(t0, t_min);
            t_max = min_max::min(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

pub struct Rectangle {
    pub p0: Vec3,
    pub p1: Vec3,
    pub p2: Vec3,
    pub material: Material,
    pub aabb: Aabb,
}

impl Rectangle {
    pub fn rectangle(
        p0: Vec3,
        p1: Vec3,
        p2: Vec3,
        material: Material,
        aabb: Aabb,
        ) -> Rectangle {
        Rectangle {
            p0,
            p1,
            p2,
            material,
            aabb,
        }
    }  
}

impl Hittable for Rectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None;
        }

        // Calculate the normal vector of the rectangle
        let normal = Vec3::unit_vector(&Vec3::cross(&(self.p1 - self.p0), &(self.p2 - self.p0)));

        // Check if the ray is parallel to the rectangle
        let t = Vec3::dot(&(self.p0 - ray.origin()), &normal) / Vec3::dot(&ray.direction(), &normal);
        if t < t_min || t > t_max {
            return None;
        }

        // Calculate the intersection point
        let p = ray.point_at_parameter(t);

        // Check if the intersection point is within the bounds of the rectangle
        Vec3::dot(&(self.p1 - self.p0), &(self.p2 - self.p0));
        let d1 = Vec3::dot(&(self.p1 - self.p0), &(p - self.p0));
        let d2 = Vec3::dot(&(self.p1 - self.p0), &(self.p2 - self.p0));
        if d1 < 0.0 || d2 < d1 {
            return None;
        }

        let d3 = Vec3::dot(&(self.p2 - self.p0), &(p - self.p0));
        let d4 = Vec3::dot(&(self.p2 - self.p0), &(self.p1 - self.p0));
        if d3 < 0.0 || d4 < d3 {
            return None;
        }

        // Return a hit record
        Some(HitRecord {
            t,
            p,
            normal,
            material: self.material,
        })
    }
}
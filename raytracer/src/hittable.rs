use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

//p = Point
#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        None
    }

}

impl HitRecord {
    
    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn set_t(&mut self, val: f64) {
        self.t = val
    }

    pub fn set_p(&mut self, val: Vec3){
        self.p = val
    }

    pub fn set_normal(&mut self, val: Vec3) {
        self.normal = val
    }

    pub fn set_material(&mut self, val: Material) {
        self.material = val
    }
}
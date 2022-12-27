use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

pub struct Triangle {
    vertices: [Vec3; 3],
    material: Material,
}

impl Triangle {
    pub fn triangle(vertices: [Vec3; 3], material: Material) -> Sphere {
        Triangle {
            vertices,
            material,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let e1 = self.vertices[1] - self.vertices[0];
        let e2 = self.vertices[2] - self.vertices[0];
        let h = ray.direction.cross(e2);
        let a = e1.dot(h);

        if a > -1e-6 && a < 1e-6 {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.vertices[0];
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(e1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * e2.dot(q);
        if t > 1e-6 {
            let hit_point = ray.point_at_parameter(t);
            let normal = self.get_normal();
            let material = self.material.clone();
            return Some(HitRecord { t, hit_point, normal, material });
        }

        None
    }

    fn get_normal(&self) -> Vector3 {
        (self.vertices[1] - self.vertices[0]).cross(self.vertices[2] - self.vertices[0]).normalize()
    }
}





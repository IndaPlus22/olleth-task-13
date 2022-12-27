use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Hittable for Aabb {
    fn hit(&self, ray: &Ray) -> bool {
        let mut t_min = (self.min.x - ray.origin.x) / ray.direction.x;
        let mut t_max = (self.max.x - ray.origin.x) / ray.direction.x;

        if t_min > t_max {
            std::mem::swap(&mut t_min, &mut t_max);
        }

        let t_min = t_min.max((self.min.y - ray.origin.y) / ray.direction.y);
        let t_max = t_max.min((self.max.y - ray.origin.y) / ray.direction.y);

        if t_min > t_max {
            return false;
        }

        let t_min = t_min.max((self.min.z - ray.origin.z) / ray.direction.z);
        let t_max = t_max.min((self.max.z - ray.origin.z) / ray.direction.z);

        t_max >= t_min
    }
}

struct Cube {
    bounds: Aabb,
    material: Material,
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        if self.bounds.hit(ray) {
            let t_min = (self.bounds.min.x - ray.origin.x) / ray.direction.x;
            let t_max = (self.bounds.max.x - ray.origin.x) / ray.direction.x;

            let t0 = t_min.min(t_max);
            let t1 = t_min.max(t_max);

            let t_min = (self.bounds.min.y - ray.origin.y) / ray.direction.y;
            let t_max = (self.bounds.max.y - ray.origin.y) / ray.direction.y;

            let t0 = t0.max(t_min.min(t_max));
            let t1 = t1.min(t_min.max(t_max));

            let t_min = (self.bounds.min.z - ray.origin.z) / ray.direction.z;
            let t_max = (self.bounds.max.z - ray.origin.z) / ray.direction.z;

            let t0 = t0.max(t_min.min(t_max));
            let t1 = t1.min(t_min.max(t_max));

            if t1 >= t0 {
                return Some(HitRecord { 
                    t: t0, 
                    p: ray.point_at_parameter(t0), 
                    normal: self.get_normal(hit_point), 
                    material: self.material, 
                });
            }
        }
        None
    }
}
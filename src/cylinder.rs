use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Cylinder {
    center: Point3,      // Center of the base
    radius: f64,         // Radius of the cylinder
    height: f64,         // Height of the cylinder (along Y-axis)
    mat: Rc<dyn Material>,
}

impl Cylinder {
    /// Create a cylinder with base at center and extending upward
    /// 
    /// # Arguments
    /// * `center` - The center of the base of the cylinder
    /// * `radius` - The radius of the cylinder
    /// * `height` - The height of the cylinder (extends along Y-axis)
    /// * `mat` - The material of the cylinder
    pub fn new(center: Point3, radius: f64, height: f64, mat: Rc<dyn Material>) -> Cylinder {
        Cylinder {
            center,
            radius: radius.abs(),
            height: height.abs(),
            mat,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let ray_origin = r.origin();
        let ray_dir = r.direction();

        // Vector from ray origin to cylinder center (at base)
        let oc = ray_origin - self.center;

        // Cylinder axis is along Y, so we project the ray and cylinder onto the XZ plane
        let a = ray_dir.x() * ray_dir.x() + ray_dir.z() * ray_dir.z();
        let b = 2.0 * (oc.x() * ray_dir.x() + oc.z() * ray_dir.z());
        let c = oc.x() * oc.x() + oc.z() * oc.z() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = f64::sqrt(discriminant);
        let mut t = (-b - sqrt_d) / (2.0 * a);

        // Check if the first intersection is within height bounds
        let hit_point = ray_origin + t * ray_dir;
        let height_offset = hit_point.y() - self.center.y();

        if t < t_min || t > t_max || height_offset < 0.0 || height_offset > self.height {
            // Try the second intersection
            t = (-b + sqrt_d) / (2.0 * a);
            let hit_point = ray_origin + t * ray_dir;
            let height_offset = hit_point.y() - self.center.y();

            if t < t_min || t > t_max || height_offset < 0.0 || height_offset > self.height {
                // Check bottom cap
                if ray_dir.y().abs() > 1e-8 {
                    let t_bottom = (self.center.y() - ray_origin.y()) / ray_dir.y();
                    if t_bottom >= t_min && t_bottom <= t_max {
                        let hit_point = ray_origin + t_bottom * ray_dir;
                        let dist_sq = (hit_point.x() - self.center.x()) * (hit_point.x() - self.center.x())
                            + (hit_point.z() - self.center.z()) * (hit_point.z() - self.center.z());
                        if dist_sq <= self.radius * self.radius {
                            rec.t = t_bottom;
                            rec.p = hit_point;
                            rec.set_face_normal(r, Vec3::new(0.0, -1.0, 0.0));
                            rec.mat = Some(self.mat.clone());
                            return true;
                        }
                    }
                }

                // Check top cap
                if ray_dir.y().abs() > 1e-8 {
                    let t_top = (self.center.y() + self.height - ray_origin.y()) / ray_dir.y();
                    if t_top >= t_min && t_top <= t_max {
                        let hit_point = ray_origin + t_top * ray_dir;
                        let dist_sq = (hit_point.x() - self.center.x()) * (hit_point.x() - self.center.x())
                            + (hit_point.z() - self.center.z()) * (hit_point.z() - self.center.z());
                        if dist_sq <= self.radius * self.radius {
                            rec.t = t_top;
                            rec.p = hit_point;
                            rec.set_face_normal(r, Vec3::new(0.0, 1.0, 0.0));
                            rec.mat = Some(self.mat.clone());
                            return true;
                        }
                    }
                }

                return false;
            }
        }

        rec.t = t;
        rec.p = ray_origin + t * ray_dir;

        // Calculate normal (perpendicular to cylinder axis on the curved surface)
        let outward_normal = (rec.p - Point3::new(self.center.x(), rec.p.y(), self.center.z())) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());
        true
    }
}

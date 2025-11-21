use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Cube {
    min: Point3,
    max: Point3,
    mat: Rc<dyn Material>,
}

impl Cube {
    /// Create a cube defined by two opposite corners
    /// 
    /// # Arguments
    /// * `min` - The minimum corner (x_min, y_min, z_min)
    /// * `max` - The maximum corner (x_max, y_max, z_max)
    /// * `mat` - The material of the cube
    pub fn new(min: Point3, max: Point3, mat: Rc<dyn Material>) -> Cube {
        Cube { min, max, mat }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut t_enter = t_min;
        let mut t_exit = t_max;
        let mut normal_axis = 0; // 0 for x, 1 for y, 2 for z
        let mut entering = true;

        let ray_origin = r.origin();
        let ray_dir = r.direction();

        // Check intersection with all three slab pairs (x, y, z)
        for axis in 0..3 {
            let min_val = if axis == 0 {
                self.min.x()
            } else if axis == 1 {
                self.min.y()
            } else {
                self.min.z()
            };

            let max_val = if axis == 0 {
                self.max.x()
            } else if axis == 1 {
                self.max.y()
            } else {
                self.max.z()
            };

            let ray_origin_val = if axis == 0 {
                ray_origin.x()
            } else if axis == 1 {
                ray_origin.y()
            } else {
                ray_origin.z()
            };

            let ray_dir_val = if axis == 0 {
                ray_dir.x()
            } else if axis == 1 {
                ray_dir.y()
            } else {
                ray_dir.z()
            };

            if ray_dir_val.abs() < 1e-8 {
                // Ray is parallel to slabs, check if origin is within
                if ray_origin_val < min_val || ray_origin_val > max_val {
                    return false;
                }
            } else {
                let t1 = (min_val - ray_origin_val) / ray_dir_val;
                let t2 = (max_val - ray_origin_val) / ray_dir_val;

                let (t_near, t_far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

                if t_near > t_enter {
                    t_enter = t_near;
                    normal_axis = axis;
                    entering = ray_dir_val < 0.0;
                }

                if t_far < t_exit {
                    t_exit = t_far;
                }

                if t_enter >= t_exit {
                    return false;
                }
            }
        }

        if t_enter < t_min || t_enter > t_max {
            return false;
        }

        rec.t = t_enter;
        rec.p = r.at(rec.t);

        // Calculate normal based on which face was hit
        let normal = match normal_axis {
            0 => {
                // X-axis face
                if entering {
                    Vec3::new(-1.0, 0.0, 0.0)
                } else {
                    Vec3::new(1.0, 0.0, 0.0)
                }
            }
            1 => {
                // Y-axis face
                if entering {
                    Vec3::new(0.0, -1.0, 0.0)
                } else {
                    Vec3::new(0.0, 1.0, 0.0)
                }
            }
            _ => {
                // Z-axis face
                if entering {
                    Vec3::new(0.0, 0.0, -1.0)
                } else {
                    Vec3::new(0.0, 0.0, 1.0)
                }
            }
        };

        rec.set_face_normal(r, normal);
        rec.mat = Some(self.mat.clone());
        true
    }
}

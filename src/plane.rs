use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

pub struct Plane {
    point: Point3,      // A point on the plane
    normal: Vec3,       // Normal vector of the plane
    mat: Rc<dyn Material>,
}

impl Plane {
    /// Create a plane defined by a point and a normal vector
    /// 
    /// # Arguments
    /// * `point` - A point that lies on the plane
    /// * `normal` - The normal vector (perpendicular to the plane)
    /// * `mat` - The material of the plane
    pub fn new(point: Point3, normal: Vec3, mat: Rc<dyn Material>) -> Plane {
        Plane {
            point,
            normal: vec3::unit_vector(normal),
            mat,
        }
    }

    /// Create a horizontal plane at a given height (Y-axis)
    /// 
    /// # Arguments
    /// * `height` - The Y coordinate of the plane
    /// * `mat` - The material of the plane
    pub fn horizontal(height: f64, mat: Rc<dyn Material>) -> Plane {
        Plane {
            point: Point3::new(0.0, height, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            mat,
        }
    }

    /// Create a vertical plane perpendicular to the Z-axis
    /// 
    /// # Arguments
    /// * `z_position` - The Z coordinate of the plane
    /// * `mat` - The material of the plane
    pub fn vertical_z(z_position: f64, mat: Rc<dyn Material>) -> Plane {
        Plane {
            point: Point3::new(0.0, 0.0, z_position),
            normal: Vec3::new(0.0, 0.0, 1.0),
            mat,
        }
    }

    /// Create a vertical plane perpendicular to the X-axis
    /// 
    /// # Arguments
    /// * `x_position` - The X coordinate of the plane
    /// * `mat` - The material of the plane
    pub fn vertical_x(x_position: f64, mat: Rc<dyn Material>) -> Plane {
        Plane {
            point: Point3::new(x_position, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 0.0),
            mat,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let ray_origin = r.origin();
        let ray_dir = r.direction();
        
        // Denominator: dot product of ray direction and plane normal
        let denom = vec3::dot(ray_dir, self.normal);
        
        // If denominator is close to 0, ray is parallel to plane
        if denom.abs() < 1e-8 {
            return false;
        }
        
        // Calculate t: (point - origin) · normal / (direction · normal)
        let t = vec3::dot(self.point - ray_origin, self.normal) / denom;
        
        // Check if t is within the acceptable range
        if t < t_min || t > t_max {
            return false;
        }
        
        rec.t = t;
        rec.p = r.at(rec.t);
        rec.set_face_normal(r, self.normal);
        rec.mat = Some(self.mat.clone());
        true
    }
}

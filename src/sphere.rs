
use crate::Point3;
use crate::vec3::dot;
use crate::hit_record::{Hittable, HitRecord};

pub struct Sphere
{
    center: Point3,
    radius: f64,
}

impl Sphere
{
    fn new(center: Point3, radius: f64) -> Sphere
    {
        Sphere
        {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere
{
    fn hit(&self, r: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool
    {
        let origin_to_center = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), origin_to_center);
        let c = origin_to_center.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0
        {
            return false
        } 
        let square_rooted = discriminant.sqrt();
        let mut root = (h - square_rooted) / a;
        if root <= ray_tmin || ray_tmax <= root
        {
            root = (h + square_rooted) / a;
            if root <= ray_tmin || ray_tmax <= root
            {
                return false;
            }
        } 

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        
        true
    }
}
use crate::interval::Interval;
use crate::Point3;
use crate::Ray;
use crate::vec3::{Vec3, dot};

#[derive(Clone, Default)]
pub struct HitRecord
{
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord
{
    pub fn new() -> HitRecord
    {
        Default::default()
    }

    // Sets the hit record normal vector. 
    // NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3)
    {
        self.front_face = dot(r.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal }
    }
}



pub trait Hittable
{
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
}
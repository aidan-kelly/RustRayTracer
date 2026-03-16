use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_unit_vector, reflect};

pub trait Material
{
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered_ray: &mut Ray) -> bool
    {
        false
    }
}

pub struct Lambertian
{
    albedo: Color,
}

impl Lambertian
{
    pub fn new(albedo: Color) -> Lambertian
    {
        Lambertian { albedo }
    }
}

impl Material for Lambertian
{
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered_ray: &mut Ray) -> bool
    {
        let mut scatter_direction = record.normal + random_unit_vector();

        if scatter_direction.near_zero()
        {
            scatter_direction = record.normal;
        }
        *scattered_ray = Ray::new(record.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal
{
    albedo: Color,
}

impl Metal
{
    pub fn new(albedo: Color) -> Metal
    {
        Metal { albedo }
    }
}

impl Material for Metal
{
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered_ray: &mut Ray) -> bool
    {
        let reflected = reflect(ray_in.direction(), record.normal);
        *scattered_ray = Ray::new(record.p, reflected);
        *attenuation = self.albedo;
        true
    }
}

pub struct NoMaterial;
impl Material for NoMaterial
{
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered_ray: &mut Ray) -> bool
    {
        false
    }
}
use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_unit_vector, reflect, unit_vector, dot, refract};

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
    fuzz: f64,
}

impl Metal
{
    pub fn new(albedo: Color, fuzz: f64) -> Metal
    {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal
{
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered_ray: &mut Ray) -> bool
    {
        let mut reflected = reflect(ray_in.direction(), record.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        *scattered_ray = Ray::new(record.p, reflected);
        *attenuation = self.albedo;
        
        dot(scattered_ray.direction(), record.normal) > 0.0
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

pub struct Dielectric
{
    refraction_index: f64,
}

impl Dielectric
{
    pub fn new(refraction_index: f64) -> Dielectric
    {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric
{
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered_ray: &mut Ray) -> bool
    {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        
        let ri = if record.front_face { 1.0/self.refraction_index } else { self.refraction_index };

        let unit_direction = unit_vector(ray_in.direction());
        let refracted = refract(unit_direction, record.normal, ri);

        *scattered_ray = Ray::new(record.p, refracted);
        true
    }
}
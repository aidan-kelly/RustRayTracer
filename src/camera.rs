use crate::material::NoMaterial;
use crate::ray::Ray;
use crate::color::{Color, write_color};
use crate::hit_record::{Hittable, HitRecord};
use crate::Point3;
use crate::interval::Interval;
use crate::rtweekend::{INFINITY, degrees_to_radians, random_double};
use crate::vec3::{Vec3, cross, random_in_unit_disk, random_on_hemisphere, random_unit_vector, unit_vector};

use std::io;
use std::sync::Arc;

#[derive(Default)]
pub struct Camera
{
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_distance: f64,
    pixel_sample_scale: f64,        // Color scale factor for a sum of pixel samples
    image_height: i32,
    camera_center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera
{
    fn initialize(&mut self)
    {
        // Calculate the image height.
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        // Camera
        //let focal_length: f64 = (self.lookfrom - self.lookat).length();
        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta/2.0);
        let viewport_height: f64 = 2.0 * h * self.focus_distance;
        let viewport_width: f64 = viewport_height * (self.image_width as f64 / self.image_height as f64);
        
        self.camera_center = self.lookfrom;

        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(cross(self.vup, self.w));
        self.v = cross(self.w, self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: Vec3 = viewport_width * self.u;
        let viewport_v: Vec3 = viewport_height * -self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width.into();
        self.pixel_delta_v = viewport_v / self.image_height.into();

        // Calculate the location of the upper left pixel.
        let viewport_upper_left: Vec3 = 
            self.camera_center - (self.focus_distance * self.w) - viewport_u/2.0 - viewport_v/2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_distance * f64::tan(degrees_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn render(&mut self, world: &dyn Hittable)
    {
        Camera::initialize(self);

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height
        {
            eprint!("\rScanlines remaining: {}         ", (self.image_height-j));
            for i in 0..self.image_width
            {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel
                {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }

                
                write_color(&mut io::stdout(), self.pixel_sample_scale * pixel_color);
            }
        }
        eprintln!("\rDone!                                              ");
    }

    fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color
    {
        if depth <= 0
        {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec= HitRecord::new(Arc::new(NoMaterial));
        if world.hit(r, &Interval::new(0.001, INFINITY), &mut rec)
        {
            let mut scattered = Ray::new(Vec3::random(), Vec3::random());
            let mut attenuation = Color::new(0.0, 0.0, 0.0);

            if rec.material.scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * Self::ray_color(&scattered, depth-1, world);
            }
            else
            {
                return Color::new(0.0, 0.0, 0.0);   
            }

            //let direction = rec.normal + random_unit_vector();
            //let reflectance_percent: f64 = 0.5;
            //return reflectance_percent * Self::ray_color(&Ray::new(rec.p, direction), depth - 1, world);
        }

        let unit_direction: Vec3 = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0-a) *Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray
    {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc + ((i as f64 + offset.x()) * self.pixel_delta_u) + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 { self.camera_center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3
    {
        let p = random_in_unit_disk();
        self.camera_center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}

fn sample_square() -> Vec3
{
    Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
}
mod camera;
mod color;
mod hit_record;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::vec3::Point3;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::color::Color;

use std::sync::Arc;

fn main() 
{    
    // World 
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Dielectric::new(1.50));

    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0/9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world);
}

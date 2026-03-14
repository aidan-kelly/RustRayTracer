mod color;
mod hit_record;
mod ray;
mod sphere;
mod vec3;

use std::io;

use color::{Color, write_color};
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

use crate::vec3::{dot, unit_vector};

// See if a ray hits a sphere
fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64
{
    let origin_to_center: Vec3 = *center - r.origin();
    let a = r.direction().length_squared();
    let h = dot(r.direction(), origin_to_center);
    let c = origin_to_center.length_squared() - radius*radius;
    let discriminant = h*h - a*c;

    if discriminant < 0.0
    {
        -1.0
    } 
    else 
    {
        (h - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color
{
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0
    {
        let n = unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5*Color::new(n.x()+1.0, n.y()+1.0, n.z()+1.0);
    }

    let unit_direction: Vec3 = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0-a) *Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    // Calculate the image height.
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u: Vec3 = viewport_u / IMAGE_WIDTH.into();
    let pixel_delta_v: Vec3 = viewport_v / IMAGE_HEIGHT.into();

    // Calculate the location of the upper left pixel.
    let viewport_upper_left: Vec3 = 
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0; 

    let pixel00_loc: Vec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);



    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in 0..IMAGE_HEIGHT
    {
        eprint!("\rScanlines remaining: {}", (IMAGE_HEIGHT-j));
        for i in 0..IMAGE_WIDTH
        {
            let pixel_center: Vec3 = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray::new(camera_center, ray_direction); 
            let pixel_color = ray_color(&r);
            write_color(&mut io::stdout(), pixel_color);
        }
    }
    eprintln!("\rDone!                                              ");
}

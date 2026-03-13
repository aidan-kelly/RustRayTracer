use std::io;

use crate::{color::write_color, vec3::Vec3};

mod color;
mod vec3;

fn main() {
    let image_height: i32 = 256;
    let image_width: i32 = 256;

    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height
    {
        eprint!("\rScanlines remaining: {}", (image_height-j));
        for i in 0..image_width
        {
            let r = i as f64 / (image_width-1) as f64;
            let g = j as f64/ (image_height-1) as f64;
            let b = 0.0;

            let pixel_color = Vec3::new(r,g,b);
            write_color(&mut io::stdout(), pixel_color);
        }
    }
    eprintln!("\rDone!                                              ");
}

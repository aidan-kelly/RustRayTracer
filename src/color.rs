use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl std::io::Write, pixel_color: Color)
{
    // Translate teh [0,1] component values to the byte range [0,255]
    let r: i32 = (255.999 * pixel_color.x()) as i32;
    let g: i32 = (255.999 * pixel_color.y()) as i32;
    let b: i32 = (255.999 * pixel_color.z()) as i32;

    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}
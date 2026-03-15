use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl std::io::Write, pixel_color: Color)
{
    let intensity: Interval = Interval::new(0.000, 0.999);

    // Translate teh [0,1] component values to the byte range [0,255]
    let r: i32 = (256.0 * intensity.clamp(pixel_color.x())) as i32;
    let g: i32 = (256.0 * intensity.clamp(pixel_color.y())) as i32;
    let b: i32 = (256.0 * intensity.clamp(pixel_color.z())) as i32;

    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}
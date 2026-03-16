use crate::rtweekend::{random_double, random_double_range};

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3
{
    e: [f64; 3],
}

impl Vec3
{
    pub fn new(x: f64, y: f64, z: f64) -> Vec3
    {
        Vec3{e: [x,y,z]}
    }

    pub fn x(&self) -> f64
    {
        self.e[0]
    }

    pub fn y(&self) -> f64
    {
        self.e[1]
    }

    pub fn z(&self) -> f64
    {
        self.e[2]
    }

    pub fn length(&self) -> f64
    {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64
    {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn random() -> Vec3
    {
        Vec3::new(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3
    {
        Vec3::new(random_double_range(min, max), random_double_range(min, max), random_double_range(min, max))
    } 

    pub fn near_zero(&self) -> bool
    {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

}

pub type Point3 = Vec3;

// Output
impl std::fmt::Display for Vec3
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

// Vec3 += Vec3
impl std::ops::AddAssign for Vec3
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

// Vec3 *= Vec3
impl std::ops::MulAssign<f64> for Vec3
{
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

// Vec3 /= Vec3
impl std::ops::DivAssign<f64> for Vec3
{
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

// -Vec3
impl std::ops::Neg for Vec3
{
    type Output = Self;

    fn neg(self) -> Self
    {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

// Vec3 + Vec3
impl std::ops::Add for Vec3
{
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

// Vec3 - Vec3
impl std::ops::Sub for Vec3
{
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

// Vec3 * Vec3
impl std::ops::Mul for Vec3
{
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

// f64 * Vec3
impl std::ops::Mul<Vec3> for f64
{
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

// Vec3 * f64
impl std::ops::Mul<f64> for Vec3
{
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

// Vec3 / f64
impl std::ops::Div<f64> for Vec3
{
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0/rhs) * self
    }
}

// Dot Product
pub fn dot(u: Vec3, v: Vec3) -> f64
{
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

// Cross Product
pub fn cross(u: Vec3, v: Vec3) -> Vec3
{
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

// Unit Vector
pub fn unit_vector(v: Vec3) -> Vec3
{
    v / v.length()
}

pub fn random_unit_vector() -> Vec3
{
    loop
    {
        let p = Vec3::random_range(-1.0, 1.0);
        let length_squared = p.length_squared();
        if 1e-160 < length_squared && length_squared <= 1.0
        {
            return p / length_squared.sqrt()
        } 
    }
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3
{
    let on_unit_sphere = random_unit_vector();
    if dot(on_unit_sphere, normal) > 0.0 //in same hemisphere as the normal
    {
        on_unit_sphere
    }
    else
    {
        -on_unit_sphere
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3
{
    v - 2.0 * dot(v,n) * n
}
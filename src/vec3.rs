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
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
}

impl std::ops::Neg for Vec3
{
    type Output = Self;

    fn neg(self) -> Self
    {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl std::ops::AddAssign for Vec3
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::MulAssign<f64> for Vec3
{
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign<f64> for Vec3
{
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl std::ops::Add for Vec3
{
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl std::ops::Sub for Vec3
{
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl std::ops::Mul for Vec3
{
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl std::ops::Mul<Vec3> for f64
{
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl std::ops::Mul<f64> for Vec3
{
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f64> for Vec3
{
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        (1.0/rhs) * self
    }
}
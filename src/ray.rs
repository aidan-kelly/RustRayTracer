pub struct Ray
{
    origin: Point3,
    direction: Vec3,
}

impl Ray
{
    pub fn New(o: Point3, d: Vec3) -> Ray
    {
        Ray{origin: o, direction: d}
    }

    pub fn Origin(&self) -> Point3
    {
        self.origin
    }

    pub fn Direction(&self) -> Vec3
    {
        self.direction
    }

    pub fn At(t: f64) -> Point3
    {
        origin + t*direction
    }
}
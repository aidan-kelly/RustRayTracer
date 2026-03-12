struct Vec3
{
    e: [f64; 3],
}

impl Vec3
{
    fn new(x: f64, y: f64, z: f64) -> Vec3
    {
        Vec3{e: [x,y,z]}
    }

    fn x(&self) -> f64
    {
        self.e[0]
    }

    fn y(&self) -> f64
    {
        self.e[1]
    }

    fn z(&self) -> f64
    {
        self.e[2]
    }
}
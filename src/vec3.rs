struct Vec3
{
    e: [f32; 3],
}

impl Vec3
{
    fn new(x: f32, y: f32, z: f32) -> Vec3
    {
        Vec3{e: [x,y,z]}
    }

    fn x(&self) -> f32
    {
        self.e[0]
    }

    fn y(&self) -> f32
    {
        self.e[1]
    }

    fn z(&self) -> f32
    {
        self.e[2]
    }
}
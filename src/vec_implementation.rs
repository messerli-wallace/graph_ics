pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x_new: f32, y_new: f32, z_new: f32) -> Self {
        Vec3 {
            x: x_new,
            y: y_new,
            z: z_new,
        }
    }

    pub fn x(self) -> f32 {
        self.x
    }

    pub fn y(self) -> f32 {
        self.y
    }

    pub fn z(self) -> f32 {
        self.z
    }

    pub fn destructure(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    pub fn plus(&mut self, v: Vec3) {
        self.x = self.x + v.x;
        self.y = self.y + v.y;
        self.z = self.z + v.z;
    }
}

use std::ops;

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

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn destructure(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    pub fn negative(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
        v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        let x = v1.y() * v2.z() - v1.z() * v2.y();
        let y = v1.z() * v2.x() - v1.x() * v2.z();
        let z = v1.x() * v2.y() - v1.y() * v2.x();
        Vec3::new(x, y, z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        let x = self.x + v.x;
        let y = self.y + v.y;
        let z = self.z + v.z;
        Vec3::new(x, y, z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        let x = self.x - v.x;
        let y = self.y - v.y;
        let z = self.z - v.z;
        Vec3::new(x, y, z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        let x = self.x * v.x;
        let y = self.y * v.y;
        let z = self.z * v.z;
        Vec3::new(x, y, z)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, v: Vec3) -> Vec3 {
        let x = self.x / v.x;
        let y = self.y / v.y;
        let z = self.z / v.z;
        Vec3::new(x, y, z)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        self.x = self.x + v.x;
        self.y = self.y + v.y;
        self.z = self.z + v.z;
    }
}

impl ops::AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, c: f32) {
        let c_v = Vec3::new(c, c, c);
        self.x = self.x + c_v.x;
        self.y = self.y + c_v.y;
        self.z = self.z + c_v.z;
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, v: Vec3) {
        self.x = self.x - v.x;
        self.y = self.y - v.y;
        self.z = self.z - v.z;
    }
}

impl ops::SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, c: f32) {
        let c_v = Vec3::new(c, c, c);
        self.x = self.x - c_v.x;
        self.y = self.y - c_v.y;
        self.z = self.z - c_v.z;
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, v: Vec3) {
        self.x = self.x * v.x;
        self.y = self.y * v.y;
        self.z = self.z * v.z;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, c: f32) {
        let c_v = Vec3::new(c, c, c);
        self.x = self.x * c_v.x;
        self.y = self.y * c_v.y;
        self.z = self.z * c_v.z;
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, v: Vec3) {
        self.x = self.x / v.x;
        self.y = self.y / v.y;
        self.z = self.z / v.z;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, c: f32) {
        let c_v = Vec3::new(c, c, c);
        self.x = self.x / c_v.x;
        self.y = self.y / c_v.y;
        self.z = self.z / c_v.z;
    }
}

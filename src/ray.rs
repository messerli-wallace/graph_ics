use crate::vec_implementation::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn at(&mut self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

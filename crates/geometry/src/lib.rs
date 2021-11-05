use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3f {
    data: [f32; 3],
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { data: [x, y, z] }
    }

    pub fn len_sqr(self) -> f32 {
        self.dot(self)
    }

    pub fn len(self) -> f32 {
        self.len_sqr().sqrt()
    }

    pub fn dot(self, rhs: Vec3f) -> f32 {
        map2_array_3f(self.data, rhs.data, f32::mul).iter().sum()
    }

    pub fn cross(self, rhs: Vec3f) -> Vec3f {
        let x = self[1] * rhs[2] - self[2] * rhs[1];
        let y = self[2] * rhs[0] - self[0] * rhs[2];
        let z = self[0] * rhs[1] - self[1] * rhs[0];
        Self::new(x, y, z)
    }
}

fn map2_array_3f<F: Fn(f32, f32) -> f32>(a: [f32; 3], b: [f32; 3], f: F) -> [f32; 3] {
    [f(a[0], b[0]), f(a[1], b[1]), f(a[2], b[2])]
}

impl Index<usize> for Vec3f {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl IndexMut<usize> for Vec3f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl Add for Vec3f {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: map2_array_3f(self.data, rhs.data, f32::add),
        }
    }
}

impl Neg for Vec3f {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            data: [-self[0], -self[1], -self[2]],
        }
    }
}

impl Sub for Vec3f {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            data: map2_array_3f(self.data, rhs.data, f32::sub),
        }
    }
}

impl Mul<f32> for Vec3f {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            data: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}

impl Mul<Vec3f> for f32 {
    type Output = Vec3f;
    fn mul(self, rhs: Vec3f) -> Self::Output {
        rhs * self
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec2f {
    data: [f32; 2],
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Self { data: [x, y] }
    }

    pub fn len_sqr(self) -> f32 {
        self.dot(self)
    }

    pub fn len(self) -> f32 {
        self.len_sqr().sqrt()
    }

    pub fn dot(self, rhs: Self) -> f32 {
        map2_array_2f(self.data, rhs.data, f32::mul).iter().sum()
    }
}

fn map2_array_2f<F: Fn(f32, f32) -> f32>(a: [f32; 2], b: [f32; 2], f: F) -> [f32; 2] {
    [f(a[0], b[0]), f(a[1], b[1])]
}

impl Index<usize> for Vec2f {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl IndexMut<usize> for Vec2f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl Add for Vec2f {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: map2_array_2f(self.data, rhs.data, f32::add),
        }
    }
}

impl Neg for Vec2f {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            data: [-self[0], -self[1]],
        }
    }
}

impl Sub for Vec2f {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            data: map2_array_2f(self.data, rhs.data, f32::sub),
        }
    }
}

impl Mul<f32> for Vec2f {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            data: [self[0] * rhs, self[1] * rhs],
        }
    }
}

impl Mul<Vec2f> for f32 {
    type Output = Vec2f;
    fn mul(self, rhs: Vec2f) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        let expected = 4;
        assert_eq!(result, expected);
    }
}

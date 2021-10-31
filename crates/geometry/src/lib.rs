use std::ops::{Add, Index, Mul, Sub};

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

impl Add for Vec3f {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: map2_array_3f(self.data, rhs.data, f32::add),
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

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        let expected = 4;
        assert_eq!(result, expected);
    }
}

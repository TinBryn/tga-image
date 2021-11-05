#![warn(missing_docs)]

//! # Some basic vectors for 2d and 3d maths,
//! Only 2d and 3d are supported, although other dimensions are possible
//!
//! The use of [`Copy`](core::marker::Copy) as a bound is used as it is expected that
//! primitive like types will be used (`f32` `usize`, etc)

use std::{
    iter::Sum,
    mem::MaybeUninit,
    ops::{Add, Index, IndexMut, Mul, Neg, Sub},
};

/// A 3d vector of f32
pub type Vec3f = Vector<f32, 3>;
/// A 3d vector of f64
pub type Vec3d = Vector<f64, 3>;
/// A 3d vector of isize
pub type Vec3i = Vector<isize, 3>;
/// A 3d vector of usize
pub type Vec3u = Vector<usize, 3>;

/// A 2d vector of f32
pub type Vec2f = Vector<f32, 2>;
/// A 2d vector of f64
pub type Vec2d = Vector<f64, 2>;
/// A 2d vector of isize
pub type Vec2i = Vector<isize, 2>;
/// A 2d vector of usize
pub type Vec2u = Vector<usize, 2>;

/// A vector that supports some basic mathematical operations  
/// ---
/// these are currently only fully supported for 2d and 3d versions
#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        [T::default(); N].into()
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(data: [T; N]) -> Self {
        Self { data }
    }
}

impl<T, const N: usize> Vector<T, N> {
    /// Converts this vector into an array of the elements
    pub fn into_array(self) -> [T; N] {
        self.data
    }
}

impl<T> Vector<T, 2> {
    /// ## Constructor for 2d vector
    /// note: rust-analyzer doesn't support the const generics used here and reports
    /// a false positive if this constructor has the same name for different sized vectors
    pub fn new2(x: T, y: T) -> Self {
        [x, y].into()
    }
}

impl<T> Vector<T, 3> {
    /// ## Constructor for 3d vector
    /// note: rust-analyzer doesn't support the const generics used here and reports
    /// a false positive if this constructor has the same name for different sized vectors
    pub fn new3(x: T, y: T, z: T) -> Self {
        [x, y, z].into()
    }
}

// there is a nice std method `array::zip` but it's unstable so I can't use it on stable
fn zip_array<T: Copy, U: Copy, const N: usize>(a: [T; N], b: [U; N]) -> [(T, U); N] {
    let mut result = [MaybeUninit::uninit(); N];
    for i in 0..N {
        result[i] = MaybeUninit::new((a[i], b[i]));
    }
    result.map(|x| unsafe { x.assume_init() })
}

impl<T, const N: usize> Vector<T, N>
where
    T: Mul<Output = T> + Sum + Copy,
{
    /// The vector dot product
    pub fn dot(self, rhs: Self) -> T {
        (self * rhs).data.into_iter().sum()
    }

    /// the square of the magnitude of this vector
    pub fn len_sqr(self) -> T {
        self.dot(self)
    }
}

impl<const N: usize> Vector<f32, N> {
    /// the magnitude of this vector
    pub fn len(self) -> f32 {
        self.len_sqr().sqrt()
    }
}

impl<const N: usize> Vector<f64, N> {
    /// the magnitude of this vector
    pub fn len(self) -> f64 {
        self.len_sqr().sqrt()
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Copy> Vector<T, 3> {
    /// the vector cross product
    pub fn cross(self, rhs: Self) -> Self {
        let x = self[1] * rhs[2] - self[2] * rhs[1];
        let y = self[2] * rhs[0] - self[0] * rhs[2];
        let z = self[0] * rhs[1] - self[1] * rhs[0];
        [x, y, z].into()
    }
}

impl<T: Add<Output = T> + Copy, const N: usize> Add for Vector<T, N> {
    type Output = Self;
    /// Elementwise addition
    fn add(self, rhs: Self) -> Self::Output {
        zip_array(self.data, rhs.data).map(|(a, b)| a + b).into()
    }
}

impl<T: Sub<Output = T> + Copy, const N: usize> Sub for Vector<T, N> {
    type Output = Self;
    /// Elementwise subtraction
    fn sub(self, rhs: Self) -> Self::Output {
        zip_array(self.data, rhs.data).map(|(a, b)| a - b).into()
    }
}

impl<T: Mul<Output = T> + Copy, const N: usize> Mul for Vector<T, N> {
    type Output = Self;
    /// Elementwise multiplication
    fn mul(self, rhs: Self) -> Self::Output {
        zip_array(self.data, rhs.data).map(|(a, b)| a * b).into()
    }
}

impl<T: Copy + Mul<Output = T>, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.data.map(|lhs| lhs * rhs).into()
    }
}

impl<T: Copy + Neg<Output = T>, const N: usize> Neg for Vector<T, N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.data.map(T::neg).into()
    }
}

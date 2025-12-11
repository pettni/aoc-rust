use num_traits::NumAssignRef;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use std::ops::{Index, IndexMut};

use crate::dir::Dir;

// Trait alias

pub trait Scalar: Copy + NumAssignRef + Neg<Output = Self> + Sum {}
impl<T> Scalar for T where T: Copy + NumAssignRef + Neg<Output = T> + Sum {}

// Base type

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
pub struct Vector<const N: usize, T: Scalar> {
    data: [T; N],
}

// Generic API

impl<const N: usize, T: Scalar> Vector<N, T> {
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [T::zero(); N],
        }
    }
    pub fn ones() -> Self {
        Self {
            data: [T::one(); N],
        }
    }

    pub fn norm_sq(&self) -> T {
        self.data.iter().map(|x| *x * *x).sum::<T>()
    }

    pub fn dist_sq(&self, other: &Self) -> T {
        (*self - *other).norm_sq()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }
}

// Generic traits

impl<const N: usize, T: Scalar> Index<usize> for Vector<N, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize, T: Scalar> IndexMut<usize> for Vector<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize, T: Scalar> FromIterator<T> for Vector<N, T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ret = Self::zero();
        for (idx, x) in iter.into_iter().enumerate() {
            ret[idx] = x;
        }
        ret
    }
}

// Operator traits

impl<const N: usize, T: Scalar> Add for Vector<N, T> {
    type Output = Self;
    fn add(self, rhs: Vector<N, T>) -> Self::Output {
        self.data
            .iter()
            .zip(rhs.data)
            .map(|(a, b)| *a + b)
            .collect::<Vector<N, T>>()
    }
}

impl<const N: usize, T: Scalar> Add<T> for Vector<N, T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        self.data.iter().map(|a| *a + rhs).collect::<Vector<N, T>>()
    }
}

impl<const N: usize, T: Scalar> Neg for Vector<N, T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.data.iter().map(|a| -(*a)).collect::<Vector<N, T>>()
    }
}

impl<const N: usize, T: Scalar> Sub for Vector<N, T> {
    type Output = Self;
    fn sub(self, rhs: Vector<N, T>) -> Self::Output {
        self + (-rhs)
    }
}

impl<const N: usize, T: Scalar> AddAssign for Vector<N, T> {
    fn add_assign(&mut self, rhs: Vector<N, T>) {
        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(a, b)| *a += b);
    }
}

impl<const N: usize, T: Scalar> Mul<T> for Vector<N, T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.data.iter().map(|a| *a * rhs).collect::<Vector<N, T>>()
    }
}

impl<const N: usize, T: Scalar> Div<T> for Vector<N, T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self.data.iter().map(|a| *a / rhs).collect::<Vector<N, T>>()
    }
}

impl<const N: usize, T: Scalar> MulAssign<T> for Vector<N, T> {
    fn mul_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a *= rhs);
    }
}

impl<const N: usize, T: Scalar> DivAssign<T> for Vector<N, T> {
    fn div_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|a| *a /= rhs);
    }
}

// Specializations

pub type Vec2<T> = Vector<2, T>;

impl<T: Scalar> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Vec2::<T> { data: [x, y] }
    }

    pub fn x(self) -> T {
        self.data[0]
    }

    pub fn y(self) -> T {
        self.data[1]
    }

    pub fn x_mut(&mut self) -> &mut T {
        self.data.get_mut(0).unwrap()
    }

    pub fn y_mut(&mut self) -> &mut T {
        self.data.get_mut(1).unwrap()
    }

    pub fn cross(&self, other: Vec2<T>) -> T {
        self.x() * other.y() - self.y() * other.x()
    }
}

pub type Vec3<T> = Vector<3, T>;

impl<T: Scalar> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Vec3::<T> { data: [x, y, z] }
    }

    pub fn x(self) -> T {
        self.data[0]
    }

    pub fn y(self) -> T {
        self.data[1]
    }

    pub fn z(self) -> T {
        self.data[2]
    }
}

pub type Vec4<T> = Vector<4, T>;

impl<T: Scalar> Vec4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Vec4::<T> { data: [x, y, z, w] }
    }
    pub fn x(self) -> T {
        self.data[0]
    }

    pub fn y(self) -> T {
        self.data[1]
    }

    pub fn z(self) -> T {
        self.data[2]
    }

    pub fn w(self) -> T {
        self.data[3]
    }
}

pub type Vec2i = Vec2<i64>;
pub type Vec3i = Vec3<i64>;
pub type Vec4i = Vec4<i64>;

pub type Vec2d = Vec2<f64>;
pub type Vec3d = Vec3<f64>;
pub type Vec4d = Vec4<f64>;

impl Vec2i {
    /// Move in direction.
    /// This uses an "image" "x-east, y-south" coordinate system.
    pub fn step(&self, dir: Dir, d: i64) -> Self {
        let (dx, dy) = match dir {
            Dir::N => (0, -d),
            Dir::E => (d, 0),
            Dir::S => (0, d),
            Dir::W => (-d, 0),
            Dir::NE => (d, -d),
            Dir::SE => (d, d),
            Dir::SW => (-d, d),
            Dir::NW => (-d, -d),
        };
        Self::new(self.x() + dx, self.y() + dy)
    }

    /// Check if (x,y) is contained in [0, w)x(0, h)
    pub fn is_in_grid(&self, h: usize, w: usize) -> bool {
        self.x() >= 0 && self.x() < w as i64 && self.y() >= 0 && self.y() < h as i64
    }

    /// Get linear row-major index.
    pub fn linear_idx(&self, w: usize) -> usize {
        (self.y() * w as i64 + self.x()) as usize
    }

    /// Manhattan distance.
    pub fn manhattan_norm(&self) -> u64 {
        self.x().unsigned_abs() + self.y().unsigned_abs()
    }

    /// Manhattan distance.
    pub fn manhattan_dist(&self, other: &Vec2i) -> u64 {
        (*self - *other).manhattan_norm()
    }
}

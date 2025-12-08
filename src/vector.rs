use num_traits::NumAssignRef;
use std::iter::Sum;
use std::ops::{Add, Neg, Sub};
use std::ops::{Index, IndexMut};

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

// Specializations

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

pub type Vec3i = Vec3<i64>;

use crate::Answer;
use num_traits::Num;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
pub struct Vector<const N: usize, T: Num + Copy> {
    data: [T; N],
}

type Vec3<T> = Vector<3, T>;

impl<T: Num + Copy> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Vec3::<T> { data: [x, y, z] }
    }
}

type Vec3i = Vec3<i64>;

impl<const N: usize, T: Num + Copy> Vector<N, T> {
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
    pub fn ones() -> Self {
        Self {
            data: [T::one(); N],
        }
    }
}

impl<const N: usize, T: Num + Copy> Index<usize> for Vector<N, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize, T: Num + Copy> IndexMut<usize> for Vector<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize, T: Num + Copy> FromIterator<T> for Vector<N, T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ret = Self::zero();
        for (i, x) in iter.enumerate() {
            ret[i] = x;
        }
        ret
    }
}

pub fn part_a(input: &str) -> Answer {
    let vs = input
        .lines()
        .map(|line| {
            let mut ns = line.split(',').map(|x| x.parse::<i64>().unwrap());
            let x = ns.next().unwrap();
            let y = ns.next().unwrap();
            let z = ns.next().unwrap();
            Vec3i::new(x, y, z)
        })
        .collect::<Vec<_>>();
    println!("{:?}", vs);
    Answer::default()
}

pub fn part_b(input: &str) -> Answer {
    let _ = input;
    Answer::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(40));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Unimplemented);
    }
}

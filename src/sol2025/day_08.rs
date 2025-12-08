use std::cmp::Ordering;

use crate::dsa::Dsa;
use crate::heap::MinHeap;
use crate::vector::Vec3i;
use crate::Answer;

type HeapEl = (usize, usize, i64);

fn common(
    input: &str,
) -> (
    Vec<Vec3i>,
    MinHeap<HeapEl, impl Fn(&HeapEl, &HeapEl) -> Ordering>,
) {
    let vs = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec3i>()
        })
        .collect::<Vec<_>>();

    let n = vs.len();

    let mut ds =
        MinHeap::with_capacity(n * (n + 1) / 2, |p1: &HeapEl, p2: &HeapEl| p1.2.cmp(&p2.2));
    for i1 in 0..n {
        for i2 in i1 + 1..n {
            ds.push((i1, i2, vs[i1].dist_sq(&vs[i2])))
        }
    }

    (vs, ds)
}

fn solve_part_a(input: &str, num_connections: usize) -> Answer {
    let (vs, mut ds) = common(input);
    let n = vs.len();

    let mut dsa = Dsa::new(n);

    for _ in 0..num_connections {
        if let Some((i1, i2, _)) = ds.pop() {
            dsa.merge(i1, i2);
        }
    }

    let mut circuit_sizes = dsa.sizes().map(|(_, s)| s).collect::<Vec<_>>();
    circuit_sizes.sort();
    let result = circuit_sizes.iter().rev().take(3).product::<usize>();

    Answer::Number(result as i64)
}

pub fn part_a(input: &str) -> Answer {
    solve_part_a(input, 1000)
}

pub fn part_b(input: &str) -> Answer {
    let (vs, mut ds) = common(input);
    let n = vs.len();

    let mut dsa = Dsa::new(n);

    while let Some((i1, i2, _)) = ds.pop() {
        dsa.merge(i1, i2);
        if dsa.len() == 1 {
            let result = vs[i1].x() * vs[i2].x();
            return Answer::Number(result);
        }
    }

    unreachable!("Not connected");
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
        let result = solve_part_a(TEST_INPUT, 10);
        assert_eq!(result, Answer::Number(40));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(25272));
    }
}

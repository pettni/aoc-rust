use crate::vector::Vec2i;
use crate::Answer;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::{max, min};

pub fn part_a(input: &str) -> Answer {
    let coords = input
        .lines()
        .map(|row| {
            row.split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec2i>()
        })
        .collect::<Vec<_>>();

    let max_area = coords
        .par_iter()
        .enumerate()
        .filter_map(|(i1, p1)| {
            coords
                .iter()
                .skip(i1 + 1)
                .map(|p2| {
                    let v = *p2 - *p1;
                    (v.x().abs() + 1) * (v.y().abs() + 1)
                })
                .max()
        })
        .max();

    Answer::Number(max_area.unwrap())
}

#[derive(Clone, Debug)]
struct Line {
    p0: Vec2i,
    p1: Vec2i,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.p1.x() == self.p0.x()
    }

    fn xmin(&self) -> i64 {
        min(self.p0.x(), self.p1.x())
    }

    fn xmax(&self) -> i64 {
        max(self.p0.x(), self.p1.x())
    }

    fn ymin(&self) -> i64 {
        min(self.p0.y(), self.p1.y())
    }

    fn ymax(&self) -> i64 {
        max(self.p0.y(), self.p1.y())
    }
}

/// Custom intersection function that tells whether l2
/// protrutes to the inside of l1.
fn intersects(l1: &Line, l2: &Line) -> bool {
    let xmin = max(l1.xmin(), l2.xmin());
    let xmax = min(l1.xmax(), l2.xmax());
    let ymin = max(l1.ymin(), l2.ymin());
    let ymax = min(l1.ymax(), l2.ymax());

    if xmin > xmax || ymin > ymax {
        return false;
    }

    let isec = Vec2i::new(xmin, ymin);

    if isec == l1.p1 {
        return false;
    }

    if (isec - l1.p0).cross(l2.p0 - l1.p0) < 0 {
        return true;
    }

    if (isec - l1.p0).cross(l2.p1 - l1.p0) < 0 {
        return true;
    }

    false
}

pub fn part_b(input: &str) -> Answer {
    let coords = input
        .lines()
        .map(|row| {
            row.split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec2i>()
        })
        .collect::<Vec<_>>();

    let (mut vlines, mut hlines): (Vec<Line>, Vec<Line>) = coords
        .iter()
        .cycle()
        .tuple_windows()
        .take(coords.len())
        .map(|(p0, p1)| Line { p0: *p0, p1: *p1 })
        .partition(|l| l.is_vertical());

    vlines.sort_by_key(|l| l.xmin());
    hlines.sort_by_key(|l| l.ymin());

    let max_area = coords
        .par_iter()
        .enumerate()
        .filter_map(|(i1, p1)| {
            coords
                .iter()
                .skip(i1 + 1)
                .filter_map(|p2| {
                    let x0 = min(p1.x(), p2.x());
                    let x1 = max(p1.x(), p2.x());
                    let y0 = min(p1.y(), p2.y());
                    let y1 = max(p1.y(), p2.y());

                    let hlines_min = hlines.partition_point(|l| l.ymin() <= y0);
                    let hlines_max = hlines.partition_point(|l| l.ymin() <= y1);

                    let vlines_min = vlines.partition_point(|l| l.xmin() <= x0);
                    let vlines_max = vlines.partition_point(|l| l.xmin() <= x1);

                    let perimiter = [
                        Vec2i::new(x0, y0),
                        Vec2i::new(x0, y1),
                        Vec2i::new(x1, y1),
                        Vec2i::new(x1, y0),
                    ];

                    let has_isect =
                        perimiter
                            .iter()
                            .cycle()
                            .tuple_windows()
                            .take(4)
                            .any(|(p0, p1)| {
                                let line = Line { p0: *p0, p1: *p1 };
                                if line.is_vertical() {
                                    &hlines[hlines_min..hlines_max]
                                } else {
                                    &vlines[vlines_min..vlines_max]
                                }
                                .iter()
                                .any(|x| intersects(&line, x))
                            });

                    if has_isect {
                        return None;
                    }
                    let v = *p2 - *p1;
                    Some((v.x().abs() + 1) * (v.y().abs() + 1))
                })
                .max()
        })
        .max();

    Answer::Number(max_area.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    "};

    const TEST_INPUT_MAN: &str = indoc! {"
        1,1
        10,1
        10,3
        12,3
        12,6
        10,6
        10,10
        1,10
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(50));
    }

    #[test]
    fn test_part_a_man() {
        let result = part_a(TEST_INPUT_MAN);
        assert_eq!(result, Answer::Number(100));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(24));
    }

    #[test]
    fn test_part_b_man() {
        let result = part_b(TEST_INPUT_MAN);
        assert_eq!(result, Answer::Number(100));
    }

    #[test]
    fn test_intersects() {
        let line = Line {
            p0: Vec2i::new(1, 1),
            p1: Vec2i::new(1, 8),
        };

        assert!(!intersects(
            &line,
            &Line {
                p0: Vec2i::new(1, 1),
                p1: Vec2i::new(5, 1)
            }
        ));

        assert!(intersects(
            &line,
            &Line {
                p0: Vec2i::new(1, 3),
                p1: Vec2i::new(5, 3)
            }
        ));

        assert!(!intersects(
            &line,
            &Line {
                p0: Vec2i::new(1, 3),
                p1: Vec2i::new(-3, 3)
            }
        ));

        assert!(!intersects(
            &line,
            &Line {
                p0: Vec2i::new(1, 8),
                p1: Vec2i::new(8, 8)
            }
        ));

        assert!(!intersects(
            &line,
            &Line {
                p0: Vec2i::new(1, 8),
                p1: Vec2i::new(-8, 8)
            }
        ));
    }
}

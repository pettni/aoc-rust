use crate::Answer;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Ival {
    lo: i64,
    hi: i64,
}

impl Ival {
    fn from_str(data: &str) -> Self {
        let mut spl = data.split("-").map(|x| x.parse::<i64>().unwrap());
        Ival {
            lo: spl.next().unwrap(),
            hi: spl.next().unwrap(),
        }
    }

    fn len(&self) -> i64 {
        std::cmp::max(0, self.hi - self.lo + 1)
    }

    fn contains(self, x: i64) -> bool {
        self.lo <= x && x <= self.hi
    }

    fn overlap(self, other: &Ival) -> bool {
        std::cmp::max(self.lo, other.lo) <= std::cmp::min(self.hi, other.hi)
    }

    fn merge(self, other: &Ival) -> Ival {
        if !self.overlap(other) {
            unreachable!("Tried to merge non-overlapping interval");
        }

        Ival {
            lo: std::cmp::min(self.lo, other.lo),
            hi: std::cmp::max(self.hi, other.hi),
        }
    }
}

pub fn part_a(input: &str) -> Answer {
    let mut spl = input.trim().split("\n\n");
    let ranges = spl
        .next()
        .unwrap()
        .lines()
        .map(Ival::from_str)
        .collect::<Vec<_>>();
    let num_fresh = spl
        .next()
        .unwrap()
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .filter(|x| ranges.iter().any(|r| r.contains(*x)))
        .count();

    Answer::Number(num_fresh as i64)
}

pub fn part_b(input: &str) -> Answer {
    let mut spl = input.trim().split("\n\n");
    let mut ranges = spl
        .next()
        .unwrap()
        .lines()
        .map(Ival::from_str)
        .collect::<Vec<_>>();

    ranges.sort();

    let fold_init: (i64, Ival) = (0, Ival { lo: 0, hi: -1 });
    let folder = |(res, cur_ival): (i64, Ival), ival: &Ival| match cur_ival.overlap(ival) {
        true => (res, cur_ival.merge(ival)),
        false => (res + cur_ival.len(), *ival),
    };
    let (num_fresh, last_ival) = ranges.iter().fold(fold_init, folder);
    Answer::Number(num_fresh + last_ival.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(3));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(14));
    }
}

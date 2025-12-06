use crate::hash::FxHashSet;
use crate::math::number_length;
use crate::Answer;

fn repeat_number(x: u64, k: u32) -> u64 {
    let mut ret = x;
    let num_digits = number_length(x);
    for _ in 0..k - 1 {
        ret *= 10u64.pow(num_digits as u32);
        ret += x;
    }
    ret
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Range {
    from: u64,
    to: u64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct InvalidNumberIterator {
    k: u32,
    rep_number: u64,
    end: u64,
}

impl Range {
    pub fn from_str(str: &str) -> Self {
        let mut spl = str.split("-").map(|x| x.parse::<u64>().unwrap());
        Range {
            from: spl.next().unwrap(),
            to: spl.next().unwrap(),
        }
    }

    pub fn iter(self, k: u32) -> InvalidNumberIterator {
        let num_digits = number_length(self.from) as u32;
        let mut rep_number = self.from / 10u64.pow((k - 1) * num_digits.div_ceil(k));
        loop {
            let x = repeat_number(rep_number, k);
            if x >= self.from {
                break;
            }
            rep_number += 1;
        }
        InvalidNumberIterator {
            k,
            rep_number,
            end: self.to,
        }
    }
}

impl Iterator for InvalidNumberIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let x = repeat_number(self.rep_number, self.k);
        if x > self.end {
            return None;
        }
        self.rep_number += 1;
        Some(x)
    }
}

pub fn part_a(input: &str) -> Answer {
    Answer::Number(
        input
            .trim()
            .split(",")
            .map(Range::from_str)
            .flat_map(|range: Range| range.iter(2))
            .sum::<u64>() as i64,
    )
}

pub fn part_b(input: &str) -> Answer {
    let unique_numbers = input
        .trim()
        .split(",")
        .map(Range::from_str)
        .flat_map(|range| {
            let k_to = number_length(range.to) as u32;
            (2..k_to + 1).flat_map(move |k| range.iter(k))
        })
        .collect::<FxHashSet<u64>>();

    Answer::Number(unique_numbers.iter().sum::<u64>() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(1227775554));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(4174379265));
    }
}

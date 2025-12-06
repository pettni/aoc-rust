use crate::math::number_length;
use crate::Answer;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Range {
    from: u64,
    to: u64,
}

impl Range {
    pub fn from_str(str: &str) -> Self {
        let mut spl = str.split("-").map(|x| x.parse::<u64>().unwrap());
        Range {
            from: spl.next().unwrap(),
            to: spl.next().unwrap(),
        }
    }

    pub fn contains(self, x: u64) -> bool {
        x >= self.from && x <= self.to
    }
}

fn get_half_number(x: u64) -> u64 {
    let half_num_digits = (number_length(x) + 1).checked_div(2).unwrap();
    x / 10u64.pow(half_num_digits as u32)
}

pub fn part_a(input: &str) -> Answer {
    let mut result = 0;

    let range_analyzer = |range: Range| {
        let mut half_number = get_half_number(range.from);
        loop {
            let num_digits = number_length(half_number) as u32;
            let x = half_number * 10u64.pow(num_digits) + half_number;

            if range.contains(x) {
                result += x;
            } else if x > range.to {
                break;
            }

            half_number += 1;
        }
    };

    input
        .trim()
        .split(",")
        .map(Range::from_str)
        .for_each(range_analyzer);

    Answer::Number(result as i64)
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
        assert_eq!(result, Answer::Unimplemented);
    }
}

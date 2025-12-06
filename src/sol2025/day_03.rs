use crate::parsing::{char_to_u32, parse_vector};
use crate::Answer;

fn find_largest_num(ns: &[u32], k: usize) -> u64 {
    if ns.len() < k {
        unreachable!("ns has less than k numbers");
    }

    let n = ns.len();
    let mut ret = 0;
    let mut kk = k;
    for idx in 0..n {
        if kk == 0 {
            return ret;
        }
        let max_in_tail = ns[idx..n - kk + 1].iter().max().unwrap();
        if ns[idx] >= *max_in_tail {
            ret = 10 * ret + ns[idx] as u64;
            kk -= 1;
        }
    }
    ret
}

pub fn part_a(input: &str) -> Answer {
    Answer::Number(
        input
            .lines()
            .map(|x| parse_vector(x, &char_to_u32).unwrap().1)
            .map(|ns| find_largest_num(&ns, 2))
            .sum::<u64>() as i64,
    )
}

pub fn part_b(input: &str) -> Answer {
    Answer::Number(
        input
            .lines()
            .map(|x| parse_vector(x, &char_to_u32).unwrap().1)
            .map(|ns| find_largest_num(&ns, 12))
            .sum::<u64>() as i64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(357));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(3121910778619));
    }
}

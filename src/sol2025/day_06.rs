use crate::parsing::{parse_row_of_ints, parse_row_of_x};
use crate::Answer;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Plus,
    Times,
}

impl Op {
    fn from_str(data: &str) -> Result<Self, String> {
        match data {
            "+" => Ok(Op::Plus),
            "*" => Ok(Op::Times),
            _ => Err("Invalid".to_string()),
        }
    }

    fn id(self) -> i64 {
        match self {
            Op::Plus => 0,
            Op::Times => 1,
        }
    }

    fn op(self, a: i64, b: i64) -> i64 {
        match self {
            Op::Plus => a + b,
            Op::Times => a * b,
        }
    }
}

pub fn part_a(input: &str) -> Answer {
    let lines = input.lines().collect::<Vec<_>>();
    let numbers = lines[0..lines.len() - 1]
        .iter()
        .map(|x| parse_row_of_ints(x).unwrap().1)
        .collect::<Vec<_>>();
    let ops = parse_row_of_x(lines[lines.len() - 1], &Op::from_str)
        .unwrap()
        .1;

    let result = (0..ops.len())
        .map(|i| {
            numbers
                .iter()
                .map(|ns| ns[i])
                .fold(ops[i].id(), |a, b| ops[i].op(a, b))
        })
        .sum();

    Answer::Number(result)
}

pub fn part_b(input: &str) -> Answer {
    let lines = input.lines().collect::<Vec<_>>();
    let numbers = lines[0..lines.len() - 1]
        .iter()
        .map(|x| x.chars().map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let w = numbers.iter().map(|x| x.len()).max().unwrap();

    let ops = parse_row_of_x(lines[lines.len() - 1], &Op::from_str)
        .unwrap()
        .1;

    let mut ops_idx = 0;
    let mut total_result = 0;
    let mut op_result = ops[ops_idx].id();

    (0..w).for_each(|hidx| {
        let vert_num = numbers.iter().filter_map(|ns| ns.get(hidx)).fold(
            None,
            |cur_opt, num_opt| match (cur_opt, num_opt) {
                (Some(cur), Some(num)) => Some(cur * 10 + *num as i64),
                (None, Some(num)) => Some(*num as i64),
                (Some(cur), None) => Some(cur),
                (None, None) => None,
            },
        );
        match vert_num {
            None => {
                total_result += op_result;
                ops_idx += 1;
                op_result = ops[ops_idx].id();
            }
            Some(x) => op_result = ops[ops_idx].op(op_result, x),
        }
    });

    Answer::Number(total_result + op_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        123 328  51 64
         45 64  387 23
          6 98  215 314
        *   +   *   +
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(4277556));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(3263827));
    }
}

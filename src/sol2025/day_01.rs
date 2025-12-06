use crate::Answer;

const START_POS: i64 = 50;
const MODULO: u64 = 100;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    L,
    R,
}

impl Dir {
    fn from_char(s: char) -> Self {
        match s {
            'L' => Dir::L,
            'R' => Dir::R,
            _ => unreachable!("Invalid dir {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Step {
    dir: Dir,
    len: u64,
}

impl Step {
    fn from_str(s: &str) -> Self {
        Step {
            dir: Dir::from_char(s.chars().nth(0).unwrap()),
            len: s[1..].parse::<u64>().unwrap(),
        }
    }

    fn num_turns(self) -> i64 {
        (self.len / MODULO) as i64
    }

    fn step(self, pos: i64) -> i64 {
        match self.dir {
            Dir::L => pos - (self.len % MODULO) as i64,
            Dir::R => pos + (self.len % MODULO) as i64,
        }
    }
}

fn parse(input: &str) -> Vec<Step> {
    input.lines().map(Step::from_str).collect::<Vec<_>>()
}


pub fn part_a(input: &str) -> Answer {
    let folder = |(zero_cnt, pos): (i64, i64), step: &Step| -> (i64, i64) {
        let new_pos = step.step(pos).rem_euclid(MODULO as i64);
        let hit_zero = new_pos == 0;
        (zero_cnt + hit_zero as i64, new_pos)
    };

    let (answer, _) = parse(input).iter().fold((0, START_POS), folder);
    Answer::Number(answer)
}

pub fn part_b(input: &str) -> Answer {
    let folder = |(zero_cnt, pos): (i64, i64), step: &Step| -> (i64, i64) {
        let new_pos = step.step(pos);
        let pass_zero = pos > 0 && (new_pos <= 0 || new_pos >= MODULO as i64);
        (
            zero_cnt + step.num_turns() + pass_zero as i64,
            new_pos.rem_euclid(MODULO as i64),
        )
    };

    let (answer, _) = parse(input).iter().fold((0, START_POS), folder);
    Answer::Number(answer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(3));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(6));
    }
}

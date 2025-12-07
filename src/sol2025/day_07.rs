use crate::parsing::{identity, parse_matrix};
use crate::Answer;

pub fn part_a(input: &str) -> Answer {
    let data = parse_matrix(input, &identity).unwrap().1;
    let n = data[0].len();
    let mut beams = vec![false; n];
    let mut new_beams = vec![false; n];

    let mut num_splits = 0;
    data.iter().for_each(|row| {
        new_beams.fill(false);
        for idx in 0..n {
            match (beams[idx], row[idx]) {
                (false, 'S') => {
                    new_beams[idx] = true;
                }
                (true, '.') => {
                    new_beams[idx] = true;
                }
                (true, '^') => {
                    num_splits += 1;
                    if let Some(el) = new_beams.get_mut(idx + 1) {
                        *el = true;
                    }
                    if let Some(el) = new_beams.get_mut(idx - 1) {
                        *el = true;
                    }
                }
                (false, _) => {}
                _ => unreachable!("Unexpected"),
            }
        }
        std::mem::swap(&mut beams, &mut new_beams);
    });

    Answer::Number(num_splits)
}

pub fn part_b(input: &str) -> Answer {
    let data = parse_matrix(input, &identity).unwrap().1;
    let n = data[0].len();
    let mut beams = vec![0; n];
    let mut new_beams = vec![0; n];

    data.iter().for_each(|row| {
        new_beams.fill(0);
        for idx in 0..n {
            match row[idx] {
                'S' => {
                    new_beams[idx] = 1;
                }
                '.' => {
                    new_beams[idx] += beams[idx];
                }
                '^' => {
                    if let Some(el) = new_beams.get_mut(idx + 1) {
                        *el += beams[idx];
                    }
                    if let Some(el) = new_beams.get_mut(idx - 1) {
                        *el += beams[idx];
                    }
                }
                _ => unreachable!("Unexpected"),
            }
        }
        std::mem::swap(&mut beams, &mut new_beams);
    });

    Answer::Number(beams.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(21));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(40));
    }
}

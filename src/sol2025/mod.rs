use crate::Solutions;

pub mod day_01;
pub mod day_02;
pub mod day_03;

pub mod day_xx;

pub const ALL: [Solutions; 3] = [
    (day_01::part_a, day_01::part_b),
    (day_02::part_a, day_02::part_b),
    (day_03::part_a, day_03::part_b),
];

#[cfg(test)]
mod tests {
    use crate::get_default_data_path;
    use std::{fs, path::PathBuf};

    use super::*;
    use crate::Answer;

    #[rustfmt::skip]
    const ANSWERS: [(Answer, Answer); 3] = [
        /* day 01 */ (Answer::Number(1195), Answer::Number(6770)),
        /* day 02 */ (Answer::Number(16793817782), Answer::Number(27469417404)),
        /* day 03 */ (Answer::Number(17443), Answer::Number(172167155440541)),
    ];

    #[test]
    #[ignore]
    fn test_all() {
        for day in 1..ALL.len() + 1 {
            let (part_a, part_b) = ALL
                .get(day.saturating_sub(1))
                .unwrap_or_else(|| panic!("Invalid day {}", day));

            let path: PathBuf = get_default_data_path(2025, day as u32);
            let data = fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("Couldn't open file {:?}", path));

            let (exp_a, exp_b) = &ANSWERS[day - 1];

            let out_a = part_a(data.as_str());
            assert_eq!(out_a, *exp_a);

            let out_b = part_b(data.as_str());
            assert_eq!(out_b, *exp_b);
        }
    }
}

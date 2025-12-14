use crate::hash::{FxHashMap, FxHashMapBuilder};
use crate::{Answer, vector::Vector};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{digit1, one_of, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::delimited,
};
use rayon::prelude::*;

type Scalar = i32;
type Veci = Vector<13, Scalar>;

#[derive(Debug, PartialEq, Clone)]
struct Problem {
    target: Veci,
    a_cols: Vec<Veci>,
    jolts: Veci,
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    let number = map_res(digit1, str::parse::<i64>);
    let mut numbers = separated_list1(one_of(","), number);
    numbers.parse(input)
}

fn parse_target(input: &str) -> IResult<&str, Vec<char>> {
    let mut target = delimited(tag("["), many1(one_of(".#")), tag("]"));
    target.parse(input)
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let button = delimited(tag("("), parse_numbers, tag(")"));
    let mut buttons = separated_list1(space1, button);
    buttons.parse(input)
}

fn parse_jolts(input: &str) -> IResult<&str, Vec<i64>> {
    let mut jolts = delimited(tag("{"), parse_numbers, tag("}"));
    jolts.parse(input)
}

fn parse_problem(input: &str) -> IResult<&str, Problem> {
    let (input, target) = parse_target(input)?;
    let (input, _) = space1(input)?;
    let (input, buttons) = parse_buttons(input)?;
    let (input, _) = space1(input)?;
    let (input, jolts) = parse_jolts(input)?;

    let target = target
        .iter()
        .map(|x| match x {
            '#' => 1,
            _ => 0,
        })
        .collect::<Veci>();
    let a_cols = buttons
        .iter()
        .map(|is| {
            let mut ret = Veci::zero();
            for i in is.iter() {
                ret[*i as usize] = 1;
            }
            ret
        })
        .collect::<Vec<_>>();

    Ok((
        input,
        Problem {
            target,
            a_cols,
            jolts: jolts.into_iter().map(|x| x as Scalar).collect::<Veci>(),
        },
    ))
}

// Find candidate 0/1 solutions to A x = b, where A \in {0, 1}
fn get_children<const PARTA: bool>(
    a_cols: &[Veci],
    b: &Veci,
) -> impl Iterator<Item = (Veci, Veci)> {
    itertools::Itertools::powerset(a_cols.iter().enumerate()).flat_map(
        |ivs: Vec<(usize, &Veci)>| {
            if (0..b.len()).all(|i| {
                let presses = ivs.iter().map(|iv| iv.1[i]).sum::<Scalar>();
                if PARTA {
                    presses % 2 == b[i]
                } else {
                    let b_rem = b[i] - presses;
                    b_rem % 2 == 0 && b_rem >= 0
                }
            }) {
                let mut x_bin = Veci::zero();
                for i in ivs.iter().map(|iv| iv.0) {
                    x_bin[i] = 1;
                }
                let b_half = (*b - matmul(a_cols, &x_bin)) / 2;
                Some((x_bin, b_half))
            } else {
                None
            }
        },
    )
}

pub fn part_a(input: &str) -> Answer {
    let response = input
        .lines()
        .par_bridge()
        .map(|line| {
            let problem = parse_problem(line).unwrap().1;
            get_children::<true>(&problem.a_cols, &problem.target)
                .map(|(x_bin, _)| x_bin.iter().sum::<Scalar>())
                .min()
                .unwrap()
        })
        .sum::<Scalar>();

    Answer::Number(response as i64)
}

enum Visit {
    First(Veci),
    Second(Veci),
}

fn solve_b(a_cols: &[Veci], b: &Veci) -> Option<Veci> {
    let mut results: FxHashMap<Veci, Option<Veci>> = FxHashMap::with_capacity(1_000);
    let mut child_map: FxHashMap<Veci, Vec<(Veci, Veci)>> = FxHashMap::with_capacity(1_000);
    let mut stack: Vec<Visit> = Vec::with_capacity(1_000);

    results.insert(Veci::zero(), Some(Veci::zero()));
    stack.push(Visit::First(*b));

    while let Some(visit) = stack.pop() {
        match visit {
            Visit::First(cur) => {
                if results.contains_key(&cur) {
                    continue;
                }
                stack.push(Visit::Second(cur));
                let children = get_children::<false>(a_cols, &cur).collect::<Vec<_>>();
                let entry = child_map.entry(cur).or_insert(children);
                for (_, b_half) in entry.iter() {
                    stack.push(Visit::First(*b_half));
                }
            }
            Visit::Second(cur) => {
                let x_this = child_map[&cur]
                    .iter()
                    .flat_map(|(x_bin, b_half)| results[b_half].map(|x_half| *x_bin + x_half * 2))
                    .min_by_key(|xs| xs.iter().sum::<Scalar>());
                results.insert(cur, x_this);
            }
        }
    }

    results[b]
}

pub fn part_b(input: &str) -> Answer {
    let response = input
        .lines()
        .par_bridge()
        .map(|line| {
            let problem = parse_problem(line).unwrap().1;
            let sol = solve_b(&problem.a_cols, &problem.jolts);
            sol.unwrap().iter().sum::<Scalar>()
        })
        .sum::<Scalar>();

    Answer::Number(response as i64)
}

fn matmul(a_cols: &[Veci], xs: &Veci) -> Veci {
    a_cols
        .iter()
        .zip(xs.iter())
        .map(|(a, x)| *a * *x)
        .sum::<Veci>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(7));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(33));
    }
}

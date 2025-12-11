use crate::Answer;
use crate::map2d::Map;
use bit_vec::BitVec;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{digit1, one_of, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::delimited,
};
use rayon::prelude::*;

#[derive(Debug, PartialEq, Clone)]
struct Problem {
    num_bulbs: usize,
    target: Vec<usize>,
    buttons: Vec<Vec<usize>>,
    jolts: Vec<usize>,
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    let number = map_res(digit1, str::parse::<usize>);
    let mut numbers = separated_list1(one_of(","), number);
    numbers.parse(input)
}

fn parse_target(input: &str) -> IResult<&str, Vec<char>> {
    let mut target = delimited(tag("["), many1(one_of(".#")), tag("]"));
    target.parse(input)
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    let button = delimited(tag("("), parse_numbers, tag(")"));
    let mut buttons = separated_list1(space1, button);
    buttons.parse(input)
}

fn parse_jolts(input: &str) -> IResult<&str, Vec<usize>> {
    let mut jolts = delimited(tag("{"), parse_numbers, tag("}"));
    jolts.parse(input)
}

fn parse_problem(input: &str) -> IResult<&str, Problem> {
    let (input, target) = parse_target(input)?;
    let (input, _) = space1(input)?;
    let (input, buttons) = parse_buttons(input)?;
    let (input, _) = space1(input)?;
    let (input, jolts) = parse_jolts(input)?;

    let num_bulbs = target.len();
    let target_idx = target
        .iter()
        .enumerate()
        .flat_map(|(i, x)| match x {
            '#' => Some(i),
            _ => None,
        })
        .collect::<Vec<_>>();

    Ok((
        input,
        Problem {
            num_bulbs,
            target: target_idx,
            buttons,
            jolts,
        },
    ))
}

fn to_bitvec(len: usize, ns: &[usize]) -> BitVec {
    let mut ret = BitVec::from_elem(len, false);
    for idx in ns.iter() {
        ret.set(*idx, true);
    }
    ret
}

pub fn part_a(input: &str) -> Answer {
    let response: usize = input
        .lines()
        .par_bridge()
        .map(|line| parse_problem(line).unwrap().1)
        .map(|problem| {
            let target = to_bitvec(problem.num_bulbs, &problem.target);
            let buttons = problem
                .buttons
                .iter()
                .map(|vs| to_bitvec(problem.num_bulbs, vs))
                .collect::<Vec<_>>();

            itertools::Itertools::powerset(buttons.iter())
                .flat_map(|vs| {
                    let mut ret = BitVec::from_elem(problem.num_bulbs, false);
                    for v in vs.iter() {
                        ret.xor(*v);
                    }
                    if ret == target {
                        return Some(vs.len());
                    }
                    None
                })
                .min()
                .unwrap()
        })
        .sum();

    Answer::Number(response as i64)
}

fn solve_b(problem: &Problem) -> usize {
    // x1 * v1 + ... + xn * vn = c
    // V x = c, solve by minimizing |x|_1
    //
    // Linear equation with 0/1 coefficients and integer RHS
    //
    // 1. General LP solving. Can implement Gaussian elimination
    // 2. Diophantine equations
    // 3. Tree search (slow..)
    //
    //  x + y = 1
    //  x + y = 3
    //
    // Is it totally unimodular?

    let mut map: Map<i64> = Map::new(problem.num_bulbs, problem.buttons.len());
    for (i, button) in problem.buttons.iter().enumerate() {
        for x in button.iter() {
            map[(*x, i)] = 1;
        }
    }

    println!("{}\n", map);

    0

    // type Veci = Vector<8, i64>;
    //
    // let mut q: VecDeque<(Veci, usize)> = VecDeque::new();
    // q.push_back((problem.jolts.iter().map(|x| *x as i64).collect::<Veci>(), 0));
    //
    // while let Some((cur, n_ops)) = q.pop_front() {
    //     if cur.iter().all(|x| *x == 0) {
    //         return n_ops;
    //     }
    //     if cur.iter().any(|x| *x < 0) {
    //         continue;
    //     }
    //     for button in problem.buttons.iter() {
    //         let mut new_opt = cur;
    //         for x in button.iter() {
    //             new_opt[*x] -= 1;
    //         }
    //         q.push_back((new_opt, n_ops + 1));
    //     }
    // }
    //
    // unreachable!("Not reachable")
}

pub fn part_b(input: &str) -> Answer {
    let response = input
        .lines()
        .par_bridge()
        .map(|line| parse_problem(line).unwrap().1)
        .map(|problem| solve_b(&problem))
        .sum::<usize>();

    Answer::Number(response as i64)
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

use crate::Answer;
use crate::hash::{FxHashMap, FxHashMapBuilder, FxHashSet, FxHashSetBuilder};
use crate::vector::Vec4i;
use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_until1},
    character::complete::{alpha1, space1},
    multi::separated_list1,
};

fn parse_numbers(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (input, name) = take_until1(":").parse(input)?;
    let (input, _) = tag(": ").parse(input)?;
    let (input, children) = separated_list1(space1, alpha1).parse(input)?;
    Ok((input, (name, children)))
}

// Solve with exhaustive dfs
pub fn part_a(input: &str) -> Answer {
    let node_to_children = input
        .lines()
        .map(|x| parse_numbers(x).unwrap().1)
        .collect::<FxHashMap<&str, Vec<&str>>>();

    let mut stack: Vec<&str> = vec!["you"];

    let mut res = 0;
    while let Some(cur) = stack.pop() {
        if cur == "out" {
            res += 1;
        } else {
            for child in node_to_children[cur].iter() {
                stack.push(child);
            }
        }
    }

    Answer::Number(res as i64)
}

// warning: no loop checking
fn topsort_helper<'a>(
    cur: &'a str,
    graph: &FxHashMap<&'a str, Vec<&'a str>>,
    visited: &mut FxHashSet<&'a str>,
    order: &mut Vec<&'a str>,
) {
    visited.insert(cur);
    if let Some(children) = graph.get(cur) {
        for child in children.iter() {
            if !visited.contains(child) {
                topsort_helper(child, graph, visited, order);
            }
        }
    }
    order.push(cur);
}

fn reverse_topological_sort<'a>(
    start: &'a str,
    graph: &FxHashMap<&'a str, Vec<&'a str>>,
) -> Vec<&'a str> {
    let mut visited: FxHashSet<&str> = FxHashSet::new();
    let mut reverse_order: Vec<&str> = vec![];
    topsort_helper(start, graph, &mut visited, &mut reverse_order);
    reverse_order
}

// Solve dynamic programming in reverse topological sort order
pub fn part_b(input: &str) -> Answer {
    let node_to_children = input
        .lines()
        .map(|x| parse_numbers(x).unwrap().1)
        .collect::<FxHashMap<&str, Vec<&str>>>();

    // reverse topological sort
    let reverse_order = reverse_topological_sort("svr", &node_to_children);

    // (visited both, visited fft, visited dac, visited none)
    let mut node_to_npaths: FxHashMap<&str, Vec4i> = FxHashMap::new();
    node_to_npaths.insert("out", Vec4i::new(0, 0, 0, 1));

    for node in reverse_order {
        if let Some(children) = node_to_children.get(node) {
            let sum_of_children = children
                .iter()
                .flat_map(|c| node_to_npaths.get(c))
                .sum::<Vec4i>();

            let node_npaths = match node {
                "fft" => Vec4i::new(
                    sum_of_children[0] + sum_of_children[2],
                    sum_of_children[3],
                    0,
                    0,
                ),
                "dac" => Vec4i::new(
                    sum_of_children[0] + sum_of_children[1],
                    0,
                    sum_of_children[3],
                    0,
                ),
                _ => sum_of_children,
            };

            let val_for_node = node_to_npaths.entry(node).or_default();
            *val_for_node = node_npaths;
        }
    }

    Answer::Number(node_to_npaths["svr"][0])
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT_A: &str = indoc! {"
        aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out
    "};

    const TEST_INPUT_B: &str = indoc! {"
        svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT_A);
        assert_eq!(result, Answer::Number(5));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT_B);
        assert_eq!(result, Answer::Number(2));
    }
}

use crate::map2d::Map;
use crate::vec2::{Vec2i, DIRECTIONS8};
use crate::Answer;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Machine,
    Free,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Tile::Free,
            '@' => Tile::Machine,
            _ => unreachable!("Invalid"),
        }
    }
}

fn can_remove(map: &Map<Tile>, pos: Vec2i) -> bool {
    match map[&pos] {
        Tile::Free => false,
        Tile::Machine => {
            DIRECTIONS8
                .iter()
                .filter(|dir| {
                    if let Some(x) = map.step_within(&pos, **dir, 1) {
                        if map[&x] == Tile::Machine {
                            return true;
                        }
                    }
                    false
                })
                .count()
                < 4
        }
    }
}

pub fn part_a(input: &str) -> Answer {
    let map = Map::from_lines(input.lines(), &Tile::from_char);
    let response = map.iter_coords().filter(|pos| can_remove(&map, *pos)).count();
    Answer::Number(response as i64)
}

pub fn part_b(input: &str) -> Answer {
    let mut map = Map::from_lines(input.lines(), &Tile::from_char);
    let mut removed_total = 0;
    loop {
        let mut removed = 0;
        map.iter_coords().for_each(|pos| {
            if can_remove(&map, pos) {
                removed += 1;
                map[&pos] = Tile::Free;
            }
        });
        if removed == 0 {
            break;
        }
        removed_total += removed;
    }
    Answer::Number(removed_total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(13));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(43));
    }
}

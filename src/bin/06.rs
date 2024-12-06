use std::collections::{HashMap, HashSet};
advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_cw(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

fn parse_grid(grid: &str) -> Vec<Vec<char>> {
    grid.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_starting_pos(grid: &Vec<Vec<char>>) -> Option <(usize, usize)> {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == '^' {
                return Some((row_idx, col_idx));
            }
        }
    }
    None
}

fn move_in_direction(grid: &Vec<Vec<char>>, dir: &Direction, curr_pos: (usize, usize)) -> Option<(usize, usize)> {

    match dir {
        Direction::North => {
            if curr_pos.0 > 0 {
                Some((curr_pos.0 - 1, curr_pos.1))
            } else {
                None
            }
        }
        Direction::South => {
            if curr_pos.0 + 1 < grid.len() {
                Some((curr_pos.0 + 1, curr_pos.1))
            } else {
                None
            }
        }
        Direction::East => {
            if curr_pos.1 + 1 < grid[curr_pos.0].len() {
                Some((curr_pos.0, curr_pos.1 + 1))
            } else {
                None
            }
        }
        Direction::West => {
            if curr_pos.1 > 0 {
                Some((curr_pos.0, curr_pos.1 - 1))
            } else {
                None
            }
        }
    }
}

fn move_until(mut curr_pos: (usize, usize), mut dir: Direction, grid: &Vec<Vec<char>>, seen_locs: &mut HashSet<(usize, usize)>,) -> (usize, usize) {
    loop {
        seen_locs.insert(curr_pos);

        match move_in_direction(&grid, &dir, curr_pos) {
            Some(new_pos) => {
                if grid[new_pos.0][new_pos.1] != '#' {
                    curr_pos = new_pos;
                } else {
                    dir = dir.turn_cw();
                }
            }
            None => {
                break;
            }
        }
    }

    curr_pos
}

pub fn part_one(input: &str) -> Option<u32> {
    let dir = Direction::North;
    let map = parse_grid(input);
    let curr_pos = find_starting_pos(&map);
    let mut seen_locs = HashSet::new();

    move_until(curr_pos?, dir, &map, &mut seen_locs);

    let total = seen_locs.len();
    Some(total as u32)
}

fn is_loop(mut curr_pos: (usize, usize), mut dir: Direction, grid: &Vec<Vec<char>>, seen_locs: &mut HashMap<(usize, usize), HashSet<Direction>>,) -> bool {
    loop {
        let entry = seen_locs.entry(curr_pos).or_insert_with(HashSet::new);
        if !entry.insert(dir) {
            return true;
        }

        match move_in_direction(&grid, &dir, curr_pos) {
            Some(new_pos) => {
                if grid[new_pos.0][new_pos.1] != '#' {
                    curr_pos = new_pos;
                } else {
                    dir = dir.turn_cw();
                }
            }
            None => {
                break;
            }
        }
    }

    false
}
pub fn part_two(input: &str) -> Option<u32> {
    let dir = Direction::North;
    let map = parse_grid(input);
    let curr_pos = find_starting_pos(&map);

    let mut total = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let mut seen_locs = HashMap::new();
            let mut new_map = map.clone();
            if new_map[i][j] != '^' {
                new_map[i][j] = '#';
                if is_loop(curr_pos?, dir, &new_map, &mut seen_locs) {
                    total += 1;
                }
            }
        }
    }

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

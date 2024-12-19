use std::collections::{HashSet, BinaryHeap};
advent_of_code::solution!(18);

#[derive(Clone, Debug, PartialEq, Eq)]
struct State{
    x: usize,
    y: usize,
    steps: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1),
];

fn alter_grid(mut grid: Vec<Vec<char>>, input: &str, bytes: usize) -> Vec<Vec<char>> {
    for (i, line) in input.lines().enumerate() {
        if i >= bytes {
            break;
        }
        if let Some((x, y)) = line.trim().split_once(',') {
            if let (Ok(x), Ok(y)) = (x.parse::<usize>(), y.parse::<usize>()) {
                if x < grid.len() && y < grid[0].len() {
                    grid[y][x] = '#';
                }
            }
        }
    }

    grid
}

fn path_find(grid: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(State {
        x: start.0,
        y: start.1,
        steps:0,
    });

    while let Some(current) = heap.pop() {
        if visited.contains(&(current.x, current.y)) {
            continue;
        }

        visited.insert((current.x, current.y));

        if (current.x, current.y) == end {
            return Some(current.steps);
        }

        for &next_direction in &DIRECTIONS {
            let new_x = current.x as isize + next_direction.0;
            let new_y = current.y as isize + next_direction.1;

            if new_x >= 0
                && new_x < grid.len() as isize
                && new_y >= 0
                && new_y < grid[0].len() as isize
                && grid[new_x as usize][new_y as usize] != '#'
            {
                let new_steps = current.steps + 1;

                heap.push(State {
                    x: new_x as usize,
                    y: new_y as usize,
                    steps: new_steps,
                });
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let x = 6;
    let bytes = 12;
    // let x = 70;
    // let bytes = 1024;

    let mut grid = vec![vec!['.'; x+1]; x+1];
    grid = alter_grid(grid, input, bytes);

    let total = path_find(grid, (0,0), (x, x));
    Some(total? as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // let x = 6;
    // let bytes = 99;
    let x = 70;
    let lines: Vec<_> = input.lines().collect();
    let bytes = lines.len();


    for i in 0..bytes {
        let mut grid = vec![vec!['.'; x+1]; x+1];
        grid = alter_grid(grid, input, i);

        let total = path_find(grid, (0,0), (x, x));
        println!("{:?}", total);

        if total == None {
            println!("{:?}", lines[i-1]);
            break;
        }
    }

    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

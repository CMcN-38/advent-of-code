use std::collections::{HashSet, BinaryHeap};
advent_of_code::solution!(16);

#[derive(Clone, Debug, PartialEq, Eq)]
struct State{
    x: usize,
    y: usize,
    direction: (isize, isize),
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State2{
    x: usize,
    y: usize,
    direction: (isize, isize),
    cost: usize,
    path: HashSet<(usize, usize)>,
}

impl Ord for State2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State2 {
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

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_location(grid: Vec<Vec<char>>, target: char) -> Option<(usize, usize)> {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == target {
                return Some((i, j));
            }
        }
    }

    None
}

fn best_score(grid: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    for &direction in &DIRECTIONS {
        heap.push(State {
            x: start.0,
            y: start.1,
            direction,
            cost:1000,
        });
    }

    while let Some(current) = heap.pop() {
        if visited.contains(&(current.x, current.y, current.direction)) {
            continue;
        }

        visited.insert((current.x, current.y, current.direction));

        if (current.x, current.y) == end {
            return Some(current.cost);
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
                let mut new_cost = current.cost + 1;
                if current.direction != next_direction {
                    new_cost += 1000;
                }

                heap.push(State {
                    x: new_x as usize,
                    y: new_y as usize,
                    direction: next_direction,
                    cost: new_cost,
                });
            }
        }
    }

    None
}

fn track_spaces(grid: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let best_score = best_score(grid.clone(), start, end);

    let mut heap = BinaryHeap::new();
    let mut best_paths: Vec<HashSet<(usize, usize)>> = Vec::new();

    for &direction in &DIRECTIONS {
        heap.push(State2 {
            x: start.0,
            y: start.1,
            direction,
            cost:1000,
            path: HashSet::from([start]),
        });
    }

    while let Some(current) = heap.pop() {
        if current.cost > best_score.expect("."){
            continue;
        }

        if (current.x, current.y) == end && current.cost == best_score.expect("."){
            best_paths.push(current.path.clone());
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
                let new_position = (new_x as usize, new_y as usize);
                if !current.path.contains(&new_position) {
                    let mut new_path = current.path.clone();
                    new_path.insert(new_position);

                    let mut new_cost = current.cost + 1;
                    if current.direction != next_direction {
                        new_cost += 1000;
                    }

                    heap.push(State2 {
                        x: new_x as usize,
                        y: new_y as usize,
                        direction: next_direction,
                        cost: new_cost,
                        path: new_path,
                    });
                }
            }
        }
    }

    let mut unique_spaces = HashSet::new();
    for path in best_paths {
        unique_spaces.extend(path);
    }

    unique_spaces.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let start = find_location(grid.clone(), 'S');
    let end = find_location(grid.clone(), 'E');

    let total = best_score(grid, start?, end?);
    println!("{:?}", total);
    Some(total?)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let start = find_location(grid.clone(), 'S');
    let end = find_location(grid.clone(), 'E');

    let total = track_spaces(grid, start?, end?);
    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}

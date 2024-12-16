use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;
advent_of_code::solution!(16);

#[derive(Clone, Debug, PartialEq, Eq)]
struct State{ x: usize,
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

fn dijkstra_unique_spaces(grid: Vec<Vec<char>>, sx: usize, sy: usize, ex: usize, ey: usize) -> usize {
    let n = grid.len();
    let m = grid[0].len();

    // Directions: (dx, dy)
    let directions: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    // Distance and predecessor maps
    let mut dist: HashMap<(usize, usize, isize, isize), usize> = HashMap::new();
    let mut prev: HashMap<(usize, usize, isize, isize), Vec<(usize, usize, isize, isize)>> = HashMap::new();

    // Initialize priority queue
    let mut heap = BinaryHeap::new();

    // Start state
    dist.insert((sx, sy, 0, 1), 0);
    heap.push(Reverse((0, (sx, sy, 0, 1))));

    while let Some(Reverse((d, (px, py, dx, dy)))) = heap.pop() {
        // Explore turning left and right
        for &(ndx, ndy) in &[(-dy, dx), (dy, -dx)] {
            let new_state = (px, py, ndx, ndy);
            if *dist.get(&new_state).unwrap_or(&usize::MAX) == d + 1000 {
                prev.entry(new_state).or_default().push((px, py, dx, dy));
            }
            if *dist.get(&new_state).unwrap_or(&usize::MAX) > d + 1000 {
                dist.insert(new_state, d + 1000);
                prev.insert(new_state, vec![(px, py, dx, dy)]);
                heap.push(Reverse((d + 1000, new_state)));
            }
        }

        // Move forward
        let npx = px as isize + dx;
        let npy = py as isize + dy;

        if npx < 0 || npx >= n as isize || npy < 0 || npy >= m as isize || grid[npx as usize][npy as usize] == '#' {
            continue;
        }

        let new_state = (npx as usize, npy as usize, dx, dy);
        if *dist.get(&new_state).unwrap_or(&usize::MAX) == d + 1 {
            prev.entry(new_state).or_default().push((px, py, dx, dy));
        }
        if *dist.get(&new_state).unwrap_or(&usize::MAX) > d + 1 {
            dist.insert(new_state, d + 1);
            prev.insert(new_state, vec![(px, py, dx, dy)]);
            heap.push(Reverse((d + 1, new_state)));
        }
    }

    // Find the best distance to the target
    let mut best_dist = usize::MAX;
    let mut end_states = Vec::new();

    for &(dx, dy) in &directions {
        if let Some(&cost) = dist.get(&(ex, ey, dx, dy)) {
            if cost < best_dist {
                best_dist = cost;
                end_states.clear();
                end_states.push((ex, ey, dx, dy));
            } else if cost == best_dist {
                end_states.push((ex, ey, dx, dy));
            }
        }
    }

    // Backtrack to find all unique spaces visited
    let mut visited_spaces = HashSet::new();
    let mut stack = Vec::new();

    for end_state in end_states {
        stack.push(end_state);
    }

    while let Some(state) = stack.pop() {
        let (x, y, dx, dy) = state;
        visited_spaces.insert((x, y));
        if let Some(parents) = prev.get(&state) {
            for &parent in parents {
                stack.push(parent);
            }
        }
    }

    visited_spaces.len()
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


    let unique_spaces = dijkstra_unique_spaces(grid, start?.0, start?.1, end?.0, end?.1);


    Some(unique_spaces as u32)
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

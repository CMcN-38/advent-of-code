advent_of_code::solution!(10);
use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn is_valid_move(x:usize, y:usize, rows:usize, cols:usize) -> bool {
    x < rows && y < cols
}

fn dfs_1(
    grid: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    target_digit: u8,
    visited: &mut HashSet<(usize, usize)>,
    seen_9: &mut HashSet <(usize, usize)>
) -> usize {
    if grid[x][y] != target_digit {
        return 0;
    }

    if target_digit == 9 {
        seen_9.insert((x, y));
        return 1;
    }

    visited.insert((x, y));

    let directions = [(-1, 0), (1,0), (0, -1), (0, 1)];
    let mut path_count = 0;

    for (dx, dy) in directions.iter() {
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;

        if is_valid_move(nx, ny, grid.len(), grid[0].len()) && !visited.contains(&(nx, ny)) && !seen_9.contains(&(nx, ny)) {
            path_count += dfs_1(grid, nx, ny, target_digit + 1, visited, seen_9);
        }
    }

    visited.remove(&(x, y));

    path_count
}

fn dfs(
    grid: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    target_digit: u8,
    visited: &mut HashSet<(usize, usize)>
) -> usize {
    if grid[x][y] != target_digit {
        return 0;
    }

    if target_digit == 9 {
        return 1;
    }

    visited.insert((x, y));

    let directions = [(-1, 0), (1,0), (0, -1), (0, 1)];
    let mut path_count = 0;

    for (dx, dy) in directions.iter() {
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;

        if is_valid_move(nx, ny, grid.len(), grid[0].len()) && !visited.contains(&(nx, ny)) {
            path_count += dfs(grid, nx, ny, target_digit + 1, visited);
        }
    }

    visited.remove(&(x, y));

    path_count
}

fn count_paths_1(grid: Vec<Vec<u8>>) -> usize {
    let mut total_paths = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                let mut visited = HashSet::new();
                let mut seen_9 = HashSet::new();
                total_paths += dfs_1(&grid, i, j, 0, &mut visited, &mut seen_9);
            }
        }
    }

    total_paths
}

fn count_paths(grid: Vec<Vec<u8>>) -> usize {
    let mut total_paths = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                let mut visited = HashSet::new();
                total_paths += dfs(&grid, i, j, 0, &mut visited);
            }
        }
    }

    total_paths
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let total = count_paths_1(grid);

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let total = count_paths(grid);

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}

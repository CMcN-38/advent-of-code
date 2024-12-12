use std::collections::HashSet;

advent_of_code::solution!(12);

fn parse_grid(grid: &str) -> Vec<Vec<char>> {
    grid.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn calculate_area_and_perimeter(grid: &Vec<Vec<char>>) -> Vec<(char, usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut results = Vec::new();

    let directions = vec![(0,1),(1,0),(0,-1),(-1,0)];

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let region_char = grid[r][c];
                let mut area = 0;
                let mut perimeter = 0;

                let mut stack = vec![(r, c)];
                visited[r][c] = true;

                while let Some((x,y)) = stack.pop() {
                    area += 1;

                    for (dx, dy) in &directions {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                            let nx = nx as usize;
                            let ny = ny as usize;

                            if grid[nx][ny] == region_char {
                                if !visited[nx][ny] {
                                    visited[nx][ny] = true;
                                    stack.push((nx,ny));
                                }
                            } else {
                                perimeter += 1;
                            }
                        } else {
                            perimeter += 1;
                        }
                    }
                }

                results.push((region_char, area, perimeter));
            }
        }
    }

    results
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Complex {
    real: i32,
    imag: i32,
}

impl Complex {
    fn new(real: i32, imag: i32) -> Self {
        Complex { real, imag }
    }

    fn add(&self, other: &Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }

    fn multiply(&self, other: &Complex) -> Complex {
        Complex::new(self.real * other.real - self.imag * other.imag,
                      self.real * other.imag + self.imag * other.real)
    }
}

fn calculate_p2(grid: &[Vec<char>]) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut visited = HashSet::new();
    let adj = [
        Complex::new(-1, 0), // Up
        Complex::new(1, 0),  // Down
        Complex::new(0, -1), // Left
        Complex::new(0, 1),  // Right
    ];
    let mut p2_soln = 0;

    fn dfs(node: (usize, usize), grid: &[Vec<char>], visited: &mut HashSet<(usize, usize)>, adj: &[Complex], plants: &mut Vec<(usize, usize)>) {
        let mut stack = vec![node];
        while let Some((x, y)) = stack.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            plants.push((x, y));
            for dir in adj.iter() {
                let nx = (x as i32 + dir.real) as usize;
                let ny = (y as i32 + dir.imag) as usize;
                if nx < grid.len() && ny < grid[0].len() && grid[nx][ny] == grid[x][y] && !visited.contains(&(nx, ny)) {
                    stack.push((nx, ny));
                }
            }
        }
    }

    for x in 0..n {
        for y in 0..m {
            if visited.contains(&(x, y)) {
                continue;
            }
            let mut plants = Vec::new();
            dfs((x, y), grid, &mut visited, &adj, &mut plants);
            let area = plants.len();

            let mut boundaries = Vec::new();
            for &(nx, ny) in &plants {
                for dir in adj.iter() {
                    let nx2 = (nx as i32 + dir.real) as usize;
                    let ny2 = (ny as i32 + dir.imag) as usize;
                    if nx2 >= n || ny2 >= m || grid[nx2][ny2] != grid[nx][ny] {
                        boundaries.push(Complex::new(nx as i32 + dir.real, ny as i32 + dir.imag));
                    }
                }
            }

            let mut sides = 0;
            while !boundaries.is_empty() {
                sides += 1;
                let boundary = boundaries.pop().unwrap();
                for &next_d in &[boundary.multiply(&Complex::new(1, 0)), boundary.multiply(&Complex::new(-1, 0))] {
                    let mut k = 1;
                    while boundaries.contains(&next_d) {
                        boundaries.retain(|&x| x != next_d);
                        k += 1;
                    }
                }
            }

            p2_soln += area as i32 * sides;
        }
    }

    p2_soln
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;

    let grid: Vec<Vec<char>> = parse_grid(input);
    let results = calculate_area_and_perimeter(&grid);

    for (_region, area, perimeter) in results {
        total += area * perimeter;
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {

    let grid: Vec<Vec<char>> = parse_grid(input);
    let total = calculate_p2(&grid);

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}

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

fn calculate_perimeter_corners(grid: &Vec<Vec<char>>) -> Vec<(char, usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut results = Vec::new();

    // Directions for right, down, left, up (clockwise).
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let region_char = grid[r][c];
                let mut area = 0;
                let mut corners = 0;
                let mut perimeter_cells = Vec::new();

                let mut stack = vec![(r, c)];
                visited[r][c] = true;

                while let Some((x, y)) = stack.pop() {
                    area += 1;

                    // Check all four directions around the current cell.
                    for (dx, dy) in &directions {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                            let nx = nx as usize;
                            let ny = ny as usize;

                            if grid[nx][ny] == region_char && !visited[nx][ny] {
                                visited[nx][ny] = true;
                                stack.push((nx, ny));
                            }

                            // If it's a boundary cell, add it to the perimeter list
                            if grid[nx][ny] != region_char || nx < 0 || nx >= rows as usize || ny < 0 || ny >= cols as usize {
                                perimeter_cells.push((x, y));
                            }
                        } else {
                            // The cell is at the boundary of the grid.
                            perimeter_cells.push((x, y));
                        }
                    }
                }

                // Now, count the corners on the perimeter.
                for (x, y) in perimeter_cells {
                    let mut adjacent_sides = 0;

                    // Check the four directions to see if we have two sides forming a corner
                    for (dx, dy) in &directions {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                            let nx = nx as usize;
                            let ny = ny as usize;
                            if grid[nx][ny] != region_char {
                                adjacent_sides += 1;
                            }
                        } else {
                            adjacent_sides += 1;  // Boundary of the grid is considered a side
                        }
                    }

                    // If two sides meet at a right angle, it's a corner
                    if adjacent_sides == 2 {
                        corners += 1;
                    }
                }

                // Store the result for this region
                results.push((region_char, area, corners));
            }
        }
    }

    results
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
    let mut total = 0;

    let grid: Vec<Vec<char>> = parse_grid(input);
    let results = calculate_perimeter_corners(&grid);

    for (region, area, sides) in results {
        println!("{}, {}, {}", region, area, sides);
        total += area * sides;
    }

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

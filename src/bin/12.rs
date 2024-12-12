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

fn calculate_area_and_sides(grid: &Vec<Vec<char>>) -> Vec<(char, usize, usize)> {
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

                    for &(dx, dy) in &[(-1,0), (0,-1)] {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if nx < 0 || ny < 0 || grid [nx as usize][ny as usize] != region_char {
                            perimeter -= 1;
                        }
                    }
                }

                results.push((region_char, area, perimeter));
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
    let results = calculate_area_and_sides(&grid);

    for (region, area, perimeter) in results {
        println!("{}, {}, {}", region, area, perimeter);
        total += area * perimeter;
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

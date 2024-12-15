advent_of_code::solution!(15);

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn parse_directions(input: &str) -> Vec<char> {
    input.replace("\n", "").chars().collect()
}

fn find_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                return Some((i, j));
            }
        }
    }
    None
}

fn move_left(pos: (usize, usize)) -> (usize, usize) {
    (pos.0, pos.1.saturating_sub(1))
}

fn move_right(pos: (usize, usize)) -> (usize, usize) {
    (pos.0, pos.1 + 1)
}

fn move_up(pos: (usize, usize)) -> (usize, usize) {
    (pos.0.saturating_sub(1), pos.1)
}

fn move_down(pos: (usize, usize)) -> (usize, usize) {
    (pos.0 + 1, pos.1)
}

fn is_within_bounds(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    pos.0 < grid.len() && pos.1 < grid[0].len()
}

fn push_box(grid: &mut Vec<Vec<char>>, from: (usize, usize), direction: char) -> bool {
    let mut current_pos = from;
    let original_pos = current_pos;

    loop {
        let next_pos = match direction {
            '<' => move_left(current_pos),
            '>' => move_right(current_pos),
            '^' => move_up(current_pos),
            'v' => move_down(current_pos),
            _ => return false,
        };

        if !is_within_bounds(grid, next_pos) {
            return false; // If out of bounds, stop pushing
        }

        if grid[next_pos.0][next_pos.1] == 'O' {
            // Another box is present, attempt to push it
            current_pos = next_pos; // Move to the next box position
        } else if grid[next_pos.0][next_pos.1] == '.' {
            // Space is empty, check if it's blocked by walls or other boxes
            if is_blocked(grid, next_pos) {
                return false;
            }
            grid[next_pos.0][next_pos.1] = 'O';
            grid[current_pos.0][current_pos.1] = 'O';
            grid[original_pos.0][original_pos.1] = '@';
            return true;
        } else {
            return false; // Can't push because it's blocked
        }
    }
}

fn is_blocked(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    let (x, y) = pos;
    !is_within_bounds(grid, pos) || grid[x][y] == '#'
}

fn move_character(grid: &mut Vec<Vec<char>>, start: &mut (usize, usize), direction: char) -> bool {
    let (x, y) = *start;

    let next_pos = match direction {
        '<' => move_left((x, y)),
        '>' => move_right((x, y)),
        '^' => move_up((x, y)),
        'v' => move_down((x, y)),
        _ => return false,
    };

    if is_within_bounds(grid, next_pos) {
        if grid[next_pos.0][next_pos.1] == '.' {
            grid[x][y] = '.';
            grid[next_pos.0][next_pos.1] = '@';
            *start = next_pos;
            return true;
        } else if grid[next_pos.0][next_pos.1] == 'O' {
            if push_box(grid, next_pos, direction) {
                grid[x][y] = '.';
                *start = next_pos;
                return true;
            }
        }
    }
    false
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

fn get_result(grid: &Vec<Vec<char>>) -> usize {
    let mut total = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                total += (100 * i) + j;
            }
        }
    }

    total
}

pub fn part_one(input: &str) -> Option<u32> {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();

    let mut grid = parse_grid(parts[0]);
    let moves = parse_directions(parts[1]);

    let mut start = find_start(&grid).expect("Start position not found");

    for direction in moves {
        println!("Attempting move: {}", direction);
        if !move_character(&mut grid, &mut start, direction) {
            println!("Move failed for direction: {}", direction);
        }
        print_grid(&grid);
    }

    Some(get_result(&grid) as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092)); // Expected value based on example input
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

advent_of_code::solution!(4);

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let mut output = Vec::new();

    for line in input.lines() {
        let mut line_vec = Vec::new();

        for char in line.chars(){
            line_vec.push(char);
        }

        output.push(line_vec);
    }

    output
}

fn count_rolls(grid: Vec<Vec<char>>, x: usize, y:usize) -> u32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    let search_matrix = [
        (-1, -1)  , (-1, 0)  , (-1, 1),
        (0, -1)              , (0, 1),
        (1, -1)   ,  (1, 0)  , (1, 1),

    ];

    for (dx, dy) in search_matrix {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && ny >= 0 && (nx as usize) < height && (ny as usize) < width {
            if grid[nx as usize][ny as usize] == '@' {
                count += 1;
            }
        }
    }

    count
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut answer = 0;

    let grid = parse_grid(input);

    for (i, line) in grid.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == '@' {
                // println!("{:?}", i);
                // println!("{:?}", j);
                if count_rolls(grid.clone(), i, j) < 4 {
                    answer += 1;
                }
            }
        }
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut answer = 0;

    let mut grid = parse_grid(input);

    let mut changed = true;
    while changed == true {
        changed = false;
        for (i, line) in grid.clone().iter().enumerate() {
            for (j, char) in line.iter().enumerate() {
                if *char == '@' {
                    // println!("{:?}", i);
                    // println!("{:?}", j);
                    if count_rolls(grid.clone(), i, j) < 4 {
                        answer += 1;
                        grid[i][j] = 'X';
                        changed = true;
                    }
                }
            }
        }
    }
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}

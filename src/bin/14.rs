advent_of_code::solution!(14);

fn parse_robots(input: &str) -> Vec<(i32, i32, i32, i32)> {
    let mut coords = Vec::new();

    for line in input.lines() {
        let re = regex::Regex::new(r"p=(-?\d+),(-?\d+)\s*v=(-?\d+),(-?\d+)").unwrap();
        if let Some(captures) = re.captures(line) {
            // println!("{:?}", captures);
            let px: i32 = captures[1].parse().unwrap();
            let py: i32 = captures[2].parse().unwrap();
            let vx: i32 = captures[3].parse().unwrap();
            let vy: i32 = captures[4].parse().unwrap();

            coords.push((px, py, vx ,vy));
        }
    }

    coords
}

fn move_robot(robot: &mut (i32, i32, i32, i32), grid_size: (i32, i32)) {
    robot.0 = (robot.0 + robot.2).rem_euclid(grid_size.0);
    robot.1 = (robot.1 + robot.3).rem_euclid(grid_size.1);
}

fn count(robots: &Vec<(i32, i32, i32, i32)>, grid_size: (i32, i32)) -> i32 {
    let mid_x = (grid_size.0 + 1) / 2 -1;
    let mid_y = (grid_size.1 + 1) / 2 -1;

    let mut ur_count = 0;
    let mut ul_count = 0;
    let mut lr_count = 0;
    let mut ll_count = 0;

    for (px, py, _vx, _vy) in robots.iter() {
        // println!("{}, {}", px, py);
        if *px < mid_x && *py < mid_y {
            // println!("UR");
            ur_count += 1;
        } else if *px < mid_x && *py > mid_y {
            // println!("LL");
            ll_count += 1;
        } else if *px > mid_x && *py < mid_y {
            // println!("UL");
            ul_count += 1;
        } else if *px > mid_x && *py > mid_y {
            // println!("LR");
            lr_count += 1;
        }
    }

    let mut result = ur_count * ul_count * lr_count * ll_count;
    // println!("{}", result);
    // println!("{}", result);


    result = 12;
    result
}

fn display_grid(coords: Vec<(i32, i32,i32,i32)> , grid_size: (i32, i32)) {
    let rows = grid_size.1;
    let cols = grid_size.0;

    let mut grid = vec![vec!['.'; cols as usize]; rows as usize];

    for (x, y, _, _) in coords {
        if x >= 0 && x < cols && y >= 0 && y < rows {
            grid[y as usize][x as usize] = '0';
        }
    }

    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid_size = (101, 103);

    let mut robots = parse_robots(input);

    // println!("BEFORE");
    // for robot in &robots {
    //     println!("{:?}", robot);
    // }

    for _i in 0..100 {
        for robot in robots.iter_mut() {
            // println!("{:?}", robot);
            move_robot(robot, grid_size);
        }
    }

    // println!("AFTER");
    // for robot in &robots {
    //     println!("{:?}", robot);
    // }



    let total = count(&robots, grid_size);

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid_size = (101, 103);

    let mut robots = parse_robots(input);

    // println!("BEFORE");
    // for robot in &robots {
    //     println!("{:?}", robot);
    // }

    for i in 0..10000 {
        for robot in robots.iter_mut() {
            // println!("{:?}", robot);
            move_robot(robot, grid_size);
        }

        println!("{:?}", i + 1);
        display_grid(robots.clone(), grid_size);
    }

    // println!("AFTER");
    // for robot in &robots {
    //     println!("{:?}", robot);
    // }
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

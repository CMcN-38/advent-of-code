advent_of_code::solution!(13);

fn parse_block(block: &str) -> Vec<(i32, i32)> {
    let mut coords = Vec::new();

    for line in block.lines() {
        let re = regex::Regex::new(r"(\w+ \w+|Prize): X[+=]?(\d+), Y[+=]?(\d+)").unwrap();
        if let Some(captures) = re.captures(line) {
            let x_val: i32 = captures[2].parse().unwrap();
            let y_val: i32 = captures[3].parse().unwrap();

            coords.push((x_val,y_val));
        }
    }

    coords
}

fn find_result(mut coords: Vec<(i32, i32)>) -> i32 {
    let cost_a = 3;
    let cost_b = 1;

    let target = coords[2];
    coords.pop();
    // println!("{:?}", target);
    // println!("{:?}", coords);

    let mut min_cost = i32::MAX;

    for a_count in 0..=100 {
        for b_count in 0..=100 {
            let resulting_c = (
                coords[0].0 * a_count + coords[1].0 * b_count,
                coords[0].1 * a_count + coords[1].1 * b_count
            );

            if resulting_c == target {
                let total_cost = a_count * cost_a + b_count * cost_b;

                if total_cost < min_cost {
                    min_cost = total_cost;
                }
            }
        }
    }
    if min_cost > 1000 {
        min_cost = 0;
    }
    // println!("{:?}", min_cost);
    min_cost
}

fn parse_block_2(block: &str) -> Vec<(f64, f64)> {
    let mut coords = Vec::new();

    for line in block.lines() {
        let re = regex::Regex::new(r"(\w+ \w+|Prize): X[+=]?(\d+), Y[+=]?(\d+)").unwrap();
        if let Some(captures) = re.captures(line) {
            let x_val: f64 = captures[2].parse().unwrap();
            let y_val: f64 = captures[3].parse().unwrap();

            coords.push((x_val,y_val));
        }
    }

    coords
}

fn find_result_2(mut coords: Vec<(f64, f64)>) -> f64 {
    let cost_a = 3.0;
    let cost_b = 1.0;
    let target = coords[2];
    let new_target = (target.0 + 10000000000000.0, target.1 + 10000000000000.0);
    coords.pop(); // Remove the target

    let ax = coords[0].0;
    let ay = coords[0].1;
    let bx = coords[1].0;
    let by = coords[1].1;
    let px = new_target.0;
    let py = new_target.1;
    println!("({}, {}), ({}, {}), ({},{})", ax, ay, bx, by, px, py);

    let ca = (px * by - py * bx) as f64/ (ax * by - ay * bx) as f64;
    let cb = (px as f64 - ax as f64 * ca)/ bx as f64;
    println!("{}, {}", ca, cb);

    if (ca - ca.round()).abs() < f64::EPSILON && (cb - cb.round()).abs() < f64::EPSILON {
        return (ca.round() as f64 * cost_a) + (cb.round() as f64 * cost_b);
    } else {
        return 0.0;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let mut total = 0;

    for block in blocks.iter() {
        let coords = parse_block(block);
        // println!("{:?}", coords);
        total += find_result(coords);
    }


    Some(total as u32)
}


pub fn part_two(input: &str) -> Option<u64> {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let mut total = 0;

    for block in blocks.iter() {
        let coords = parse_block_2(block);
        // println!("{:?}", coords);
        total += find_result_2(coords) as i64;
    }


    Some(total as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}

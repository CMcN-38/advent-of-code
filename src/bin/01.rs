advent_of_code::solution!(1);

fn parse_input(input: &str) -> Vec<i32> {
    let mut parsed_output = Vec::new();

    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let mut number: i32 = line[1..].trim().parse().unwrap();


        if direction == 'L' {
            parsed_output.push(-number);
        } else {
            parsed_output.push(number);
        }
    }

    return parsed_output
}


pub fn part_one(input: &str) -> Option<u32> {
    let start = 50;
    let mut counter = 0;
    let instructions = parse_input(input);

    // println!("{}", start);
    // println!("{:?}", instructions);

    let mut loc = start;

    for instruct in instructions {
        let mut num = instruct;

        while num > 100 {
            num -= 100;
        }
        while num < -100 {
            num += 100;
        }

        loc += num;

        if loc > 99 {
            loc -= 100;
        } else if loc < 0 {
            loc += 100;
        }

        if loc == 0 {
            counter += 1;
        }
    }

    return Some(counter)
}

pub fn part_two(input: &str) -> Option<i32> {
    let start = 50;
    let mut counter = 0;
    let instructions = parse_input(input);

    // println!("{}", start);
    // println!("{:?}", instructions);

    let mut loc = start;

    for instruct in instructions {
        let mut num = instruct;

        while num > 100 {
            num -= 100;
            counter += 1;
        }
        while num < -100 {
            num += 100;
            counter += 1;
        }

        if loc == 0 && num < 0 {
            counter -= 1;
        }

        loc += num;

        if loc > 100 {
            loc -= 100;
            counter += 1;
        } else if loc == 100 {
            loc -= 100;
        } else if loc < 0 {
            loc += 100;
            counter += 1;
        }


        if loc == 0 {
            counter += 1;
        }
        // println!("Turn");
        // println!("Loc: {}", loc);
        // println!("Count: {}", counter);
    }


    return Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<String> {
    let vec: Vec<String> = input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    vec
}

fn blink_func(vec: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();

    for num_str in vec {
        match num_str.parse::<isize>() {
            Ok(num) => {
                if num == 0 {
                    result.push("1".to_string());
                } else if num.to_string().len() % 2 == 0 {
                    let half_len = num_str.len() / 2;
                    let (first_half, second_half) = num_str.split_at(half_len);
                    let first_part = first_half.trim_start_matches('0');
                    let mut second_part = second_half.trim_start_matches('0');
                    if second_part == "" {
                        second_part = "0";
                    }
                    result.push(first_part.to_string());
                    result.push(second_part.to_string());
                } else {
                    result.push((num * 2024).to_string());
                }
            }
            Err(_) => {
                result.push("INVALID".to_string());
            }
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let num = 25;

    let mut stones = parse_input(input);

    for _i in 0..num {
        stones = blink_func(stones);
        // println!("{:?}", stones);
    }

    let total = stones.len();

    Some(total as usize)
}


pub fn part_two(input: &str) -> Option<usize> {
    let num = 75;

    let mut stones = parse_input(input);

    for _i in 0..num {
        stones = blink_func(stones);
        // println!("{:?}", stones);
    }

    let total = stones.len();

    Some(total as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

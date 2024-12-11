advent_of_code::solution!(11);
use std::collections::HashMap;


fn parse_input(input: &str) -> Vec<String> {
    let vec: Vec<String> = input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    vec
}

fn parse_input_i(input: &str) -> Vec<i64> {
    let vec: Vec<i64> = input.split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
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

struct Memoization {
    cache: HashMap<(i64, u32), i64>,
}

impl Memoization {
    fn new() -> Self {
        Memoization {
            cache: HashMap::new(),
        }
    }

    fn count(&mut self, stone: i64, steps: u32) -> i64 {
        // Check if the result is already cached
        if let Some(&result) = self.cache.get(&(stone, steps)) {
            return result;
        }

        // Base case
        let result = if steps == 0 {
            1
        } else if stone == 0 {
            self.count(1, steps - 1)
        } else {
            let string = stone.to_string();
            let length = string.len();

            if length % 2 == 0 {
                let mid = length / 2;
                let left = string[..mid].parse::<i64>().unwrap();
                let right = string[mid..].parse::<i64>().unwrap();
                self.count(left, steps - 1) + self.count(right, steps - 1)
            } else {
                self.count(stone * 2024, steps - 1)
            }
        };

        // Cache the result
        self.cache.insert((stone, steps), result);
        result
    }
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
    let stones = parse_input_i(input);

    let mut memo = Memoization::new();
    let result: i64 = stones.iter().map(|&stone| memo.count(stone, num)).sum();

    Some(result as usize)
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
        assert_eq!(result, Some(55312));
    }
}

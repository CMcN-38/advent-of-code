use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;

    for line in input.lines() {
        for captures in re.captures_iter(line) {
            let num1: i32 = captures[1].parse().unwrap();
            let num2: i32 = captures[2].parse().unwrap();
            println!("{}", num1);
            println!("{}", num2);
            total += num1 * num2;
        }
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let mut to_do = true;
    let mut total = 0;

    for line in input.lines() {
        for captures in re.captures_iter(line) {
            if let Some(_) = captures.get(3) {
                to_do = true;
            } else if let Some(_) = captures.get(4) {
                to_do = false;
            } else if to_do == true {
                let num1: i32 = captures[1].parse().unwrap();
                let num2: i32 = captures[2].parse().unwrap();
                total += num1 * num2;
            }
        }
    }

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}

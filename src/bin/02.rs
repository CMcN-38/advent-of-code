advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    for line in input.lines() {
        //create list of u32 for the line
        let parts: Vec<&str> = line.split_whitespace().collect();

        let numbers: Vec<&u32> = line.split_whitespace().iter()
            .filter(|s| s.parse::<u32>().ok())
            .collect();

        let mut is_valid: bool = false;
        if numbers[0] < numbers[1] {
            is_valid = numbers.windows(2).all(|pair|pair[1] - pair[0] <= 3);
        }
        if numbers[0] > numbers[1] {
            is_valid = numbers.windows(2).all(|pair|pair[0] - pair [1] >= 3);
        }

        if is_valid {
            total += 1;
        }
    }

    Some(total)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

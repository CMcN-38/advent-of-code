advent_of_code::solution!(7);

fn parse_input(input: &str) -> (i64, Vec<i64>) {
    let parts: Vec<&str> = input.split(':').collect();
    let first_value: i64 = parts[0].trim().parse().unwrap();

    let vector: Vec<i64> = parts[1].trim().split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect();

    (first_value, vector)
}

fn test_math(test_val: i64, eq_vals: &[i64]) -> i64 {
    fn helper(test_val: i64, current: i64, idx: usize, eq_vals: &[i64]) -> i64 {
        if idx == eq_vals.len() {
            return if current == test_val {1} else {0};
        }

        let next_number = eq_vals[idx];

        let mut count = 0;


        count += helper(test_val, current + next_number, idx + 1, eq_vals);

        if let Some(_) = current.checked_mul(next_number) {
            count += helper(test_val, current * next_number, idx + 1, eq_vals);
        }

        if count > 0 {
            count = 1;
        }
        count
    }

    helper(test_val, eq_vals[0], 1, eq_vals)
}


fn concatenate(a: i64, b: i64) -> i64 {
    let string_a = a.to_string();
    let string_b = b.to_string();

    let concatenated = string_a + &string_b;
    concatenated.parse().unwrap()
}

fn test_math_and_cat(test_val: i64, eq_vals: &[i64]) -> i64 {
    fn helper(test_val: i64, current: i64, idx: usize, eq_vals: &[i64]) -> i64 {
        if idx == eq_vals.len() {
            return if current == test_val {1} else {0};
        }

        let next_number = eq_vals[idx];

        let mut count = 0;


        count += helper(test_val, current + next_number, idx + 1, eq_vals);

        if let Some(_) = current.checked_mul(next_number) {
            count += helper(test_val, current * next_number, idx + 1, eq_vals);
        }

        let concatenated = concatenate(current, next_number);
        count += helper(test_val, concatenated, idx + 1, eq_vals);

        if count > 0 {
            count = 1;
        }
        count
    }

    helper(test_val, eq_vals[0], 1, eq_vals)
}


pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;

    for line in input.lines() {
        let parsed_line = parse_input(line);
        println!("{:?}", parsed_line);
        total += parsed_line.0 * test_math(parsed_line.0, &parsed_line.1);
        println!("{:?}", total);
    }

    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;

    for line in input.lines() {
        let parsed_line = parse_input(line);
        println!("{:?}", parsed_line);
        total += parsed_line.0 * test_math_and_cat(parsed_line.0, &parsed_line.1);
        println!("{:?}", total);
    }

    Some(total as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}

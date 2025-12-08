advent_of_code::solution!(3);

fn parse_input(input: &str) -> Vec<&str> {
    let mut output = Vec::new();

    for line in input.lines() {
        // println!("{:?}", line);
        output.push(line.trim());
    }

    output
}

fn largest_two_digit(n: &str) -> u64 {
    let digits: Vec<u32> = n
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut best = 0;

    for i in 0..digits.len() {
        for j in i + 1..digits.len() {
            let val = digits[i] * 10 + digits[j];
            if val > best {
                best = val;
            }
        }
    }

    // println!("{:?}", best);

    best as u64
}

fn largest_k_digit_subsequence(num: &str, k: usize) -> usize {
    let digits: Vec<char> = num.chars().collect();
    let n = digits.len();
    assert!(k <= n, "Cannot pick more digits than exist");

    let mut result = String::with_capacity(k);
    let mut start = 0;

    for remaining in (1..=k).rev() {
        let end = n - remaining;

        let mut max_digit = '0';
        let mut max_index = start;

        for i in start..=end {
            if digits[i] > max_digit {
                max_digit = digits[i];
                max_index = i;
            }
        }

        result.push(max_digit);
        start = max_index + 1;
    }

    result.parse().expect("ERROR here")
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut answer = 0;
    let parsed = parse_input(input);

    for bank in parsed {
        answer += largest_two_digit(bank)
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut answer = 0;
    let parsed = parse_input(input);

    for bank in parsed {
        answer += largest_k_digit_subsequence(bank, 12)
    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}

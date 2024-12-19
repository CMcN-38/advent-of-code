use std::collections::HashSet;
advent_of_code::solution!(19);

fn parse_input(input: &str) -> (HashSet<String>, Vec<String>) {
    let mut sections = input.split("\n\n");
    let towel_patterns = sections
        .next()
        .expect("Invalid Input")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<HashSet<_>>();

    let desired_designs = sections
        .next()
        .expect("Invalid Input")
        .lines()
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();

    (towel_patterns, desired_designs)
}

fn is_possible_design(design: &str, towels: &HashSet<String>) -> bool {
    let mut dp = vec![false; design.len() + 1];

    dp[0] = true;

    for i in 0..design.len() {
        if dp[i] {
            for towel in towels {
                if design[i..].starts_with(towel) {
                    dp[i + towel.len()] = true;
                }
            }
        }
    }

    dp[design.len()]
}

fn count_possible(designs: &[String], towels: &HashSet<String>) -> usize {
    designs.iter().filter(|design| is_possible_design(design, towels)).count()
}

fn count_ways_to_create(design: &str, towels: &HashSet<String>) -> usize {
    let mut dp = vec![0; design.len() + 1];
    dp[0] = 1;

    for i in 0..design.len() {
        if dp[i] > 0 {
            for towel in towels {
                if design[i..].starts_with(towel) {
                    dp[i + towel.len()] += dp[i];
                }
            }
        }
    }

    dp[design.len()]
}

fn count_total_ways(designs: &[String], towels: &HashSet<String>) -> usize {
    designs.iter().map(|design| count_ways_to_create(design, towels)).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (towel_patterns, desired_designs) = parse_input(input.trim());

    let possible_count = count_possible(&desired_designs, &towel_patterns);

    Some(possible_count as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (towel_patterns, desired_designs) = parse_input(input.trim());

    let possible_count = count_total_ways(&desired_designs, &towel_patterns);

    Some(possible_count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}

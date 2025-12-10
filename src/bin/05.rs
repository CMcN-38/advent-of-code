advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<u64>) {
    let mut output_ranges = Vec::new();
    let mut output_ingredients = Vec::new();

    let parts: Vec<&str> = input.splitn(2, "\n\n").collect();

    for line in parts.get(0).expect("No First Half").lines() {
        let mut line_vec = Vec::new();
        for range in line.split("-"){
            // println!("{:?}", line);
            // println!("{:?}", range);
            line_vec.push(range.trim().parse().expect("NOT INT"));
        }

        output_ranges.push(line_vec);
    }

    for line in parts.get(1).expect("No Second Half").lines() {
        output_ingredients.push(line.trim().parse().expect("NOT INT 2"));
    }

    (output_ranges, output_ingredients)
}

fn in_range(i: u64, ranges: Vec<Vec<u64>>) -> bool {

    for r in ranges {
        if i >= r[0] && i <= r[1] {
            return true;
        }
    }

    false
}

fn calc_unique_values(ranges: Vec<Vec<u64>>) -> u64 {
    let mut intervals: Vec<(u64, u64)> = ranges
        .into_iter()
        .filter_map(|v| {
            if v.len() == 2 {
                Some((v[0], v[1]))
            } else {
                None
            }
        })
        .collect();

    intervals.sort_by_key(|(start, _)| *start);

    let mut merged: Vec<(u64, u64)> = Vec::new();

    for (start, end) in intervals {
        if let Some((_last_start, last_end)) = merged.last_mut() {
            if start <= *last_end + 1 {
                *last_end = (*last_end).max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    merged.into_iter().map(|(s,e)| e - s + 1).sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut answer = 0;

    let (ranges, ingredients) = parse_input(input);

    for i in ingredients {
        if in_range(i, ranges.clone()) {
            answer += 1;
        }
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _ingredients) = parse_input(input);

    Some(calc_unique_values(ranges))
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
        assert_eq!(result, Some(14));
    }
}

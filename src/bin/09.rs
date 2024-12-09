advent_of_code::solution!(9);

pub fn explode_input(input: &str) -> Vec<String> {
    let mut exploded = Vec::new();

    for (idx, charac) in input.trim().chars().enumerate() {
        if idx % 2 == 0 {
            for _i in 0..(charac.to_digit(10).expect("Not Valid Number") as i32) {
                let new_idx = (idx + 1) /2;
                exploded.push(new_idx.to_string());
            }
        } else {
            for _i in 0..(charac.to_digit(10).expect("Not Valid Number") as i32) {
                exploded.push(".".to_string())
            }
        }
    }

    exploded
}

fn parse_vec(vec: Vec<String>) -> Vec<char> {
    let vec_chars: Vec<char> = vec.into_iter().map(|s| s.chars().next().unwrap()).collect();
    vec_chars
}

fn order_disk(vec: &mut Vec<String>) {
    let mut left = 0;
    let mut right = vec.len() - 1;

    while left < right {
        while left < vec.len() && vec[left] != '.'.to_string() {
            left += 1;
        }

        while right > 0 && vec[right] == '.'.to_string() {
            right -= 1;
        }

        if left < right {
            vec.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}
fn rearrange_vector(mut vec: Vec<char>) -> Vec<char> {
    // Helper function to find the next block of dots large enough to fit a given size.
    fn find_first_dot_block(vec: &[char], size: usize) -> Option<usize> {
        let mut count = 0;
        let mut start = None;

        for (i, &c) in vec.iter().enumerate() {
            if c == '.' {
                if start.is_none() {
                    start = Some(i);
                }
                count += 1;
                if count >= size {
                    return start;
                }
            } else {
                count = 0;
                start = None;
            }
        }
        None
    }
    let mut seen: Vec<char> = Vec::new();
    let mut i = vec.len() - 1;
    while i > 0 {
        // Identify the current block of numbers.
        if vec[i].is_digit(10) && !seen.contains(&vec[*&i]) {
            seen.push(vec[i]);
            let end = i;
            while i > 0 && vec[i - 1].is_digit(10) && vec[i-1] == vec[i]{
                i -= 1;
            }
            let start = i;
            let block_size = end - start + 1;
            let block: Vec<char> = vec[start..=end].to_vec();

            // Find the first block of dots that can fit this block of numbers.
            if let Some(dot_start) = find_first_dot_block(&vec, block_size) {
                // Move the block of numbers to the dots' position.
                for j in 0..block_size {
                    vec[dot_start + j] = block[j];
                }
                // Replace the original position of the block with dots.
                for j in start..=end {
                    vec[j] = '.';
                }
            }

            println!("{:?}", vec);
        }
        i -= 1;
    }

    vec
}


fn calc_checksum(vec: Vec<String>) -> u64 {
    let mut total = 0;

    for (idx, i) in vec.iter().enumerate() {
        if *i == '.'.to_string() {
            continue;
        } else {
            let number: i64 = i.parse().expect("Not Valid digit");
            total += number * (idx as i64);
        }
    }

    total as u64
}

fn calc_checksum_new(vec: Vec<char>) -> u64 {
    let mut total = 0;

    for (idx, i) in vec.iter().enumerate() {
        if *i == '.' {
            continue;
        } else {
            let number = i.to_digit(10).expect("None") ;
            total += (number as i64) * (idx as i64);
        }
    }

    total as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut exploded = explode_input(input);
    order_disk(&mut exploded);

    Some(calc_checksum(exploded) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut exploded = explode_input(input);
    let parsed_exploded = parse_vec(exploded);
    let ordered = rearrange_vector(parsed_exploded);

    println!("{:?}", ordered);
    Some(calc_checksum_new(ordered) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}

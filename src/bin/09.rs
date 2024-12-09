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

fn order_disk_new(vec: &mut Vec<String>) {
    let mut left = 0;
    let mut right = vec.len() - 1;

    while left < right {
        while left < vec.len() && vec[left] != '.'.to_string() {
            left += 1;
        }

        while right > 0 && vec[right] == '.'.to_string() {
            right -= 1;
        }
        println!("{} {}", left, right);

        if left < right {
            let block_value = &vec[right];
            let mut block_end = right;

            while block_end > 0 && vec[block_end - 1] == *block_value {
                block_end -= 1;
            }

            let mut dot_end = left;
            while dot_end < vec.len() && vec[dot_end] == '.'.to_string() {
                dot_end += 1;
            }

            println!("{} {} {} {}", left, dot_end, block_end, right);
            if dot_end - left - 1 >= right - block_end{
                for i in block_end..right+1 {
                    vec.swap(i, left);
                    left += 1;
                    println!("{:?}", vec);
                }
            } else {
                right = block_end - 1;
                left = dot_end;
            }
        }
    }
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

pub fn part_one(input: &str) -> Option<u64> {
    let mut exploded = explode_input(input);
    order_disk(&mut exploded);

    Some(calc_checksum(exploded) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut exploded = explode_input(input);
    order_disk_new(&mut exploded);

    println!("{:?}", exploded);
    Some(calc_checksum(exploded) as u64)
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

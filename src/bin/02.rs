advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    for line in input.lines() {
        //create list of u32 for the line
        let numbers: Vec<u32> = line.split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        // println!("{:?}",numbers);

        let mut is_valid: bool = false;
        if numbers[0] < numbers[1] {
            // println!("1 bigger than 0");
            is_valid = numbers.windows(2).all(|pair|(pair[1] as i32 - pair[0] as i32) <= 3 && (pair[1] as i32 - pair[0] as i32) > 0);
        }
        if numbers[0] > numbers[1] {
            is_valid = numbers.windows(2).all(|pair|(pair[0] as i32 - pair[1] as i32) <= 3 && (pair[0] as i32 - pair[1] as i32) > 0);
        }

        if is_valid {
            total += 1;
        }

        // println!("{}", total);
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    for line in input.lines() {
        //create list of u32 for the line
        let numbers: Vec<u32> = line.split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        // println!("{:?}",numbers);

        fn is_valid_seq_asc(numbers: &Vec<u32>) -> bool {
            numbers.windows(2).all(|pair|{
                let diff = pair[1] as i32 - pair[0] as i32;
                diff >= 1 && diff <= 3
            })
        }

        fn is_valid_seq_desc(numbers: &Vec<u32>) -> bool {
            numbers.windows(2).all(|pair|{
                let diff = pair[1] as i32 - pair[0] as i32;
                diff <= -1 && diff >= -3
            })
        }

        let mut is_valid: bool = false;
        let mut bad_comp: bool = false;

        if is_valid_seq_asc(&numbers) || is_valid_seq_desc(&numbers) {
            is_valid = true;
        } else {
            for i in 0..numbers.len() {
                let mut modified_list = numbers.clone();
                modified_list.remove(i);


                // println!("{:?}",modified_list);

                if is_valid_seq_asc(&modified_list) || is_valid_seq_desc(&modified_list) {
                    bad_comp = true;
                    break;
                }
            }
        }

        if is_valid || bad_comp {
            total += 1;
        }

        // println!("{}", is_valid);
        // println!("{}", bad_comp);
    }

    Some(total)
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
        assert_eq!(result, Some(12));
    }
}

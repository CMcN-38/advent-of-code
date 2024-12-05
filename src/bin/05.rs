use std::collections::HashMap;


advent_of_code::solution!(5);

pub fn is_valid_line(line: &str, map: &HashMap<i32, Vec<i32>>) -> bool {
    let numbers: Vec<i32> = line.split(',').filter_map(|s| s.parse().ok()).collect();

    for &num in &numbers {
        if let Some(values) = map.get(&num) {
            let num_index = numbers.iter().position(|&x| x == num).unwrap();

            for &value in values {
                if let Some(value_index) = numbers.iter().position(|&x| x == value) {
                    if value_index < num_index {
                        return false;
                    }
                }
            }
        }
    }
    // println!("{}", line);
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();
    if sections.len() != 2 {
        panic!("Expected two sections, but found {}", sections.len());
    }

    let first_section = sections[0];
    let second_section = sections[1];

    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    for entry in first_section.lines() {
        let parts: Vec<&str> = entry.split('|').collect();
        let key: i32 = parts[0].parse().unwrap();
        let value: i32 = parts[1].parse().unwrap();

        map.entry(key).or_insert_with(Vec::new).push(value);
    }

    // println!("HashMap: {:#?}", map);
    let test_string = second_section.replace("\n", "\n");
    // println!("\nTest String:\n{}", test_string);

    let test_lines: Vec<String> = test_string.lines().map(|line| line.to_string()).collect();
    let valid_lines: Vec<String> = test_lines.into_iter()
        .filter(|line| is_valid_line(line, &map))
        .collect();

    // println!("Valid Lines:");
    // println!("{}", valid_lines.len());
    // for line in valid_lines {
    //     println!("{}",line);
    // }

    let mut total = 0;

    for line in valid_lines {
        let numbers: Vec<i32> = line.split(',').filter_map(|s| s.parse().ok()).collect();
        let middle_index = numbers.len() / 2;
        total += numbers[middle_index];
    }

    Some(total as u32)
}

pub fn reorder_invalid_lines(invalid_lines: Vec<String>, map: &HashMap<i32, Vec<i32>>) -> Vec<String> {
    // Helper function to check if a number a should come before b according to the map
    fn should_swap(a: i32, b: i32, map: &HashMap<i32, Vec<i32>>) -> bool {
        // If b is a dependency of a (i.e., a should come before b), we should swap
        if let Some(dependencies) = map.get(&b) {
            if dependencies.contains(&a) {
                return true; // Swap a and b
            }
        }
        false
    }

    // Bubble sort helper to sort a line based on map dependencies
    fn bubble_sort_line(mut numbers: Vec<i32>, map: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..numbers.len() - 1 {
                // Compare adjacent numbers and swap if necessary
                if should_swap(numbers[i], numbers[i + 1], map) {
                    numbers.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
        numbers
    }

    // Reorder each invalid line independently using bubble sort
    let mut reordered_lines = Vec::new();

    for line in invalid_lines {
        // Parse the numbers in the line
        let mut numbers: Vec<i32> = line.split(',')
                                        .filter_map(|s| s.parse().ok())
                                        .collect();

        // Sort the numbers using bubble sort
        let sorted_numbers = bubble_sort_line(numbers, map);

        // Convert the sorted numbers back to a string and add to the result
        let sorted_line = sorted_numbers.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join(",");
        reordered_lines.push(sorted_line);
    }

    reordered_lines
}

pub fn part_two(input: &str) -> Option<u32> {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();
    if sections.len() != 2 {
        panic!("Expected two sections, but found {}", sections.len());
    }

    let first_section = sections[0];
    let second_section = sections[1];

    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    for entry in first_section.lines() {
        let parts: Vec<&str> = entry.split('|').collect();
        let key: i32 = parts[0].parse().unwrap();
        let value: i32 = parts[1].parse().unwrap();

        map.entry(key).or_insert_with(Vec::new).push(value);
    }

    // println!("HashMap: {:#?}", map);
    let test_string = second_section.replace("\n", "\n");
    // println!("\nTest String:\n{}", test_string);

    let test_lines: Vec<String> = test_string.lines().map(|line| line.to_string()).collect();
    let mut invalid_lines: Vec<String> = Vec::new();
    let _valid_lines: Vec<String> = test_lines.into_iter()
        .filter(|line|{
            if is_valid_line(line, &map) {
                true
            } else {
                invalid_lines.push(line.clone());
                false
            }
        })
        .collect();

    // println!("Valid Lines:");
    // println!("{}", valid_lines.len());
    // for line in valid_lines {
    //     println!("{}",line);
    // }
    // println!("Invalid Lines:");
    // println!("{}", invalid_lines.len());
    // for line in &invalid_lines {
    //     println!("{}",line);
    // }

    let mut total = 0;
    // for line in valid_lines {
    //     let numbers: Vec<i32> = line.split(',').filter_map(|s| s.parse().ok()).collect();
    //     let middle_index = numbers.len() / 2;
    //     total += numbers[middle_index];
    // }

    let reordered_list = reorder_invalid_lines(invalid_lines, &map);

    println!("Reodered Lines:");
    for line in &reordered_list {
        println!("{}", line);
    }


    for line in reordered_list {
        let numbers: Vec<i32> = line.split(',').filter_map(|s| s.parse().ok()).collect();
        let middle_index = numbers.len() / 2;
        total += numbers[middle_index];
    }

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}

use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
        let mut list1 = Vec::new();
        let mut list2 = Vec::new();

        //Create the lists
        for line in input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                list1.push(num1);
                list2.push(num2);
            }
        }

        //sort the lists
        list1.sort();
        list2.sort();

        let mut total = 0;

        //Add the absolute difference between each pair to the total
        for i in 0..list1.len() {
            total += (list1[i] - list2[i]).abs();
        }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
        let mut list1 = Vec::new();
        let mut list2 = Vec::new();

        //Create the lists
        for line in input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                list1.push(num1);
                list2.push(num2);
            }
        }

        //sort the lists
        list1.sort();
        list2.sort();

        let mut counts = HashMap::new();
        for &num in &list2 {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut total: u32 = 0;

        for &num in &list1 {
            if let Some(&count) = counts.get(&num) {
                total += num * count;
            }
        }


    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut parsed_output = Vec::new();

    for section in input.split(",") {
        let mut st_en = Vec::new();

        for sep_sec in section.split("-") {
            // println!("{:?}", sep_sec);
            st_en.push(sep_sec.trim().parse().expect("Not INT"))
        }

        parsed_output.push(st_en);
    }

    parsed_output
}

fn check_dup_patterns(num: u64) -> bool {
    let stringed = num.to_string();
    let len = stringed.len();

    if stringed[0..len/2] == stringed[len/2..] {
        return true;
    }

    false
}

fn check_mult_patterns(num: u64) -> bool {
    let stringed = num.to_string();
    let len = stringed.len();

    for sub_len in 1..=len / 2 {
        if len % sub_len != 0 {
            continue;
        }

        let substr = &stringed[..sub_len];
        let repeat_count = len / sub_len;

        if repeat_count >= 2 && substr.repeat(repeat_count) == stringed {
            // println!("{:?}", stringed);
            return true;
        }
    }

    false
}

fn count_start_to_end_1(start_end: Vec<u64>) -> u64 { let start = start_end[0];
    let end = start_end[1];
    let mut current = start;
    let mut count = 0;

    while current <= end {

        if check_dup_patterns(current) {
            // count+=1;
            count+=current;
        }

        current += 1;
    }

    count
}

fn count_start_to_end_2(start_end: Vec<u64>) -> u64 { let start = start_end[0];
    let end = start_end[1];
    let mut current = start;
    let mut count = 0;

    while current <= end {

        if check_mult_patterns(current) {
            // count+=1;
            count+=current;
        }

        current += 1;
    }

    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let list = parse_input(input);
    let mut answer = 0;

    for section in list {
        answer += count_start_to_end_1(section);
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let list = parse_input(input);
    let mut answer = 0;

    for section in list {
        answer += count_start_to_end_2(section);
    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}

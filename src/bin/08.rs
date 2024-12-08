use regex::Regex;
use std::collections::{HashMap, HashSet};
advent_of_code::solution!(8);

fn get_nodes_and_locs(input: &str) -> Vec<(char, usize, usize)> {
    let re = Regex::new(r"[A-Za-z0-9]").unwrap();
    let mut positions = Vec::new();

    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, char) in line.chars().enumerate() {
            if re.is_match(&char.to_string()) {
                positions.push((char, row_idx, col_idx));
            }
        }
    }

    positions
}


fn get_matching_pairs(nodes: &[(char, usize, usize)]) -> Vec<((usize, usize), (usize, usize))> {
    let mut pairs = Vec::new();
    let mut seen_pairs: HashSet<((usize, usize), (usize, usize))> = HashSet::new();

    let mut grouped: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for &(char, row, col) in nodes {
        grouped.entry(char).or_insert(Vec::new()).push((row, col));
    }

    for (_, positions) in grouped {
        for i in 0..positions.len() {
            for j in 1..positions.len() {
                let (row1, col1) = positions[i];
                let (row2, col2) = positions[j];

                if (row1 == row2) && (col1 == col2) {
                    continue;
                }

                let pair1 = ((row1, col1), (row2, col2));
                let pair2 = ((row2, col2), (row1, col1));

                if seen_pairs.contains(&pair1) || seen_pairs.contains(&pair2) {
                    continue;
                }

                pairs.push(((row1, col1), (row2, col2)));
                seen_pairs.insert(pair1);
            }
        }
    }

    pairs
}

fn find_slope((row_1, col_1): (usize, usize), (row_2, col_2): (usize, usize)) -> (isize, isize) {
    let row_diff = (row_2 as isize) - (row_1 as isize);
    let col_diff = (col_2 as isize) - (col_1 as isize);

    (row_diff, col_diff)
}

pub fn part_one(input: &str) -> Option<u32> {
    let all_nodes = get_nodes_and_locs(input);
    let pairs = get_matching_pairs(&all_nodes);

    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();

    let mut new_nodes: HashSet<(isize, isize)> = HashSet::new();

    for ((row_1, col_1), (row_2, col_2)) in pairs {

        // println!("({},{}) -> ({},{})", row_1, col_1, row_2, col_2);
        let (row_diff, col_diff) = find_slope((row_1, col_1), (row_2, col_2));
        let forward_vec: (isize, isize) = ((row_2 as isize) + row_diff, (col_2 as isize) + col_diff);
        let backward_vec: (isize, isize) = ((row_1 as isize) - row_diff, (col_1 as isize) - col_diff);

        // println!("Difference = ({},{})", row_diff, col_diff);

        if forward_vec.0 >= 0 && forward_vec.0 < (num_rows as isize) && forward_vec.1 >= 0 && forward_vec.1 < (num_cols as isize) {
            println!("({},{})", forward_vec.0, forward_vec.1);
            new_nodes.insert(forward_vec);
        }

        if backward_vec.0 >= 0 && backward_vec.0 < (num_rows as isize) && backward_vec.1 >= 0 && backward_vec.1 < (num_cols as isize) {
            println!("({},{})", backward_vec.0, backward_vec.1);
            new_nodes.insert(backward_vec);
        }
    }

    Some(new_nodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let all_nodes = get_nodes_and_locs(input);
    let pairs = get_matching_pairs(&all_nodes);

    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();

    let mut new_nodes: HashSet<(isize, isize)> = HashSet::new();

    for ((row_1, col_1), (row_2, col_2)) in pairs {

        for i in 0..1000 {
            // println!("({},{}) -> ({},{})", row_1, col_1, row_2, col_2);
            let (mut row_diff, mut col_diff) = find_slope((row_1, col_1), (row_2, col_2));

            row_diff *= i;
            col_diff *= i;

            let forward_vec: (isize, isize) = ((row_2 as isize) + row_diff, (col_2 as isize) + col_diff);
            let backward_vec: (isize, isize) = ((row_1 as isize) - row_diff, (col_1 as isize) - col_diff);

            // println!("Difference = ({},{})", row_diff, col_diff);

            if forward_vec.0 >= 0 && forward_vec.0 < (num_rows as isize) && forward_vec.1 >= 0 && forward_vec.1 < (num_cols as isize) {
                println!("({},{})", forward_vec.0, forward_vec.1);
                new_nodes.insert(forward_vec);
            }

            if backward_vec.0 >= 0 && backward_vec.0 < (num_rows as isize) && backward_vec.1 >= 0 && backward_vec.1 < (num_cols as isize) {
                println!("({},{})", backward_vec.0, backward_vec.1);
                new_nodes.insert(backward_vec);
            }
        }
    }

    Some(new_nodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

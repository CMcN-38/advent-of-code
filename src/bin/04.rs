advent_of_code::solution!(4);

pub fn count_word_in_lines(lines: Vec<String>, word: &str, reverse_word: &str) -> usize {
    let mut count = 0;
    for line in lines {
        count += line.matches(word).count();
        count += line.matches(reverse_word).count();}
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let word = "XMAS";
    let reverse_word = word.chars().rev().collect::<String>();
    let mut total = 0;

    //horizontal count
    let horizontal_lines: Vec<String> = lines.clone();
    total += count_word_in_lines(horizontal_lines, word, &reverse_word);

    //vertical count
    let mut vertical_lines = Vec::new();
    for col in 0..cols{
        let mut column = String::new();
        for row in 0..rows {
            column.push(lines[row].chars().nth(col).unwrap());
        }
        vertical_lines.push(column);
    }

    total += count_word_in_lines(vertical_lines, word, &reverse_word);

    //Diagonal
    let mut diagonals_tl_br = Vec::new();

    for start_row in 0..rows {
        let mut diagonal = String::new();
        let mut r = start_row;
        let mut c = 0;
        while r < rows && c < cols {
            diagonal.push(lines[r].chars().nth(c).unwrap());
            r += 1;
            c += 1;
        }
        diagonals_tl_br.push(diagonal);
    }

    for start_col in 1..cols {
        let mut diagonal = String::new();
        let mut r =0;
        let mut c = start_col;
        while r < rows && c < cols {
            diagonal.push(lines[r].chars().nth(c).unwrap());
            r += 1;
            c += 1;
        }
        diagonals_tl_br.push(diagonal);
    }

    total += count_word_in_lines(diagonals_tl_br, word, &reverse_word);

    // Diagonals from the top right

    let mut diagonals_tr_bl = Vec::new();

    for start_row in 0..rows {
        let mut diagonal = String::new();
        let mut r = start_row;
        let mut c = cols - 1;
        while r < rows {
            diagonal.push(lines[r].chars().nth(c).unwrap());
            r += 1;
            if c == 0 {break;}
            c -= 1;
        }
        diagonals_tr_bl.push(diagonal);
    }

    for start_col in (0..cols - 1).rev() {
        let mut diagonal = String::new();
        let mut r =0;
        let mut c = start_col;
        while r < rows{
            diagonal.push(lines[r].chars().nth(c).unwrap());
            r += 1;
            if c == 0 {break;}
            c -= 1;
        }
        diagonals_tr_bl.push(diagonal);
    }

    total += count_word_in_lines(diagonals_tr_bl, word, &reverse_word);


    Some(total as u32)
}

pub fn check_cross(i: usize, j:usize, lines: &Vec<String>, rows: usize, cols: usize) -> bool {
    if i == 0 || j == 0 || i == rows - 1 || j == cols - 1 {
        return false;
    }

    let middle = lines[i].chars().nth(j).unwrap();
    if middle != 'A' {
        return false;
    }

    let tl_forward = lines[i-1].chars().nth(j-1).unwrap() =='M' && lines[i+1].chars().nth(j+1).unwrap() == 'S';
    let bl_forward = lines[i-1].chars().nth(j+1).unwrap() =='S' && lines[i+1].chars().nth(j-1).unwrap() == 'M';

    let bl_backward = lines[i-1].chars().nth(j+1).unwrap() =='M' && lines[i+1].chars().nth(j-1).unwrap() == 'S';
    let tl_backward = lines[i-1].chars().nth(j-1).unwrap() =='S' && lines[i+1].chars().nth(j+1).unwrap() == 'M';

    println!("{},*,{}", lines[i-1].chars().nth(j-1).unwrap(), lines[i-1].chars().nth(j+1).unwrap());
    println!("*,{},*", lines[i].chars().nth(j).unwrap());
    println!("{},*,{}", lines[i+1].chars().nth(j-1).unwrap(), lines[i+1].chars().nth(j+1).unwrap());
    println!("");

    (tl_forward && bl_forward) ||
    (tl_forward && bl_backward) ||
    (tl_backward && bl_forward) ||
    (tl_backward && bl_backward)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let mut total = 0;

    for i in 1..rows-1 {
        for j in 1..cols-1 {
            if check_cross(i, j, &lines, rows, cols) {
                total += 1;
            }
        }
    }

    Some(total as u32)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

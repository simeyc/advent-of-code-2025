use std::cmp::{max, min};
use std::fs;

fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.split('\n').collect();
    let y_max = lines.len() - 1;
    let mut result = 0;
    for (i, line) in lines.iter().enumerate() {
        let x_max = line.len() - 1;
        for (j, ch) in line.char_indices() {
            if ch != '@' {
                continue; // Not a roll of paper; skip.
            }
            let mut num_adjacent_rolls = 0;
            // Iterate 9-char grid around ch.
            let top = max(i as i32 - 1, 0);
            let btm = min(i as i32 + 1, y_max as i32);
            for y in top..=btm {
                let left = max(j as i32 - 1, 0);
                let right = min(j as i32 + 1, x_max as i32);
                for x in left..=right {
                    if y == i as i32 && x == j as i32 {
                        continue; // Ignore char under test.
                    }
                    if lines[y as usize].chars().nth(x as usize).unwrap() == '@' {
                        num_adjacent_rolls += 1;
                    }
                }
            }
            if num_adjacent_rolls < 4 {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let part1_result = part1(&input);
    println!("part1 result: {}", part1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
        ..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.\
    ";

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 13);
    }
}

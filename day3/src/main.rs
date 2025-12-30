use std::char::from_digit;
use std::fs;

const PART2_BANK_SZ: usize = 12;

fn part1(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let mut maxes = [0; 2];
        for (i, ch) in line.char_indices() {
            let num = ch.to_digit(10).unwrap();
            if num > maxes[0] {
                if i == line.len() - 1 {
                    // No more digits to follow; make this the 2nd max.
                    maxes[1] = num;
                } else {
                    // More digits to follow, make this the 1st max and clear the 2nd.
                    maxes[0] = num;
                    maxes[1] = 0;
                }
            } else if num > maxes[1] {
                maxes[1] = num;
            }
        }
        println!("maxes: [{}, {}]", maxes[0], maxes[1]);
        let joltage: i32 = format!("{}{}", maxes[0], maxes[1]).parse().unwrap();
        total += joltage;
    }
    println!("total: {total}");
    total
}

// Recursion for the craic! Prototype:
// Any 9s between index 0 and -11?
// - yes -> select, any 9s between it and index -10?...
// - no -> any 8s?...
fn select_maxes(chars: &str, bank: &mut String, digit: char) {
    let bank_slots = PART2_BANK_SZ - bank.len();
    if bank_slots == 0 {
        return;
    }
    if chars.len() <= bank_slots {
        bank.push_str(chars);
        return;
    }
    let idx = &chars[..=chars.len() - bank_slots].find(digit);
    match idx {
        Some(i) => {
            bank.push(digit);
            select_maxes(&chars[i + 1..], bank, '9');
        }
        None => {
            let next_digit = from_digit(digit.to_digit(10).unwrap() - 1, 10).unwrap();
            select_maxes(chars, bank, next_digit);
        }
    }
}

fn part2(input: &str) -> u64 {
    let mut total: u64 = 0;
    for line in input.lines() {
        let mut bank = "".to_owned();
        select_maxes(line, &mut bank, '9');
        println!("bank: {bank:?}");
        let joltage: u64 = bank.parse().unwrap();
        total += joltage;
    }
    println!("total: {total}");
    total
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    //part1(&input);
    part2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result, 3121910778619);
    }
}

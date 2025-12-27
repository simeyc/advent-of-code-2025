use std::fs;

fn part1(input: &str) -> i32 {
    let mut dial = 50;
    let mut password = 0;
    for line in input.lines() {
        let left = line.bytes().nth(0).unwrap() == b'L';
        let num: i32 = line[1..].to_string().parse().unwrap();
        if left {
            dial -= num;
        } else {
            dial += num;
        }
        dial %= 100;
        if dial == 0 {
            password += 1;
        }
    }
    password
}

fn part2(input: &str) -> i32 {
    let mut dial = 50;
    let mut password = 0;
    for line in input.lines() {
        let mut num: i32 = line[1..].to_string().parse().unwrap();
        // Count number of full revolutions.
        password += num / 100;
        // Move dial by remaining partial turn.
        if line.as_bytes()[0] == b'L' {
            num = -num;
        }
        let dial_before = dial;
        dial += num % 100;
        // Inc password if partial turn passed or landed on 0.
        if dial_before != 0 && (dial <= 0 || dial >= 100) {
            password += 1;
        }
        // Adjust dial position to 0-99 range.
        dial %= 100;
        if dial < 0 {
            dial += 100;
        }
    }
    password
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    //let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    println!("part1 password: {}", part1(&input));
    println!("part2 password: {}", part2(&input));
}

use std::fs;

fn part1() {
    let input = fs::read_to_string("input.txt").unwrap();
    //let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
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
    println!("password: {}", password);
}

fn main() {
    part1();
}

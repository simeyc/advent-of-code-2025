use std::fs;

fn part1() {
    let text = fs::read_to_string("input.txt").unwrap();
    let ranges = text.split(",");
    let mut total = 0;
    for range in ranges {
        let mut bounds = range.split("-");
        let start: i64 = bounds.next().unwrap().parse().unwrap();
        let end: i64 = bounds.next().unwrap().parse().unwrap();
        println!("start: {}, end:{}", start, end);
        for i in start..end {
            let i_str = i.to_string();
            if i_str.len() % 2 != 0 {
                continue; // Odd length; can't be repeated sequence.
            }
            let seq1 = &i_str[0..i_str.len() / 2];
            let seq2 = &i_str[i_str.len() / 2..i_str.len()];
            if seq1 == seq2 {
                // Repeated sequence; invalid.
                println!("invalid: {}", i);
                total += i;
            }
        }
    }
    println!("total: {}", total);
}

fn main() {
    part1();
}

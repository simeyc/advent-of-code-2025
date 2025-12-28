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

fn is_invalid(num_str: &str) -> bool {
    let len = num_str.len();
    // For odd lengths, minimum repetitions is 3; for even it's 2.
    let min_repetitions = if len.is_multiple_of(2) { 2 } else { 3 };
    'outer: for i in 1..=len / min_repetitions {
        if !len.is_multiple_of(i) {
            continue; // Repeated sequence can't be of length i.
        }
        let seq = &num_str[0..i];
        for j in 2..=len / i {
            if num_str[i * (j - 1)..i * j] != *seq {
                // Sequence not repeated; try next sequence length.
                continue 'outer;
            }
        }
        return true;
    }
    false
}

fn part2() {
    let text = fs::read_to_string("input.txt").unwrap();
    //let text = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let ranges = text.split(",");
    let mut total = 0;
    for range in ranges {
        let mut bounds = range.split("-");
        let start: i64 = bounds.next().unwrap().parse().unwrap();
        let end: i64 = bounds.next().unwrap().parse().unwrap();
        println!("start: {}, end:{}", start, end);
        for i in start..=end {
            if is_invalid(&i.to_string()) {
                println!("invalid: {}", i);
                total += i;
            }
        }
    }
    println!("total: {}", total);
}

fn main() {
    //part1();
    part2();
}

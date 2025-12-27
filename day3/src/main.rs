use std::fs;

fn part1() {
    let input = fs::read_to_string("input.txt").unwrap();
    //let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
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
}

fn main() {
    part1();
}

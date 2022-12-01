use std::io::{BufRead, BufReader};
use std::fs::File;
use std::vec::Vec;

fn main() {
    let file = File::open("input.txt").expect("Should have been able to open file.");
    let reader = BufReader::new(file);

    let mut elfindex: usize = 0;
    let mut maxindex: usize = 0;
    let mut elves: Vec<i32> = Vec::new();
    elves.push(0);

    for line in reader.lines() {
        let rline = line.expect("Line should resolve, not fail.");
        if rline == "" {
            if elves[elfindex] > elves[maxindex] {
                maxindex = elfindex;
            }

            elfindex += 1;
            elves.push(0);
        } else {
            elves[elfindex] += rline.parse::<i32>().unwrap();
        }
    }
    println!("Elf with most calories: Elf {} with {} calories.", maxindex, elves[maxindex]);

}

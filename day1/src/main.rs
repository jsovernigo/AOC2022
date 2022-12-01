use std::io::{BufRead, BufReader};
use std::fs::File;
use std::vec::Vec;

fn main() {
    let file = File::open("input.txt").expect("Should have been able to open file.");
    let reader = BufReader::new(file);

    let mut i: usize = 0;
    let mut maxindices: Vec<usize> = vec![0, 0, 0];
    let mut elves: Vec<i32> = Vec::new();
    elves.push(0);

    for line in reader.lines() {
        let rline = line.expect("Line should resolve, not fail.");
        if rline == "" {
            if elves[i] > elves[maxindices[0]] {
                // "cascade down"
                maxindices[2] = maxindices[1];
                maxindices[1] = maxindices[0];
                maxindices[0] = i;
            } else if elves[i] > elves[maxindices[1]] {
                maxindices[2] = maxindices[1];
                maxindices[1] = i;
            } else if elves[i] > elves[maxindices[2]] {
                maxindices[2] = i;
            }

            i += 1;
            elves.push(0);
        } else {
            // add this food item to the current elf's count.
            elves[i] += rline.parse::<i32>().unwrap();
        }
    }
    println!("Elves with most calories:\n\t1. Elf {} with {} calories,\n\t2. Elf {} with {} calories,\n\t3. Elf {} with {} calories.\nSum: {}", 
        maxindices[0], elves[maxindices[0]],
        maxindices[1], elves[maxindices[1]],
        maxindices[2], elves[maxindices[2]],
        elves[maxindices[0]] + elves[maxindices[1]] + elves[maxindices[2]]
    )
}


use std::fs::File;
use std::io::{BufReader, BufRead};

macro_rules! isbetween {
    ($thing:expr, $first:expr, $last:expr) => {
        ($thing >= $first && $thing <= $last)
    };
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut containsum = 0;
    let mut overlapsum = 0;
    
    for line in reader.lines() {
        let rline = line.unwrap();
	//13-18,22-41
        let halves = rline.split(",").collect::<Vec<&str>>();
        let firstindicies = halves[0].split('-').map(|i| i.parse::<u32>().ok().unwrap()).collect::<Vec<u32>>();
        let secondindicies = halves[1].split('-').map(|i| i.parse::<u32>().ok().unwrap()).collect::<Vec<u32>>();

        // part 1 framework.
        if (isbetween!(firstindicies[0], secondindicies[0], secondindicies[1]) && isbetween!(firstindicies[1], secondindicies[0], secondindicies[1])) || 
            (isbetween!(secondindicies[0], firstindicies[0], firstindicies[1]) && isbetween!(secondindicies[1], firstindicies[0], firstindicies[1])) {
            containsum += 1;
        }

        // part 2
        if isbetween!(firstindicies[0], secondindicies[0], secondindicies[1]) || isbetween!(firstindicies[1], secondindicies[0], secondindicies[1]) ||
            isbetween!(secondindicies[0], firstindicies[0], firstindicies[0]) || isbetween!(secondindicies[1], firstindicies[0], firstindicies[1]) {
            overlapsum += 1;
        }
    }

    println!("Number of containing ranges: {}", containsum);
    println!("Number of overlapping ranges: {}", overlapsum);
}

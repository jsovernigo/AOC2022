use std::fs::File;
use std::io::{BufReader, BufRead};

macro_rules! parse_move_line {
    ($line:expr) => {
        $line.split(" ")
            .map(|item| item.parse::<usize>())
            .filter(|item| item.is_ok())
            .map(|item| item.unwrap())
            .collect::<Vec<usize>>()
    };
}

fn main() {
    let solution_number = 2;

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines().map(|ln| ln.unwrap());

    // separate move and crate lines into their own iterators.
    let cratelines: Vec<String> = lines.by_ref().take_while(|ln| ln.trim().len() > 0).collect();
    let movelines: Vec<String> = lines.collect();

    // first, do some nasty split math to get the number of stacks.
    let cratestacks = cratelines.last().unwrap();
    let nstacks = cratestacks.split("   ").count();

    // then, init a vector with that many stacks.
    let mut stacks: Vec<Vec<String>> = (0..nstacks).map(|_| Vec::new()).collect();

    // for every line of crats, but in reverse (i.e. floor 1..n)
    for line in cratelines.iter().rev().skip(1) {

        // for every "floor" of crates, grab every set of 3 chars (i.e. "   " or "[X]") and convert those to a string.
        // Note that for every char, if it occurs in a 1-based-index divisible by 4, it is a separator.
        // Separators follow the line equation p_sep(x) = 4(x) + 3 (i.e., every separator occurs every 4 chars, beginning at y-int 3)
        let crates = line.chars().enumerate()
            .filter(|&(i, _)| (i + 1) % 4 != 0)
            .into_iter()
            .map(|(_, c)| c)
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();
        
        // for all the "crates" on this floor, stack them appropriately based on their column.
        for (i, c) in crates.iter().enumerate() {

            // ignore empty crates.
            if c.trim().is_empty() {
                continue;
            }

            // if, for some reason, this stack is empty, create a new vector (should never happen - off by 1 correction?)
            if i > stacks.len() {
                stacks.push(Vec::new());
            } else {
                stacks[i].push(c.to_string());
            }
        }
    }
    
    // for each "move" command line
    for line in movelines {

        // get the numbers from the line in our disgusting macro
        let numbers = parse_move_line!(line);

        // appropriately map the numbers to their roles
        let (n, s1, s2) = (numbers[0], numbers[1] - 1, numbers[2] - 1);
        
        if solution_number == 1 {
            // for each of the n crates to move,
            for _ in 0..n {

                // move them, if you can, panic (bad input) if you can't.
                match stacks[s1].pop() {
                    Some(c) => stacks[s2].push(c),
                    None => panic!("Tried to remove something that wasn't there."),
                }
            }
        } else {
            let mut transfer: Vec<String> = Vec::new();
            (0..n).for_each(|_| match stacks[s1].pop() {
                Some(c) => transfer.push(c),
                None => panic!("Tried to remove something that wasn't there."),
            });
            (0..n).for_each(|_| match transfer.pop() {
                Some(c) => stacks[s2].push(c),
                None => panic!("Tried to remove something that wasn't there."),
            });
        }
    }

    // finally, print the last member of each stack.
    for stack in stacks {
        println!("{}", stack.last().unwrap_or(&String::from("empty")));
    }

}

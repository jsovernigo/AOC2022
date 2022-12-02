use std::io::{BufRead, BufReader};
use std::fs::File;

// enum for grabbing values from R, P, S implicitly.
#[derive(Copy, Clone, PartialEq, Eq)]
enum RPSMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum RPSResult {
    Loss = 0,
    Tie = 3,
    Win = 6
}

// Opponent's moves, mapped A, B, C => Rock, Paper, Scissors.
// map moves, mapped X, Y, Z => Rock, Paper Scissors (for now...)
macro_rules! fmvmap {
    ($fmv:expr) => {
        match $fmv {
            "A" | "X" => RPSMove::Rock,
            "B" | "Y" => RPSMove::Paper,
            "C" | "Z" => RPSMove::Scissors,
            &_ => panic!("Unidentified move found in file.")
        }
    }
}

macro_rules! frmap {
    ($fmv:expr) => {
        match $fmv {
            "X" => RPSResult::Loss,
            "Y" => RPSResult::Tie,
            "Z" => RPSResult::Win,
            &_ => panic!("Unidentified result found in file.")
        }
    }
}

fn getfixedmove(opponentmove: RPSMove, outcome: RPSResult) -> RPSMove {
    match outcome {
        RPSResult::Win => match opponentmove {
            RPSMove::Rock => RPSMove::Paper,
            RPSMove::Paper => RPSMove::Scissors,
            RPSMove::Scissors => RPSMove::Rock
        },
        RPSResult::Loss => match opponentmove {
            RPSMove::Rock => RPSMove::Scissors,
            RPSMove::Paper => RPSMove::Rock,
            RPSMove::Scissors => RPSMove::Paper
        }
        RPSResult::Tie => opponentmove
    }
}

fn movevalue(mv: RPSMove) -> u32 {
    match mv {
       RPSMove::Rock => 1, 
       RPSMove::Paper => 2, 
       RPSMove::Scissors => 3 
    }
}

fn getscore(theirs: RPSMove, ours: RPSMove) -> u32 {
    if theirs == ours {
        movevalue(ours) + 3
    } else if (theirs == RPSMove::Rock && ours == RPSMove::Scissors) ||
                (theirs == RPSMove::Paper && ours == RPSMove::Rock) ||
                (theirs == RPSMove::Scissors && ours == RPSMove::Paper) {
        movevalue(ours)
    } else {
        movevalue(ours) + 6
    }
}

fn main() {

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;
    let mut fixedscore = 0;

    for line in reader.lines() {
        let rline = line.unwrap();
        let iter = rline.split(" ");
        let moves = iter.map(|mv| 
            fmvmap!(mv)
        ).collect::<Vec<RPSMove>>();

        let outcome = frmap!(rline.split(" ").last().unwrap());
        let fixedmove = getfixedmove(moves[0], outcome);
        fixedscore += getscore (moves[0], fixedmove);

        score += getscore(moves[0], moves[1]);
    }

    println!("Score 1: {}", score);
    println!("Score 2: {}", fixedscore);
}

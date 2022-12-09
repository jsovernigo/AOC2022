extern crate scanner;

use scanner::inner_scan;
use scanner::scanf;

use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;

type Point = (i32, i32);
type Delta = (i32, i32);

macro_rules! delta {
    ($x:expr, $y:expr) => {{
        let x: i32 = $x;
        let y: i32 = $y;
        (x, y)
    }}
}

macro_rules! find_delta {
    ($p1:expr, $p2:expr) => {{
        let (x1, y1): (i32, i32) = $p1;
        let (x2, y2): (i32, i32) = $p2;
        (x2 - x1, y2 - y1)
    }}
}

macro_rules! change_point_by_delta {
    ($point:expr, $delta:expr) => { {
        let (x, y): (i32, i32) = $point;
        let (dx, dy): (i32, i32) = $delta;

        (x + dx, y + dy)
    }}
}

fn get_tail_move(head: Point, tail: Point, headmove: Delta) -> Delta {
    let (dx, dy) = find_delta!(tail, head); // gets the offset from the head
    let (ndx, ndy) = find_delta!(tail, change_point_by_delta!(head, headmove));

    // i.e., the new offset is less than the must-change distance.
    if ndx.abs() <= 1 && ndy.abs() <= 1 {
        return delta!(0, 0);
    }

    // if we ever move diagonally and we were diagonal, move tail -> head
    if ndx.abs() > 1 && ndy.abs() > 1 {
        return delta!(dx, dy);
    }

    // we moved in a cardinal x direction
    if ndx.abs() > 1 {

        // but not vertically
        if ndy.abs() == 0 {
            return delta!(dx, 0);
        }
        return delta!(dx, ndy);
    }

    // we moved in a cardinal y direction
    if ndy.abs() > 1 {

        // but not horizontally
        if ndx.abs() == 0 {
            return delta!(0, dy);
        }
        return delta!(ndx, dy);
    }

    panic!("Couldn't calculate move from here.");
}

fn main() {
    let mut history_map: HashMap<Point, bool> = HashMap::new();
    let mut unique_points = 0;

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rope: Vec<Point> = Vec::new();
    for _ in 0..10 {
        rope.push((0, 0));
    }

    for line in reader.lines() {
        let rline = line.unwrap();
        let (direction, n) = scanf!(&rline, "{} {}", String, i32);

        let mut head_delta: Delta = (0, 0);

        if direction == "U" {
            head_delta = (0, 1);

        } else if direction == "L" {
            head_delta = (-1, 0);

        } else if direction == "R" {
            head_delta = (1, 0);

        } else if direction == "D" {
            head_delta = (0, -1);
        } else {
            panic!("Unknown direction {} found.", direction);
        }

        // for the number of times we need to move, 
        for _ in 0..n {

            let mut last_position = rope[0];
            rope[0] = change_point_by_delta!(rope[0], head_delta);
            
            // update each point in turn
            for i in 1..rope.len() {
                let tail_move = get_tail_move(
                    last_position,
                    rope[i],
                    find_delta!(last_position, rope[i - 1])
                );

                last_position = rope[i];
                // actually update the tail
                rope[i] = change_point_by_delta!(rope[i], tail_move);
            }

            // match the history map - it's never read, so we can ignore the Some(_) case.
            match history_map.insert(rope[rope.len() - 1], true) {
                Some(_) => (),
                None => unique_points += 1,
            }
        }

    }

    println!("Number of unique points: {}", unique_points);

}

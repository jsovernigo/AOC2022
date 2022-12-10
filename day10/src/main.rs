extern crate lazy_static;
use std::collections::HashMap ;

use lazy_static::lazy_static;
use regex::Regex;

use std::io::{BufReader, BufRead};
use std::fs::File;

lazy_static! {
    static ref RE_NOOP: Regex = Regex::new(r"noop").unwrap();
    static ref RE_ADDXI: Regex = Regex::new(r"addx (-?\d+)").unwrap();
}

enum Instruction {
    Addxi(i32),
    Noop,
    Error,
}

fn get_execution_time_for_instruction(instruction: &Instruction) -> u8 {
    match instruction {
        Instruction::Addxi(_) => 1,
        Instruction::Noop => 0,
        Instruction::Error => 0,
    }
}

fn decode_instruction(instruction: &str) -> Instruction {

    // if this is a noop line.
    if RE_NOOP.is_match(&instruction) {
        Instruction::Noop

    // if our line is an addx v instruction, 
    } else if RE_ADDXI.is_match(&instruction) {

        // attempt to capture v, and
        match RE_ADDXI.captures(&instruction) {

            // if v was captured, 
            Some(captures) => {
                let value = captures[1].parse::<i32>().unwrap();
                Instruction::Addxi(value)
            },

            // if v wasn't captured, we must panic - there was an instruction parse error.
            None => Instruction::Error
        }

    // We have no idea what this line is.
    } else {
        Instruction::Error
    }

}

fn dump_vbuffer(vbuffer: [bool; 40]) {
    for px in vbuffer {
        match px {
            false => print!("."),
            true => print!("#"),
        }
    }
    println!("");
}

fn enact_instruction(instruction: &Instruction, variables: &mut HashMap<&str, i32>) {
    match instruction {
        Instruction::Addxi(value) => {
            variables.entry("x")
                .and_modify(|x| *x += *value)
                .or_insert(*value);
        },
        Instruction::Noop => {

        },
        Instruction::Error => {
            panic!("An unreadable instruction found its way to the CPU enact() function.");
        }
    }
}

fn fetch_and_decode(mut instructions: impl Iterator<Item=(usize, String)>) -> Option<Instruction> {
    match instructions.next() {
        Some((_, line))=> {
            Some(decode_instruction(&line))
        },
        None => None
    }
}

fn simcpu(lines: impl Iterator<Item=String> , sample_at: &Vec<usize>) -> Vec<i32> {
    let mut cycle: usize = 1;

    let variables: &mut HashMap<&str, i32> = &mut HashMap::new();

    let mut instructions = lines.enumerate();

    let mut varsamples: Vec<i32> = Vec::new();
    let mut sample_index: usize = 0;

    let current_instruction: &mut Option<(Instruction, u8)> = &mut None;
    let mut vbuffer: [bool; 40] = core::array::from_fn(|_| false);

    // force insert the "x" register.
    variables.insert("x", 1);

    loop {

        // FETCH + DECODE
        // if there is no current instruction
        if current_instruction.is_none() {
            match fetch_and_decode(&mut instructions) {

                // but we can fetch one,
                Some(instruction) => {
                    match instruction {
                        Instruction::Error => panic!("Could not decode instruction."),
                        i => {
                            let execution_time = get_execution_time_for_instruction(&i);
                            *current_instruction = Some((i, execution_time));
                        }
                    }
                }

                // but if we can't we are done. Leave the loop.
                None => break,
            }
        }

        let vbuffer_index = (cycle - 1) % 40;

        let x  = match usize::try_from(*variables.get("x").unwrap()) {
            Ok(n) => n,
            Err(_) => 0,
        };

        if vbuffer_index.abs_diff(x) < 2 {
            vbuffer[vbuffer_index] = true;
        } else {
            vbuffer[vbuffer_index] = false;
        }

        // every 40 cycles, the system dumps the buffer.
        if cycle % 40 == 0 {
            dump_vbuffer(vbuffer);
            vbuffer.iter_mut().for_each(|px| *px = false);
        }

        // EXECUTE
        match current_instruction {
            // if there is some instruction...
            Some((instruction, cycles_left)) => {
                
                if *cycles_left == 0 {
                    enact_instruction(instruction, variables);
                    *current_instruction = None;
                } else {
                    *cycles_left -= 1;
                }
            },

            // there is no instruction currently loaded...
            None => panic!("Something went wrong: no current instruction but previous did not break!")
        } 

        cycle += 1;

        // sample x at predefined intervals.
        if sample_index < sample_at.len() && cycle == sample_at[sample_index] {
            match variables.get("x") {
                Some(x) => varsamples.push(*x),
                None => panic!("Variable x was unavailable at cycle {} for unknown reason.", cycle)
            }
            sample_index += 1;
        }
    }

    varsamples
}

fn main() {

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let sample_cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];

    let varsamples = simcpu(
        reader.lines()
        .map(|line| line.unwrap()), 
        &sample_cycles);


    let mut sum = 0;

    println!("");

    for (value, cycleno) in varsamples.iter().zip(sample_cycles) {
        println!("At cycle no. {}, x = {}, yielding {}", 
            cycleno, 
            value, 
            value * cycleno as i32);

        sum += value * (cycleno as i32);
    }

    println!("Sum {}", sum);
}

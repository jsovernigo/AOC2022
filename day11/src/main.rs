extern crate scanner;

use lazy_static::lazy_static;
use regex::Regex;

mod item;
mod monkey;
mod monkey_operation;

use item::ItemWorry;
use monkey::Monkey;
use monkey_operation::MonkeyOperation;

use std::collections::VecDeque;
use std::io::{BufReader, BufRead};
use std::fs::File;


lazy_static! {
    static ref RE_MONKEY_ID_LN: Regex = Regex::new(
        r"Monkey (\d+):").unwrap();

    static ref RE_STARTING_ITEMS_LN: Regex = Regex::new(
        r"\s+Starting items:(,? \d+)+").unwrap();

    static ref RE_OPERATION_LN: Regex = Regex::new(
        r"\s+Operation: new = old ([-+*/]) (old|\d+)").unwrap();

    static ref RE_TEST_LN: Regex = Regex::new(
        r"\s+Test: divisible by (\d+)").unwrap();

    static ref RE_TRUE_LN: Regex = Regex::new(
        r"\s+If true: throw to monkey (\d+)").unwrap();

    static ref RE_FALSE_LN: Regex = Regex::new(
        r"\s+If false: throw to monkey (\d+)").unwrap();
}

fn collect_n_lines(
        lines: &mut impl Iterator<Item=String>, 
        n: usize) -> Option<Vec<String>> {
    
    let mut n_lines: Vec<String> = Vec::new();
    
    for _ in 0..n {
        match lines.next() {
            Some(line) => n_lines.push(line),
            None => return None
        }
    }
        
    return Some(n_lines);
}

fn build_monkeys(lines: &mut impl Iterator<Item=String>) -> Vec<Monkey> {

    let mut monkeys: Vec<Monkey>= Vec::new();

    loop {
        match collect_n_lines(lines, 6) {

            // there were six lines to get!
            Some(monkey_lines) => {

                // start setting up the variables to capture the monkey.
                let id: usize;
                let mut starting_items: VecDeque<ItemWorry> = VecDeque::new();
                let operation: MonkeyOperation;
                let test_value: i32;
                let true_id: usize;
                let false_id: usize;

                // capture the monkey ID line.
                match RE_MONKEY_ID_LN.captures(&monkey_lines[0]) {
                    Some(captures) => {
                        //let id_test = &captures[1];
                        id = captures[1]
                            .trim()
                            .parse::<usize>()
                            .expect("Got a monkey ID that wasn't a number.");
                    },
                    None => panic!("Couldn't parse a monkey's id line.")
                }

                // capture the starting items line.
                if RE_STARTING_ITEMS_LN.is_match(&monkey_lines[1]) {
                    let number_strings = monkey_lines[1]
                        .split(":")
                        .last()
                        .unwrap()
                        .split(",");

                    for nline in number_strings {
                        starting_items.push_back(
                            nline.trim()
                            .parse::<i32>()
                            .unwrap()
                        );
                    }
                }

                // capture the operation the monkye performs.
                match RE_OPERATION_LN.captures(&monkey_lines[2]) {
                    Some(captures) => {
                        let operator = &captures[1];
                        let operand = &captures[2];
                        operation = MonkeyOperation::new(operator, operand);
                    },
                    None => panic!("Failed to parse the operation for monkey {}", id)
                } 

                // capture the test value the monkey uses to test their item.
                match RE_TEST_LN.captures(&monkey_lines[3]) {
                    Some(captures) => {
                        test_value = captures[1]
                            .parse::<i32>()
                            .expect("Could not parse test value.");
                    },
                    None => panic!("Failed to parse the test value for monkey {}", id)
                }

                match RE_TRUE_LN.captures(&monkey_lines[4]) {
                    Some(captures) => {
                        true_id = captures[1]
                            .trim()
                            .parse::<usize>()
                            .expect("Couldn't parse the ID to usize.");
                    },
                    None => panic!("Failed to parse true monkey true ID for monkey {}", id)
                }

                match RE_FALSE_LN.captures(&monkey_lines[5]) {
                    Some(captures) => {
                        false_id = captures[1]
                            .trim()
                            .parse::<usize>()
                            .expect("Couldn't parse the false ID to usize.");
                    },
                    None => panic!("Failed to parse false monkey ID for monkey {}", id)
                }

                monkeys.push(Monkey::new(
                    id, starting_items, operation, test_value, true_id, false_id
                ));

                let _ = lines.next(); // eat the newline
            }

            None => break
        }
    }
    
    monkeys
}

fn main() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader
        .lines()
        .map(|l| l.unwrap())
        .into_iter();

    let mut monkeys = build_monkeys(&mut lines);

    //for monkey in monkeys {
    //   println!("{}\n", monkey);
    //}

    for _round in 0..20 {

        for i in 0..monkeys.len() {

            println!("Monkey {}:", i);

            // process this monkey's items.
            while monkeys[i].has_items() {

                match monkeys[i].handle_next_item() {

                    // we have an item, so queue it up for another monkey.
                    Some((item, test_result)) => {
                        let monkey_id = monkeys[i].get_throw_monkey_id(test_result);
                        monkeys[monkey_id].receive_item(item);
                    }, 
                    None => panic!("Monkey {} has items but failed to yield any.", monkeys[i].get_id())
                }
            }
        }
    }

    monkeys.sort_by(
        |a, b|  {
            if a.get_items_processed() < b.get_items_processed() {
                std::cmp::Ordering::Less
            }else if a.get_items_processed() > b.get_items_processed() {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }});

    for (i, monkey) in monkeys.iter().enumerate() {
        println!("{}. {}", i, monkey.get_items_processed());
    }

}

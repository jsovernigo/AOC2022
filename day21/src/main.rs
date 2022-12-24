extern crate regex;

use regex::Regex;
use lazy_static::lazy_static;

use std::fs::File;
use std::io::{BufReader, BufRead};


use trees::{Tree, Node};

type MonkeyID = String;

lazy_static! {
    static ref RE_ASSIGN: Regex = Regex::new(
        r"(\w+): (\d+)"
    ).unwrap();

    static ref RE_EXPR: Regex = Regex::new(
        r"(\w+): (\w+) ([-+*/]) (\w+)"
    ).unwrap();
}

enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide
}

enum Expression {
    Resolved(i64),
    Unresolved((MonkeyID, Operator, MonkeyID))
}

impl Operator {
    fn new(op: &str) -> Operator {
        match op {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            anything_else => panic!("Can't parse operator {}", anything_else)
        }
    }

    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Plus => a + b,
            Operator::Minus => a - b,
            Operator::Multiply => a * b,
            Operator::Divide => a / b
        }
    }
}

fn expression_from_line(line: &str) -> (MonkeyID, Expression) {
    if RE_ASSIGN.is_match(line) {
        match RE_ASSIGN.captures(line) {
            Some(captures) => {
                let id: String = captures[1].to_string();
                let value: i64 = captures[2].parse().unwrap();

                return (id, Expression::Resolved(value));
            },
            None => panic!("Why did we match ASSIGN, but can't capture?")
        }
    } else if RE_EXPR.is_match(line) {
        match RE_EXPR.captures(line) {
            Some(captures) => {
                let id: MonkeyID = captures[1].to_string();
                let id1: MonkeyID = captures[2].to_string();
                let id2: MonkeyID = captures[4].to_string();
                let op: Operator = Operator::new(&captures[3]);

                return (id, Expression::Unresolved((id1, op, id2)));
            },
            None => panic!("Why did we match EXPR but can't capture?")
        }
    } 
    
    panic!("Cannot parse expr line {}", line);
}

fn build_tree<'a>(
    root: &'a mut Node<(MonkeyID, Expression)>, 
    nodes: &'a mut Vec<(MonkeyID, Expression)>) {
    
    let (_id, expr) = root.data();
    match expr {
        Expression::Unresolved((id1, _op, id2)) => {
            let condition: fn (&(MonkeyID, Expression), &MonkeyID) -> bool = 
                |(other_id, _), id| id == other_id;

            let position1 = nodes
                    .iter()
                    .position(|x| condition(&x, id1))
                    .expect("Could not find index of expected monkey");
            let monkey1 = nodes.remove(position1);

            let position2 = nodes
                    .iter()
                    .position(|x| condition(&x, id2))
                    .expect("Could not find index of expected monkey");
            let monkey2 = nodes.remove(position2);

            root.push_back(Tree::new(monkey1));
            root.push_back(Tree::new(monkey2));

            for child in root.iter_mut() {
                build_tree(child.get_mut(), nodes);
            }
        },
        Expression::Resolved(_) => {}
    }
}

fn resolve_tree(root: &Node<(MonkeyID, Expression)>) -> i64 {
    let (_, expr) = root.data();

    match expr {
        Expression::Resolved(n) => *n,
        Expression::Unresolved((_, op, _)) => {
            let v1 = resolve_tree(root.front().unwrap());
            let v2 = resolve_tree(root.back().unwrap());

            op.apply(v1, v2)
        }
    }
}
    
fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut i = 0;

    let mut node_list: Vec<(MonkeyID, Expression)> = Vec::new();
    let mut root_node: Option<Tree<(MonkeyID, Expression)>> = None;

    for line in reader.lines() {
        let rline = line.unwrap();

        let (id, expr) = expression_from_line(&rline);
        if id == "root" {
            root_node = Some(Tree::new((id, expr)));
        } else {
            node_list.push((id, expr));
        }

        i += 1;
    }

    println!("{} lines read.", i);

    match root_node {
        Some(mut root) => {
            let root_node = root.root_mut();
            build_tree(root_node.get_mut(), &mut node_list);
            println!("Value of root is {}", resolve_tree(root.root()));
            
        },
        None => panic!("No root node was found.")
    }

}

extern crate scanner;
use scanner::scanf;
use scanner::inner_scan;

use std::fs::File;
use std::io::{BufReader, BufRead};
//use trees::Tree;

fn main() {

    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);

    //let ftree = Tree::new();

    let mut currentdirectory = String::from("/");

    for line in reader.lines() {
        let rline = line.unwrap();

        println!("Got line {}", rline);

        // cmd line.
        if rline.starts_with("$") {
            let (fname,) = scanf!(&rline, "$ cd {}", String);
            currentdirectory = fname;
        // file output line.
        } else {
            let (size, name) = scanf!(&rline, "{} {}", usize, String);
            println!("Found file {} of size {} in dir {}.", name, size, currentdirectory);
        }

    }
    

}

use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    
    // 4 for start-of-coms markers, 14 for start-of-message
    let marker_length = 14; 

    // for each line in the coms file (see test.txt)
    for line in reader.lines() {

        // check if line unwraps from error
        match line {

            // in the valid case,
            Ok(l) => {
                    let mut i = 0;
                    let mut iter = l.bytes();
                    let mut history: Vec<u8> = Vec::new();

                    // collect the minimum history
                    for _i in 0..(marker_length - 1) {

                        match iter.next() {

                            // a new history character
                            Some(n) => {
                                    history.insert(0, n);
                                    i += 1
                                },

                            // too little coms content for a marker to exist
                            None => panic!("too few items to populate history."),
                        }
                    }

                    
                    // loop for the rest of the line
                    loop {

                        match iter.next() {

                            // a new character occurred
                            Some(n) =>

                                // if this character exists in the history, or
                                if history.contains(&n) || 

                                        // history contains any element i
                                        history
                                        .iter()
                                        .any(|i| 
                                            
                                            // that if filtered by, results in more than one result
                                            history.iter()
                                                .filter(|&o| *o == *i)
                                                .count() > 1) {

                                    // get rid of oldest history character and insert the newest.
                                    history.pop();
                                    history.insert(0, n);
                                    i += 1;

                                // i.e. no characters are repeated in history: marker found.
                                } else {
                                    println!("First marker found at {}", i + 1);
                                    break;
                                },

                            // this line does not contain a marker.
                            None =>  {
                                println!("this line does not contain a marker.");
                                break;
                            }
                        }
                    }

                },

            // line had some error occur during assignment - this is likely EOF.
            Err(_) =>
                break,
        }
    }
}

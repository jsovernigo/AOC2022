extern crate scanner;
use scanner::scanf;
use scanner::inner_scan;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::pin::Pin;


use trees::{Tree, Node};

struct Directory {
    name: String,
    files: Vec<(String, usize)>,
    cachedsize: Option<usize>,
}

impl std::fmt::Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Dir {} with {} files\n", self.name, self.files.len())

    }
}

impl Directory {
    fn new(name: String) -> Directory {
        return Directory { 
            name: name,
            files: Vec::new(), 
            cachedsize: None,
        }
    }
    
    fn update_cached_size(&mut self, newsize: usize) {
        match self.cachedsize {
            Some(n) => self.cachedsize = Some(n + newsize),
            None => self.cachedsize = Some(newsize)
        }
    }

    fn add(&mut self, filename: String, size: usize) {
        self.files.push((filename, size));
        self.update_cached_size(size);
    }

    fn get_cached_size(&self) -> usize {
        return match self.cachedsize {
            Some(size) => size,
            None => 0,
        };
    }

}


fn construct_tree_from_lines(
    line_iter: &mut impl Iterator <Item = Result<String, std::io::Error>>, 
    mut current: Pin<&mut Node<Directory>>) {

    loop {
        match line_iter.next() {
            Some(line) =>
                match line {
                    Ok(rline) => {

                        // line with a command.
                        if rline.contains("$") {

                            if rline.contains("cd") {
                                let (dirname, ) = scanf!(&rline, "$ cd {}", String);
                              
                                // abort from this depth.
                                if dirname == ".." {
                                    return
                                }

                                // otherwise, just descend to the next level.
                                for child in current.iter_mut() {
                                    if child.data().name == dirname {
                                        construct_tree_from_lines(line_iter, child);
                                    }
                                }
                                
                            } else if rline.contains("ls") {
                                // ignore, currently.
                            }

                        // line with a "dir" entry.
                        } else if rline.contains("dir") {
                            let (dirname, ) = scanf!(&rline, "dir {}", String);
                            current.push_back(
                                Tree::new(
                                Directory::new(dirname)
                                )
                            );

                        // line with a file entry.
                        } else {
                            let (size, name) = scanf!(&rline, "{} {}", usize, String);
                            current.data_mut().add(name, size);
                        }
                    },
                    Err(_) =>
                        panic!("Why did we encounter an io error during reading?")
                }

            None => break,
        }
    }

}

fn get_sum_of_directory(current: &Node<Directory>) -> usize {
    if current.has_no_child() {
        return current.data().get_cached_size();
    } else {
        let mut directorysize: usize = current.data().get_cached_size();

        for child in current.iter() {
            directorysize += get_sum_of_directory(child);
        }
        directorysize
    }
}

fn get_sum_less_than_100k(current: &Node<Directory>) -> usize {
    let mut less_than_100ksum: usize = 0;
    let directory_sum: usize = get_sum_of_directory(current);

    for child in current.iter() {
        less_than_100ksum += get_sum_less_than_100k(child);
    }

    if directory_sum < 100_000 {
        less_than_100ksum += directory_sum;
    }

    less_than_100ksum
}

fn find_smallest_directory_greater_than_n(current: &Node<Directory>, n: usize) -> Option<(&Node<Directory>, usize)> {
    
    // we may need to choose the current directory of our current tree if no child > n
    let dir_size: usize = get_sum_of_directory(current);

    // base case - can't find a sufficient directory if we are too small.
    if dir_size < n {
        return None;
    }

    // base case - we are a leaf dir, do we have big files?
    if current.has_no_child() {
        if dir_size > n {
            return Some((current, dir_size));
        } else {
            return None;
        }

    }

    // assume we are the smallest until we find a descendant that proves otherwise.
    let mut smallest_descendant_gt_n = (current, dir_size);

    // for each child in the current directory
    for child in current.iter() {
        let (_, smallest_size) = smallest_descendant_gt_n;

        // find the smallest descendant for each child, and
        match find_smallest_directory_greater_than_n(child, n) {

            // if one exists, 
            Some((descendant, descendant_size)) =>

                // check if it is smaller than the smallest recorded
                if descendant_size < smallest_size {
                    smallest_descendant_gt_n = (descendant, descendant_size);
                }, 

            // if none exist, we can just continue.
            None => continue,
        }

    }

    Some(smallest_descendant_gt_n)
}

fn main() {

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut ftree: Tree<Directory> = Tree::new(Directory::new("/".to_string()));

    construct_tree_from_lines(&mut reader.lines().skip(1), ftree.root_mut());

    let ftree = ftree;
    println!("{}", get_sum_less_than_100k(ftree.root()));

    let total_space: usize = 70_000_000;
    let occupied_space = get_sum_of_directory(ftree.root());
    let free_space = total_space - occupied_space;
    let update_size: usize = 30_000_000;
    let needed_space: usize = update_size - free_space;

    let smallest_descendant_gt_n = 
        find_smallest_directory_greater_than_n(
            ftree.root(), 
            needed_space);
   
    match smallest_descendant_gt_n {
        Some((d, dsize)) => 
            println!("{}: {}", d.data().name, dsize),
        
        None =>
            println!("No smallest node > n found.")
    }


}

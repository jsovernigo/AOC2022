use std::io::{BufReader, BufRead};
use std::fs::File;

use std::num;
use std::vec::Vec;

struct Grove (Vec<Vec<u8>>);

impl Grove {
    fn new() -> Grove {
        Grove(Vec::new())
    }

    fn newrow(&mut self) {
        self.0.push(Vec::new());
    }

    fn newtree(&mut self, tree: u8) {
        let lastrow = self.0.last_mut();
        match lastrow {
            Some(row) => row.push(tree),
            None => panic!("Tried to add a tree to a grove with no rows."),
        }
    }

    fn get(&self, row: usize, column: usize) -> u8 {
        assert!(row < self.0.len());
        assert!(column < self.0[row].len());
        self.0[row][column]
    }

    fn get_perimiter(&self) -> usize {
        // This assumes a square grove, always.
        self.0.len() * self.0[0].len()
    }

    fn get_n_rows(&self) -> usize {
        self.0.len()
    }

    fn get_n_columns(&self) -> usize {
        self.0[0].len()
    }
}

impl std::fmt::Display for Grove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in &self.0 {
            for tree in row  {
                match write!(f, "{}", tree) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
            match write!(f, "\n") {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

        }

        Ok(())
    }
}


fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grove: Grove = Grove::new();

    for line in reader.lines() {

        // initialize this row of the grove.
        grove.newrow();

        let rline = line.unwrap();
        let treerow = rline.trim().bytes();

        for tree in treerow {
            grove.newtree(tree - b'0');
        }
    }


    let mut number_visible_trees = 0;
    //number_visible_trees += grove.get_perimiter(); // perimiter trees are ALWAYS visible.

    let mut visible_grid: Vec<Vec<bool>> = Vec::new();
    for i in 0..grove.get_n_rows() {
        visible_grid.push(Vec::new());
        for _ in 0..grove.get_n_columns() {
            visible_grid[i].push(false);
        }
    }

    // left/right visibility
    for i in 0..grove.get_n_rows() {
        let mut lmax = grove.get(i, 0);
        let mut rmax = grove.get(i, grove.get_n_columns() - 1);

        // first and last tree are always visible.
        visible_grid[i][0] = true;
        visible_grid[i][grove.get_n_columns() - 1] = true;
        
        // left 
        for j in 1..(grove.get_n_columns() - 1) {
            let tree = grove.get(i, j);
            if tree > lmax {
                visible_grid[i][j] = visible_grid[i][j] || true;
                lmax = tree;
            }
        }
    
        // right
        for j in (1..(grove.get_n_columns() - 1)).rev() {
            let tree = grove.get(i, j);
            if tree > rmax {
                visible_grid[i][j] = visible_grid[i][j] || true;
                rmax = tree;
            }
        }
    }

    // up/down visibility
    for j in 0..grove.get_n_columns() {
        let mut dmax = grove.get(0, j);
        let mut umax = grove.get(grove.get_n_rows() - 1, j);

        visible_grid[0][j] = true;
        visible_grid[grove.get_n_rows() - 1][j] = true;

        // down first
        for i in 1..(grove.get_n_rows()) {
            let tree = grove.get(i, j);
            if tree > dmax {
                visible_grid[i][j] = visible_grid[i][j] || true;
                dmax = tree;
            }
        }
        
        // then up
        for i in (1..(grove.get_n_rows())).rev() {
            let tree = grove.get(i, j);
            if tree > umax {
                visible_grid[i][j] = visible_grid[i][j] || true;
                umax = tree;
            }
        }
    }

    // finally, count all the entries that are "true"
    for i in 0..grove.get_n_rows() {
        for j in 0..grove.get_n_columns() {
            if visible_grid[i][j] {
                number_visible_trees += 1;
            }
        }
    }

    println!("{}", number_visible_trees);

}

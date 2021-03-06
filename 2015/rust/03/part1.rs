// adventofcode - day 3
// part 1

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

fn main(){
    println!("Advent of Code - day 3 | part 1");

    // import data
    let steps = import_data();

    // we'll use a HasMap to keep track of the points (used as keys in the map)
    // we already visited -> Every point can only be stored once. Therefore, at
    // the end we can simply read out the number of stored locations
    let mut visited = HashMap::new();
    let mut santa = Point::new(0,0);

    // save the initial point
    visited.insert( Point::new(0, 0), true );
    for step in steps.chars(){

        match step {
            '>' => santa.x += 1,
            '<' => santa.x += -1,
            '^' => santa.y += 1,
            'v' => santa.y += -1,
            _ => {},
        }

        // save Santas current position
        // Note: We have to create a new point because of Rust's move & borrow
        // concepts
        visited.insert( Point::new(santa.x, santa.y), true);
    }

    println!("Santa visited {} unique houses", visited.len() );
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("input.txt") {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    let mut data = String::new();
    match file.read_to_string(&mut data){
        Ok(_) => {},
        Err(e) => panic!("file error: {}", e),
    };

	data
}

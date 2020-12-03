use std::io::BufReader;
use std::{error::Error, fs::File, io::prelude::*};
fn main() -> Result<(), Box<dyn Error>> {
    let input_file = File::open("day03/input.txt")?;
    let mut trees = 0;
    let mut right_index = 0;
    for line in BufReader::new(input_file).lines() {
        let line = line?;
        let line_length = line.len();
        let maybe_tree = line.chars().nth(right_index % line_length).unwrap();

        if maybe_tree == '#' {
            trees += 1;
        }
        right_index += 3;
    }
    println!("{}", trees);
    Ok(())
}

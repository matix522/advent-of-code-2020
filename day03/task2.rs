use std::io::{BufReader, SeekFrom};
use std::{error::Error, fs::File, io::prelude::*};
fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("day03/input.txt")?;

    let mut result = 1;
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for (right_change, down_change) in slopes {
        input_file.seek(SeekFrom::Start(0))?;
        let mut trees = 0;
        let mut right_index = 0;
        for line in BufReader::new(&input_file)
            .lines()
            .enumerate()
            .filter(|&(i, _)| i % down_change == 0)
            .map(|(_, s)| s)
        {
            let line = line?;
            let line_length = line.len();
            let maybe_tree = line.chars().nth(right_index % line_length).unwrap();

            if maybe_tree == '#' {
                trees += 1;
            }
            right_index += right_change;
        }
        result *= trees;
    }
    println!("{}", result);
    Ok(())
}

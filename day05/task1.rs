use std::{cmp::max, cmp::min, io::BufReader};
use std::{error::Error, fs::File, io::prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = File::open("day05/input.txt")?;
    let mut max_id = 0;
    let mut min_id = 1024;
    for line in BufReader::new(input_file).lines() {
        let line = line?;
        let (row, seat) = line.split_at(7);
        let mut row_id = 0;
        let mut seat_id = 0;
        for (i, c) in row.char_indices() {
            if c == 'B' {
                row_id += 2usize.pow(6 - i as u32);
            }
        }
        for (i, c) in seat.char_indices() {
            if c == 'R' {
                seat_id += 2usize.pow(2 - i as u32);
            }
        }
        let id = 8 * row_id + seat_id;
        max_id = max(max_id, id);
        min_id = min(min_id, id);
    }
    println!("min: {} max: {}", min_id, max_id);
    Ok(())
}

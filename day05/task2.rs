use std::{io::BufReader};
use std::{error::Error, fs::File, io::prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = File::open("day05/input.txt")?;
    let max_id = 933;
    let _min_id = 6;
    let mut ids = vec![false; max_id + 1];
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
        ids[id] = true;
    }
    let id: Vec<usize> = ids
        .iter()
        .enumerate()
        .filter(|(_, &val)| !val)
        .map(|(idx, _)| idx)
        .collect();
    println!("{:?}", id);
    Ok(())
}

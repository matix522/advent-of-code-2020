use std::collections::BTreeSet;
use std::{error::Error, fs::File, io::prelude::*};
fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("day06/input.txt")?;

    let mut data = String::new();

    input_file.read_to_string(&mut data)?;
    let data = data;

    let mut question_count = 0;

    for group in data.split("\n\n") {
        let mut group_awnsers: BTreeSet<char> = BTreeSet::new();
        for c in group.chars().filter(|c| c.is_alphabetic()) {
            group_awnsers.insert(c);
        }
        question_count += group_awnsers.len();
    }
    println!("{}", question_count);
    Ok(())
}

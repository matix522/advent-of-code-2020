use std::collections::BTreeSet;
use std::{error::Error, fs::File, io::prelude::*};
fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("day06/input.txt")?;

    let mut data = String::new();

    input_file.read_to_string(&mut data)?;
    let data = data;

    let mut question_count = 0;

    for group in data.split("\n\n") {
        let mut group_awnsers: Option<BTreeSet<char>> = None;
        for person in group.split('\n') {
            let mut person_awnsers: BTreeSet<char> = BTreeSet::new();
            for c in person.chars().filter(|c| c.is_alphabetic()) {
                person_awnsers.insert(c);
            }
            if let Some(awnsers) = &mut group_awnsers {
                *awnsers = awnsers.intersection(&person_awnsers).map(|&c| c).collect();
            } else {
                group_awnsers = Some(person_awnsers);
            }
        }
        if let Some(awnsers) = &mut group_awnsers {
            question_count += awnsers.len();
        }
    }
    println!("{}", question_count);
    Ok(())
}

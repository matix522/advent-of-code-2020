use std::io::BufReader;
use std::str::FromStr;
use std::{error::Error, fs::File, io::prelude::*};
fn get_next<'a, T>(iter: &mut impl Iterator<Item = &'a str>) -> Result<T, <T as FromStr>::Err>
where
    T: FromStr,
{
    iter.next().unwrap().parse::<T>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = File::open("day02/input.txt")?;
    let mut correct_password_counter = 0;
    for line in BufReader::new(input_file).lines() {
        let line = line?;
        let mut elements = line.split(|c| c == '-' || c == ':' || c == ' ');

        let first_index_plus_one: usize = get_next(&mut elements)?;
        let second_index_plus_one: usize = get_next(&mut elements)?;

        let first_index: usize = first_index_plus_one - 1;
        let second_index: usize = second_index_plus_one - 1;

        let letter: char = get_next(&mut elements)?;
        let password: &str = elements.skip(1).next().unwrap();

        let correct_letter_count = password
            .char_indices()
            .filter(|&(i, _)| first_index == i || second_index == i)
            .filter(|&(_, c)| c == letter)
            .count();

        if correct_letter_count == 1 {
            correct_password_counter += 1;
        }
    }
    println!("{}", correct_password_counter);
    Ok(())
}

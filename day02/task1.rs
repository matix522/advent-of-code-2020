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

        let lower_bound: usize = get_next(&mut elements)?;
        let upper_bound: usize = get_next(&mut elements)?;
        let letter: char = get_next(&mut elements)?;
        let password: &str = elements.skip(1).next().unwrap();
        
        let letter_count = password.chars().filter(|&c| c == letter).count();

        if  (lower_bound..=upper_bound).contains(&letter_count) {
            correct_password_counter += 1;
        }
    }
    println!("{}", correct_password_counter);
    Ok(())
}

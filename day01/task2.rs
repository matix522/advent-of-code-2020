use std::io::BufReader;
use std::{error::Error, fs::File, io::prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = File::open("day01/input.txt")?;
    let mut numbers: Vec<u32> = Vec::new();
    for line in BufReader::new(input_file).lines() {
        numbers.push(line?.parse()?);
    }
    let numbers = numbers;
    for &i in &numbers {
        for &j in &numbers {
            for &k in &numbers {
                if i + j + k == 2020 {
                    println!("{}", i * j * k);
                    return Ok(());
                }
            }
        }
    }
    println!("None");
    Ok(())
}

use regex::Regex;
use std::collections::BTreeSet;
use std::{error::Error, fs::File, io::prelude::*};
fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("day04/input.txt")?;

    let required = vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
    let _optional = vec!["cid"];

    let eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    let mut data = String::new();

    input_file.read_to_string(&mut data)?;
    let data = data;

    let mut correct_password_count = 0;

    for passport_data in data.split("\n\n") {
        let mut passport: BTreeSet<String> = BTreeSet::new();
        for entry in passport_data.split_whitespace() {
            let mut split = entry.split(':');
            let key = split.next().unwrap().to_owned();
            let value = split.next().unwrap().to_owned();

            let is_correct = match &key[..] {
                "byr" => {
                    let year: usize = value.parse()?;
                    (1920..=2002).contains(&year)
                }
                "iyr" => {
                    let year: usize = value.parse()?;
                    (2010..=2020).contains(&year)
                }
                "eyr" => {
                    let year: usize = value.parse()?;
                    (2020..=2030).contains(&year)
                }
                "hgt" => {
                    let len = value.len();
                    let cm_regex = Regex::new("^1[0-9][0-9]cm$")?;
                    let in_regex = Regex::new("^[0-9][0-9]in$")?;
                    if cm_regex.is_match(&value[..]) {
                        let height: usize = value[..len - 2].parse()?;
                        (150..=193).contains(&height)
                    } else if in_regex.is_match(&value[..]) {
                        let height: usize = value[..len - 2].parse()?;
                        (59..=76).contains(&height)
                    } else {
                        false
                    }
                }
                "hcl" => {
                    let regex = Regex::new("^#[0-9a-f]{6,6}$")?;
                    regex.is_match(&value[..])
                }
                "ecl" => eye_colors.contains(&&value[..]),
                "pid" => {
                    let regex = Regex::new("^[0-9]{9,9}$")?;
                    regex.is_match(&value[..])
                }
                _ => false,
            };
            if is_correct {
                passport.insert(key);
            }
        }
        if passport
            .iter()
            .filter(|&key| required.contains(&&key[..]))
            .count()
            == required.len()
        {
            correct_password_count += 1;
        }
    }
    println!("{}", correct_password_count);
    Ok(())
}

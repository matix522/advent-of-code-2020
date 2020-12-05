use std::collections::BTreeMap;
use std::{error::Error, fs::File, io::prelude::*};
fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("day04/input.txt")?;

    let required = vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
    let _optional = vec!["cid"];

    let mut data = String::new();

    input_file.read_to_string(&mut data)?;
    let data = data;

    let mut correct_password_count = 0;

    for passport_data in data.split("\n\n") {
        let mut passport: BTreeMap<String, String> = BTreeMap::new();
        for entry in passport_data.split_whitespace() {
            let mut split = entry.split(':');
            let key = split.next().unwrap().to_owned();
            let value = split.next().unwrap().to_owned();
            passport.insert(key, value.to_owned());
        }
        if passport
            .keys()
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

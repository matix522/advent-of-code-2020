use regex::Regex;
use std::{error::Error, fs::File, io::prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("day07/input.txt")?;

    let mut data = String::new();

    input_file.read_to_string(&mut data)?;
    let data = data;

    let left_bag_regex = Regex::new(r"(.*) (.*) bag").unwrap();
    let right_bag_regex = Regex::new(r"(\d+) (.*) (.*) bag").unwrap();

    let left_bag_info = |bag_description| -> Option<_> {
        let capt = left_bag_regex.captures_iter(bag_description).next()?;
        Some(format!("{} {}", &capt[1], &capt[2]))
    };

    let right_bag_info = |bag_description| -> Option<_> {
        let capt = right_bag_regex.captures_iter(bag_description).next()?;
        Some((
            capt[1].parse::<usize>().ok()?,
            format!("{} {}", &capt[2], &capt[3]),
        ))
    };

    let graph: Vec<_> = data
        .split("\n")
        .map(|rule| {
            let mut split = rule.split(" contain ");
            let outer = split.next().and_then(left_bag_info).unwrap();
            let inner: Vec<_> = split
                .next()
                .unwrap()
                .split(',')
                .map(right_bag_info)
                .filter_map(|x| x)
                .collect();

            (outer, inner)
        })
        .collect();

    let mut count = 0;
    for (name, _) in &graph {
        let mut closed_set = Vec::new();
        let mut open_queue = Vec::new();
        open_queue.push(name);

        'dfs: while !open_queue.is_empty() {
            let bag = open_queue.pop().unwrap();
            closed_set.push(bag);

            let (_, inner_bags) = graph.iter().find(|(outer, _)| outer == bag).unwrap();
            for (_, name) in inner_bags {
                if name == "shiny gold" {
                    count += 1;
                    break 'dfs;
                }
                if !closed_set.contains(&name) && !open_queue.contains(&name) {
                    open_queue.push(name);
                }
            }
        }
    }
    // println!("{}", closed_set.len());
    println!("{}", count);
    Ok(())
}

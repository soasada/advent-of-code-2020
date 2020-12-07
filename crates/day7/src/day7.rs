use files::load_str_vec_from;
use std::collections::HashMap;
use regex::{Regex, Match};

pub fn problem1() -> i64 {
    let bags_restrictions_loaded = load_str_vec_from("crates/day7/src/day7_input.txt");

    if let Ok(bags_restrictions) = bags_restrictions_loaded {
        let m: HashMap<_, _> = bags_restrictions.iter()
            .map(|line| line.split(" contain ").collect::<Vec<&str>>())
            .map(|v| (v[0].replace(" bags", ""), parse_children_bags(v[1])))
            .collect();

        let a = 1;
    }

    0
}

pub fn problem2() -> i64 {
    0
}

fn parse_children_bags(children_line: &str) -> Vec<&str> {
    return Regex::new(r"(\d) ([a-z ]+) bag[s]?[,.]?").unwrap()
        .captures_iter(children_line)
        .map(|c| c.get(2).unwrap().as_str())
        .collect::<Vec<&str>>();
}
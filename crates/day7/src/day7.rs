use files::load_str_vec_from;
use std::collections::{HashMap, HashSet};
use regex::Regex;

pub fn problem1() -> i64 {
    let bags_restrictions_loaded = load_str_vec_from("crates/day7/src/day7_input.txt");

    if let Ok(bags_restrictions) = bags_restrictions_loaded {
        let m: HashMap<_, _> = bags_restrictions.iter()
            .map(|line| line.split(" contain ").collect::<Vec<&str>>())
            .map(|v| (v[0].replace(" bags", ""), parse_children_bags(v[1])))
            .collect();

        let mut result = HashSet::new();
        for b in m.keys() {
            visit_bags(&m, b, &mut result);
        }

        return result.len() as i64;
    }

    0
}

pub fn problem2() -> i64 {
    let bags_restrictions_loaded = load_str_vec_from("crates/day7/src/day7_input.txt");

    if let Ok(bags_restrictions) = bags_restrictions_loaded {
        let m: HashMap<_, _> = bags_restrictions.iter()
            .map(|line| line.split(" contain ").collect::<Vec<&str>>())
            .map(|v| (v[0].replace(" bags", ""), parse_children_bags(v[1])))
            .collect();

        return visit_bags2(&m, "shiny gold");
    }

    0
}

fn parse_children_bags(children_line: &str) -> Vec<(&str, i64)> {
    return Regex::new(r"(\d) ([a-z ]+) bag[s]?[,.]?").unwrap()
        .captures_iter(children_line)
        .map(|c| (c.get(2).unwrap().as_str(), c.get(1).unwrap().as_str().parse::<i64>().unwrap()))
        .collect::<Vec<(&str, i64)>>();
}

fn visit_bags(bags: &HashMap<String, Vec<(&str, i64)>>, bag_name: &String, result: &mut HashSet<String>) -> bool {
    for c in bags.get(bag_name).unwrap().iter() {
        if (*c).0 == "shiny gold" {
            result.insert(bag_name.clone());
            return true;
        } else if visit_bags(bags, &String::from((*c).0), result) {
            result.insert(bag_name.clone());
            return true;
        }
    }

    return false;
}

fn visit_bags2(bags: &HashMap<String, Vec<(&str, i64)>>, bag_name: &str) -> i64 {
    let mut cont = 0;
    for c in bags.get(bag_name).unwrap().iter() {
        cont += (*c).1 + (*c).1 * visit_bags2(bags, (*c).0);
    }

    return cont;
}
use files::load_str_vec_from;
use std::collections::HashSet;
use std::panic::resume_unwind;

pub fn problem1() -> i64 {
    return load_declaration_forms().iter()
        .map(|form| form.chars().collect::<HashSet<_>>())
        .collect::<Vec<HashSet<_>>>().iter()
        .fold(0, |acc, s| acc + s.len()) as i64;
}

pub fn problem2() -> i64 {
    let declaration_forms_loaded = load_str_vec_from("crates/day6/src/day6_input.txt");

    if let Ok(declaration_forms) = declaration_forms_loaded {
        let mut responses = Vec::new();
        for line in declaration_forms.iter() {
            if line != "" {
                responses.push(line.chars().collect::<HashSet<_>>());
            } else {
                let x = responses.iter()
                    .fold(HashSet::<char>::new(), |acc, s| {
                        let y = acc.intersection(&s).cloned().collect();
                        return y;
                    })
                    .len();
                let a = 1;
                let b = 1;
            }
        }
    }
    0
}

fn load_declaration_forms() -> Vec<String> {
    let declaration_forms_loaded = load_str_vec_from("crates/day6/src/day6_input.txt");

    if let Ok(declaration_forms) = declaration_forms_loaded {
        let mut group = String::new();
        let mut groups = Vec::new();

        for line in declaration_forms.iter() {
            if line != "" {
                group.push_str(line);
            } else {
                groups.push(group);
                group = String::new();
            }
        }

        groups.push(group);
        return groups;
    }

    return Vec::new();
}
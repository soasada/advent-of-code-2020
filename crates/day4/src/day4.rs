use files::load_str_vec_from;
use std::collections::HashMap;
use regex::Regex;

pub fn problem1() -> i64 {
    let mut valid_passports = 0;
    let passport_fields = load_passport_fields();

    let mut checked_fields = 0;
    let mut contains_cid = false;

    for (i, field) in passport_fields.iter().enumerate() {
        if *field != "" {
            if field.contains("cid") {
                contains_cid = true;
            }

            for required_field in required_fields().keys() {
                if field.contains(required_field) {
                    checked_fields += 1;
                    break;
                }
            }
        } else {
            if checked_fields >= 7 && !contains_cid {
                valid_passports += 1;
            } else if checked_fields >= 8 {
                valid_passports += 1;
            }
            checked_fields = 0;
            contains_cid = false;
        }

        if i == passport_fields.len() - 1 {
            if checked_fields >= 7 && !contains_cid {
                valid_passports += 1;
            } else if checked_fields >= 8 {
                valid_passports += 1;
            }
        }
    }


    valid_passports
}

pub fn problem2() -> i64 {
    0
}

fn load_passport_fields() -> Vec<String> {
    let passports_loaded = load_str_vec_from("crates/day4/src/day4_input.txt");

    if let Ok(passports) = passports_loaded {
        return passports.iter()
            .flat_map(|str| str.split(" "))
            .map(str::to_owned)
            .collect();
    }

    return Vec::new();
}

fn required_fields() -> HashMap<&'static str, fn(&str) -> bool> {
    let mut fields_validations: HashMap<&str, fn(&str) -> bool> = HashMap::new();
    fields_validations.insert("byr", |val| {
        if val.len() == 4 {
            if let Ok(birth_year) = val.parse::<i32>() {
                birth_year >= 1920 && birth_year <= 2002
            } else {
                false
            }
        } else {
            false
        }
    });

    fields_validations.insert("iyr", |val| {
        if val.len() == 4 {
            if let Ok(issue_year) = val.parse::<i32>() {
                issue_year >= 2010 && issue_year <= 2020
            } else {
                false
            }
        } else {
            false
        }
    });

    fields_validations.insert("eyr", |val| {
        if val.len() == 4 {
            if let Ok(expiration_year) = val.parse::<i32>() {
                expiration_year >= 2020 && expiration_year <= 2030
            } else {
                false
            }
        } else {
            false
        }
    });

    fields_validations.insert("hgt", |val| {
        if val.contains("cm") {
            if let Ok(cms) = val.replace("cm", "").parse::<i32>() {
                if cms >= 150 && cms <= 193 {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else if val.contains("in") {
            if let Ok(ins) = val.replace("in", "").parse::<i32>() {
                if ins >= 59 && ins <= 76 {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    });

    fields_validations.insert("hcl", |val| {
        let re = Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$").unwrap();
        return re.is_match(val);
    });

    fields_validations.insert("ecl", |val| {
        let eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        return eye_colors.contains(&val);
    });

    fields_validations.insert("pid", |val| {
        if val.len() == 9 {
            match val.parse::<i32>() {
                Ok(_n) => true,
                _ => false
            }
        } else {
            false
        }
    });

    fields_validations.insert("cid", |_val| true);

    // println!("Resutl: {}", fields_validations.get("pid").unwrap()("0123456789"));

    fields_validations
}
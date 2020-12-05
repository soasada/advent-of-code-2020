use files::load_str_vec_from;

pub fn problem1() -> i64 {
    let passports_loaded = load_str_vec_from("crates/day4/src/day4_input.txt");
    let mut valid_passports = 0;
    let mut passport_fields = Vec::new();

    if let Ok(passports) = passports_loaded {
        passports.iter()
            .map(|line| line.split(" ").collect::<Vec<&str>>())
            .for_each(|mut vec| passport_fields.append(&mut vec));

        let mut checked_fields = 0;
        let mut contains_cid = false;

        for (i, field) in passport_fields.iter().enumerate() {
            if *field != "" {
                if field.contains("cid") {
                    contains_cid = true;
                }

                for required_field in required_fields().iter() {
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
    }

    valid_passports
}

pub fn problem2() -> i64 {
    0
}

fn required_fields() -> [&'static str; 8] {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
}
use files::load_str_vec_from;
use regex::Regex;

pub fn problem1() -> i32 {
    let passwords_vec = load_str_vec_from("crates/day2/src/day2_input.txt");
    let passwords = match passwords_vec {
        Ok(p) => p,
        Err(e) => panic!("Problem with the vector: {:?}", e),
    };
    let mut valid_passwords = 0;
    let re = Regex::new(r"(\d{1,2})-(\d{1,2}) ([a-z])").unwrap();

    for password in passwords.iter() {
        let p = password.split(":").collect::<Vec<&str>>();
        let password_conditions = re.captures(p[0]).unwrap();
        let min = password_conditions[1].parse::<i32>().unwrap();
        let max = password_conditions[2].parse::<i32>().unwrap();
        let letter = password_conditions[3].parse::<char>().unwrap();
        let full_password = p[1];

        let mut occurrences = 0;

        for c in full_password.chars() {
            if c == letter {
                occurrences = occurrences + 1;
            }
        }

        if occurrences >= min && occurrences <= max {
            valid_passwords = valid_passwords + 1;
        }
    }
    valid_passwords
}

pub fn problem2() -> i64 {
    let passwords_vec = load_str_vec_from("crates/day2/src/day2_input.txt");
    let passwords = match passwords_vec {
        Ok(p) => p,
        Err(e) => panic!("Problem with the vector: {:?}", e),
    };
    let mut valid_passwords = 0;
    let re = Regex::new(r"(\d{1,2})-(\d{1,2}) ([a-z])").unwrap();

    for password in passwords.iter() {
        let p = password.split(":").collect::<Vec<&str>>();
        let password_conditions = re.captures(p[0]).unwrap();
        let pos1 = password_conditions[1].parse::<usize>().unwrap();
        let pos2 = password_conditions[2].parse::<usize>().unwrap();
        let letter = password_conditions[3].parse::<char>().unwrap();
        let full_password = p[1].trim();

        let mut occurrences = 0;

        for (i, c) in full_password.chars().enumerate() {
            if c == letter && ((i + 1) == pos1 || (i + 1) == pos2) {
                occurrences = occurrences + 1;
            }
        }

        if occurrences == 1 {
            valid_passwords = valid_passwords + 1;
        }
    }
    valid_passwords
}
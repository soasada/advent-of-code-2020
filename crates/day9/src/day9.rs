use files::load_uint_vec_from;
use std::ptr::replace;

pub fn problem1() -> i64 {
    let numbers_loaded = load_uint_vec_from("crates/day9/src/day9_input.txt");

    if let Ok(numbers) = numbers_loaded {
        let mut i = 25;
        let mut found = false;
        let mut found_number = 0;

        while !found {
            let number = numbers[i];
            let mut has_sum = false;
            for n in numbers[0..i].iter() {
                for j in numbers[0..i].iter() {
                    if n + j == number {
                        has_sum = true;
                        break;
                    }
                }

                if has_sum {
                    break;
                }
            }

            if !has_sum {
                found = true;
                found_number = number;
            } else {
                has_sum = false;
                i += 1;
            }
        }

        return found_number;
    }

    0
}

pub fn problem2() -> i64 {
    let numbers_loaded = load_uint_vec_from("crates/day9/src/day9_input.txt");

    if let Ok(numbers) = numbers_loaded {
        let wanted_sum = problem1();
        let mut i = 0;
        let mut j = 1;
        let mut s = numbers[i] + numbers[j];

        while i < numbers.len() && j < numbers.len() && s != wanted_sum {
            if s > wanted_sum {
                s -= numbers[i];
                i += 1;
            } else if s < wanted_sum {
                j += 1;
                s += numbers[j];
            }
        }

        let range: Vec<_> = numbers[i..(j + 1)].iter().collect();

        return *range.iter().min().unwrap() + *range.iter().max().unwrap();
    }

    0
}
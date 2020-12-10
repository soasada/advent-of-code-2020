use files::load_uint_vec_from;

pub fn problem1() -> i64 {
    let numbers_loaded = load_uint_vec_from("crates/day10/src/day10_input.txt");

    if let Ok(mut numbers) = numbers_loaded {
        numbers.sort();
        let mut i = 0;
        let mut j = 1;

        let mut jolt1 = 1;
        let mut jolt3 = 1;

        while i < numbers.len() && j < numbers.len() {
            let difference = numbers[j] - numbers[i];
            if difference == 1 {
                jolt1 += 1;
            } else if difference == 3 {
                jolt3 += 1;
            }
            i += 1;
            j += 1;
        }
        return jolt1 * jolt3;
    }
    0
}

pub fn problem2() -> i64 {
    let numbers_loaded = load_uint_vec_from("crates/day10/src/day10_input.txt");

    if let Ok(mut numbers) = numbers_loaded {
        numbers.sort();

        let mut count = 0;
        let mut one = 0;
        let mut two = 0;
        let mut three = 0;
        let mut i = 0;

        while i < numbers.len() - 1 {
            let valid;

            if i == 0 {
                valid = numbers[i + 1] < 3;
            } else {
                valid = numbers[i + 1] - numbers[i - 1] < 3;
            }

            if count == 3 {
                count = 0;
                three += 1;
            }

            if valid {
                count += 1;
            } else {
                if count == 2 {
                    two += 1;
                }

                if count == 1 {
                    one += 1;
                }

                count = 0;
            }
            i += 1;
        }

        if count == 3 {
            three += 1;
        }

        if count == 2 {
            two += 1;
        }

        if count == 1 {
            one += 1;
        }

        return i64::pow(7, three) * i64::pow(4, two) * i64::pow(2, one);
    }
    0
}
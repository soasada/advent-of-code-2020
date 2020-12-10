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
    0
}
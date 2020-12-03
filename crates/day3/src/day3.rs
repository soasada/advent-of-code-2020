use files::load_str_vec_from;

pub fn problem1() -> i32 {
    let forest_result = load_str_vec_from("crates/day3/src/day3_input.txt");
    let mut trees = 0;

    if let Ok(forest) = forest_result {
        let mut right = 0;
        for line in forest.iter() {
            let chars = line.chars().collect::<Vec<char>>();

            let cell = chars[right];

            if cell == '#' {
                trees = trees + 1;
            }

            right += 3;

            if right > chars.len() - 1 {
                right %= chars.len();
            }
        }
    }

    trees
}

pub fn problem2() -> i64 {
    0
}
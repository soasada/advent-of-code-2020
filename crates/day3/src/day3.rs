use files::load_str_vec_from;

pub fn problem1(right: usize, down: usize) -> i64 {
    let forest_result = load_str_vec_from("crates/day3/src/day3_input.txt");
    let mut trees = 0;

    if let Ok(forest) = forest_result {
        let mut right_cont = 0;
        for (i, line) in forest.iter().enumerate() {
            if i % down == 0 {
                let chars = line.chars().collect::<Vec<char>>();

                let cell = chars[right_cont];

                if cell == '#' {
                    trees = trees + 1;
                }

                right_cont += right;

                if right_cont > chars.len() - 1 {
                    right_cont %= chars.len();
                }
            }
        }
    }

    trees
}

pub fn problem2() -> i64 {
    problem1(1, 1) *
        problem1(3, 1) *
        problem1(5, 1) *
        problem1(7, 1) *
        problem1(1, 2)
}
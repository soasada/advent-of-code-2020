use files::load_uint_vec_from;

pub fn problem1() -> u32 {
    let numbers = load_uint_vec_from("crates/day1/src/day1_input.txt");

    let mut result = 0;
    match numbers {
        Ok(v) => for num in v.iter() {
            for num2 in v.iter() {
                if num + num2 == 2020 {
                    result = num * num2;
                    break;
                }
            }
        },
        Err(e) => println!("ERROR, {}", e)
    };
    result
}

pub fn problem2() -> u32 {
    let numbers = load_uint_vec_from("crates/day1/src/day1_input.txt");

    let mut result = 0;
    match numbers {
        Ok(v) => for num in v.iter() {
            for num2 in v.iter() {
                for num3 in v.iter() {
                    if num + num2 + num3 == 2020 {
                        result = num * num2 * num3;
                        break;
                    }
                }
            }
        },
        Err(e) => println!("ERROR, {}", e)
    };
    result
}
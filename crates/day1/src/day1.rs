use files::load_uint_vec_from;
use std::collections::HashMap;

pub fn problem1() -> i64 {
    let numbers = load_uint_vec_from("crates/day1/src/day1_input.txt");
    let mut pairs = HashMap::new();
    let year: i64 = 2020;

    match numbers {
        Ok(ref nums) =>
            for n in nums.iter().copied() {
                pairs.insert(year - n, n);
            },
        _ => panic!()
    }

    let mut result = 0;
    match numbers {
        Ok(ref v) => for num in v.iter() {
            let x = match pairs.get(num) {
                Some(n) => *n,
                _ => 0
            };

            if num + x == year {
                result = num * x;
                break;
            }
        },
        Err(e) => println!("ERROR, {}", e)
    };
    result
}

pub fn problem2() -> i64 {
    let numbers = load_uint_vec_from("crates/day1/src/day1_input.txt");
    let mut pairs = HashMap::new();
    let mut products = HashMap::new();
    let year: i64 = 2020;

    match numbers {
        Ok(ref nums) =>
            for n in nums.iter().copied() {
                for n2 in nums.iter().copied() {
                    pairs.insert(year - n - n2, n + n2);
                    products.insert(year - n - n2, n * n2);
                }
            },
        _ => panic!()
    }

    let mut result = 0;
    match numbers {
        Ok(ref v) => for num in v.iter() {
            let x = match pairs.get(num) {
                Some(n) => *n,
                _ => 0
            };

            let product = match products.get(num) {
                Some(n) => *n,
                _ => 0
            };

            if num + x == year {
                result = num * product;
                break;
            }
        },
        Err(e) => println!("ERROR, {}", e)
    };
    result
}
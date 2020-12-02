use std::fs;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::convert::Infallible;
use std::num::ParseIntError;

pub fn load_uint_vec_from(filename: &str) -> Result<Vec<i64>, ParseIntError> {
    return load_vec_from::<i64>(filename);
}

pub fn load_str_vec_from(filename: &str) -> Result<Vec<String>, Infallible> {
    return load_vec_from::<String>(filename);
}

fn load_vec_from<T: FromStr>(filename: &str) -> Result<Vec<T>, T::Err> {
    let file = fs::File::open(filename).expect("File not found");
    let br = BufReader::new(file);

    return br.lines()
        .map(|line| line.expect("No line"))
        .map(|num| num.parse::<T>())
        .collect();
}
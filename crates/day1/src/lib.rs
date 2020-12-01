use std::fs;
use std::io::{BufReader, BufRead, Error, ErrorKind};

pub fn problem1() -> u32 {
    let file = fs::File::open("crates/day1/src/day1_input.txt").expect("File not found");
    let br = BufReader::new(file);

    let numbers: Result<Vec<u32>, Error> = br.lines()
        .map(|line| line.expect("No line"))
        .map(|num| num.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect();

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
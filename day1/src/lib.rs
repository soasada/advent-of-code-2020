use std::fs;
use std::io::{BufReader, BufRead, Error, ErrorKind};

pub fn result() -> u16 {
    let file = fs::File::open("day1/src/day1_input.txt").expect("File not found");
    let br = BufReader::new(file);

    let numbers: Result<Vec<u16>, Error> = br.lines()
        .map(|line| line.expect("No line"))
        .map(|num| num.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect();

    numbers?.

    5
}
use std::fs;
use std::io::{BufReader, BufRead, Error, ErrorKind};

pub fn load_uint_vec_from(filename: &str) -> Result<Vec<i64>, Error> {
    let file = fs::File::open(filename).expect("File not found");
    let br = BufReader::new(file);

    return br.lines()
        .map(|line| line.expect("No line"))
        .map(|num| num.parse::<i64>().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect();
}
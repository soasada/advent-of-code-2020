use files::load_str_vec_from;

pub fn problem1() -> i64 {
    return (*decode_rows().iter().max().unwrap()) as i64;
}

pub fn problem2() -> i64 {
    let seat_ids = decode_rows();
    let min_seat_id = *seat_ids.iter().min().unwrap() + 1;
    let max_seat_id = *seat_ids.iter().max().unwrap();

    for i in min_seat_id..max_seat_id {
        if !seat_ids.contains(&i) {
            return i as i64;
        }
    }

    return -1;
}

fn decode_rows() -> Vec<isize> {
    let boarding_passes_loaded = load_str_vec_from("crates/day5/src/day5_input.txt");

    if let Ok(boarding_passes) = boarding_passes_loaded {
        return boarding_passes.iter()
            .map(|boarding_pass| decode_boarding_pass(boarding_pass))
            .map(|binary| to_dec(&binary))
            .collect();
    }

    return Vec::new();
}

fn decode_boarding_pass(boarding_pass: &String) -> String {
    return boarding_pass.chars()
        .map(|c| {
            if c == 'F' || c == 'L' {
                '0'
            } else {
                '1'
            }
        })
        .collect::<Vec<char>>()
        .into_iter()
        .collect();
}

fn to_dec(binary: &String) -> isize {
    return isize::from_str_radix(binary, 2).unwrap();
}
use core::num::ParseIntError;

fn main() {
    let input = std::env::args().nth(1).unwrap();

    let mut acumulator = 0;
   
    for line in input.lines() {
        match get_calibration_value(line) {
            Ok(v) => acumulator += v,
            Err(_) => {}
        }
    }
   println!("{}", acumulator);
}

fn get_calibration_value(line: &str) -> Result<i32, ParseIntError> {
    let numeric = line.chars().filter(|c| c.is_digit(10)).collect::<String>();
    let len = numeric.len();

    [&numeric[0..1], &numeric[len-1..]].concat().parse()
}

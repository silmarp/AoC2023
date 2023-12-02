use core::num::ParseIntError;

fn main() {
   let _a = get_calibration_value("1abc2");
   let _b = get_calibration_value("pqr3stu8vwx");
   let _c = get_calibration_value("a1b2c3d4e5f");
   let _d = get_calibration_value("treb7uchet");
}

fn get_calibration_value(line: &str) -> Result<i32, ParseIntError> {
    let numeric = line.chars().filter(|c| c.is_digit(10)).collect::<String>();
    let len = numeric.len();

    [&numeric[0..1], &numeric[len-1..]].concat().parse()
}

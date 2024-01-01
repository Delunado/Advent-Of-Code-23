use std::fs;

struct CalibrationLine<'a> {
    pub line: &'a str,
}

impl<'a> CalibrationLine<'a> {
    fn new(line: &str) -> CalibrationLine {
        CalibrationLine { line }
    }

    // This is, finding the first and last digit and summing them
    pub fn get_calibration_value(&self) -> u32 {
        let mut line_digits: Vec<char> = Vec::new();

        for character in self.line.chars() {
            if character.is_digit(10) {
                line_digits.push(character);
            }
        }

        if line_digits.len() >= 1 {
            let mut final_number = String::new();
            final_number.push(*line_digits.first().unwrap());
            final_number.push(*line_digits.last().unwrap());

            let final_digit_value = final_number.parse::<u32>();

            match final_digit_value {
                Ok(valid_digit) => return valid_digit,
                _ => return 0,
            }
        }

        return 0;
    }
}

pub fn resolve() {
    let path_file = "input/day1.txt";

    let file_content = fs::read_to_string(path_file).expect("There is a problem reading the file!");

    let mut calibration_lines: Vec<CalibrationLine> = Vec::new();

    for line in file_content.lines() {
        calibration_lines.push(CalibrationLine::new(line));
    }

    let mut calibration_result: u32 = 0;

    for calibration_line in calibration_lines {
        calibration_result += calibration_line.get_calibration_value();
    }

    println!("Solution is: {}", calibration_result);
}
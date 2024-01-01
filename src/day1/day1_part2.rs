use std::fs;

use crate::lib::contains_digit_as_string;

struct CalibrationLine<'a> {
    pub line: &'a str,
    digits_as_chars_buffer: Vec<char>,
}

impl<'a> CalibrationLine<'a> {
    fn new(line: &str) -> CalibrationLine {
        let mut calibration_line = CalibrationLine {
            line,
            digits_as_chars_buffer: Vec::new(),
        };

        calibration_line.process();

        return calibration_line;
    }

    fn process(&mut self) {
        // Gets every digit in the line, being char digits or string digits, and save them in the buffer
        let mut search_digits_buffer: String = String::new();

        for character in self.line.chars() {
            if character.is_digit(10) {
                self.digits_as_chars_buffer.push(character);
            } else {
                search_digits_buffer.push(character);

                if let Some(digit_as_char) = contains_digit_as_string(&search_digits_buffer) {
                    self.digits_as_chars_buffer.push(digit_as_char);
                    
                    search_digits_buffer.clear();
                    search_digits_buffer.push(character);
                }
            }
        }
    }

    // This is, finding the first and last digit and summing them
    pub fn get_calibration_value(&self) -> u32 {
        let (first_digit_char, last_digit_char) = (
            self.digits_as_chars_buffer.first(),
            self.digits_as_chars_buffer.last(),
        );

        let mut final_number_string = String::new();

        if let Some(first_digit_char) = first_digit_char {
            final_number_string.push(*first_digit_char);
        }

        if let Some(last_digit_char) = last_digit_char {
            final_number_string.push(*last_digit_char);
        }

        match final_number_string.parse::<u32>() {
            Ok(number) => number,
            _ => 0,
        }
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

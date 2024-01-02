use std::fs;

pub fn resolve() {
    let path_file = "input/day2.txt";

    let file_content = fs::read_to_string(path_file).expect("There is a problem reading the file!");

    let mut sum: u32 = 0;

    for line in file_content.lines() {
        let content: Vec<&str> = line
            .split(&[':', ' ', ';', ','][..])
            .filter(|&c| c != "")
            .collect();

        let mut min_red_count = 0;
        let mut min_green_count = 0;
        let mut min_blue_count = 0;

        for i in (2..content.len()).step_by(2) {
            let number_of_balls = content[i].parse::<u32>().expect("It should be a number!");
            let ball_type = content[i + 1];

            match ball_type {
                "red" => {
                    if number_of_balls > min_red_count {
                        min_red_count = number_of_balls;
                    }
                }
                "green" => {
                    if number_of_balls > min_green_count {
                        min_green_count = number_of_balls;
                    }
                }
                "blue" => {
                    if number_of_balls > min_blue_count {
                        min_blue_count = number_of_balls;
                    }
                }
                other => panic!("Error! {}", other),
            }
        }

        sum += min_red_count * min_blue_count * min_green_count;
    }

    println!("Sum is {}", sum);
}
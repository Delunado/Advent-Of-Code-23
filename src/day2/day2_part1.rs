use std::fs;

pub fn resolve() {
    let path_file = "input/day2.txt";

    let file_content = fs::read_to_string(path_file).expect("There is a problem reading the file!");

    let mut sum: u32 = 0;
    let mut should_sum: bool = true;

    for line in file_content.lines() {
        let content: Vec<&str> = line
            .split(&[':', ' ', ';', ','][..])
            .filter(|&c| c != "")
            .collect();

        for i in (2..content.len()).step_by(2) {
            let number_of_balls = content[i].parse::<u32>().expect("It should be a number!");
            let ball_type = content[i + 1];

            match ball_type {
                "red" => should_sum = number_of_balls <= 12,
                "green" => should_sum = number_of_balls <= 13,
                "blue" => should_sum = number_of_balls <= 14,
                other => panic!("Error! {}", other),
            }

            if !should_sum {
                break;
            };
        }

        if should_sum {
            let current_id = content[1].parse::<u32>().expect("It should be a number!");
            sum += current_id;
        }
    }

    println!("Sum is {}", sum);
}
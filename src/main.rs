#![allow(non_snake_case)]

use std::io;
use std::thread;
use std::time::Duration;




mod solutions {
    pub mod day1;
    pub mod day2;
}

fn main() {
    println!("Advent of code solver.");
    println!("Type the day you wanted solved :sunglasses:");

    let mut input = String::new();
    let result = match io::stdin().read_line(&mut input) {
        Ok(res) => {
            match input.trim().len(){
                0 => panic!("Ree"),
                1 => assign_input_to_file(&input.trim()),
                _ => panic!("Actual length: {}", res),
            }
        }
        Err(_) => panic!("Error"),
    };

    match result {
        Ok(answer) => {
            println!("Your input: {}", answer);
            thread::sleep(Duration::from_secs(3));
        },

        Err(error) => {
            println!("Couldn't parse your input. Error: {}", error);
            thread::sleep(Duration::from_secs(3));
        }
    }
}

fn assign_input_to_file(input: &str) -> Result<String, String> {
    return match input {
        "1" => Ok(solutions::day1::run()),
        "2" => Ok(solutions::day2::run()),
        _ => Err(format!("Error: {}", input)),
    }
}

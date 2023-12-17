use std::io;
use std::time::Duration;
use std::thread;

mod solutions {
    pub mod day1;
    pub mod day2;
}

fn main() {
    
    println!("Advent of code solver.");
    println!("Type the day you wanted solved :sunglasses:");
    
    let mut input = String::with_capacity(1);
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            AssignInputToFile(&input);
        }
        Err(error) => {
            println!("Couldn't parse your input. Error: {}", error);
            thread::sleep(Duration::from_secs(3));
        }
    }
}

fn AssignInputToFile(input: &str) -> String
{
    let mut result = String::new();
    match input
    {
        "1" => result = solutions::day1::run(),
        "2" => result = solutions::day2::run(),
        _ => result = String::from("errror"),
    }
    return result;
}
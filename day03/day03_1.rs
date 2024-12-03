use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut results: Vec<i32> = Vec::new();
    for (_, [num1, num2]) in re.captures_iter(&contents).map(|c| c.extract()) {
        results.push(num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap());
    }

    let score: i32 = results.iter().sum();
    println!("{score}");

}
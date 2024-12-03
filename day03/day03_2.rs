use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    // get indices of multiplications
    let mut mult_indices = Vec::new();
    for cap in re.captures_iter(&contents) {
        mult_indices.push(cap.get(0).unwrap().start());
    }
    // get multiplications
    let mut results: Vec<i32> = Vec::new();
    for (_, [num1, num2]) in re.captures_iter(&contents).map(|c| c.extract()) {
        results.push(num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap());
    }

    let re = Regex::new(r"(do\(\)|don\'t\(\))").unwrap();
    // get indices of intructions
    let mut instructions_indices = Vec::new();
    for cap in re.captures_iter(&contents) {
        instructions_indices.push(cap.get(0).unwrap().start());
    }
    let mut results_instructions = Vec::new();
    for (_, [instruction]) in re.captures_iter(&contents).map(|c| c.extract()) {
        results_instructions.push(instruction);
    }

    let mut start = 0;
    let mut status: bool = true;
    let mut good_regions = Vec::new();
    for i in 0..results_instructions.len() {
        let current = instructions_indices[i];
        if results_instructions[i] == "don't()" && status {
            good_regions.push((start, current));
            status = false;
        } else if results_instructions[i] == "do()" && !status {
            start = instructions_indices[i];
            status = true
        }
        if results_instructions[i] == "do()" && i == results_instructions.len() - 1 {
            good_regions.push((start, contents.len() - 1));
        }
    }

    let mut score: i32 = 0;

    for i in 0..results.len() {
        for j in 0..good_regions.len() {
            if mult_indices[i] >= good_regions[j].0 && mult_indices[i] <= good_regions[j].1 {
                score += results[i];
            }
        }
    }

    println!("{:?}", results_instructions);
    println!("{:?}", instructions_indices);
    println!("{:?}", good_regions);
    println!("{score}");

}
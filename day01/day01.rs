use std::env;
use std::fs;
use std::collections::HashMap; 

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut vector1: Vec<i32> = Vec::new();
    let mut vector2: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let line_vec: Vec<&str> = line.split_whitespace().collect();
        vector1.push(line_vec[0].parse::<i32>().unwrap());
        vector2.push(line_vec[1].parse::<i32>().unwrap());
    }
    vector1.sort();
    vector2.sort();

    if vector1.len() != vector2.len() {
        std::process::exit(0);
    }
    
    let mut total_diff: i32 = 0;
    let mut value_counter: HashMap<i32, i32> = HashMap::new();

    for i in 0..vector1.len() {
        let value1 = vector1[i];
        let value2 = vector2[i];
        total_diff += (value2 - value1).abs();

        if vector1.contains(&value2) {
            value_counter.entry(value2).and_modify(|counter| *counter += 1).or_insert(1);
        }
    }
    println!("Total difference: {total_diff}");

    let mut similarity_score: i32 = 0;
    for value in vector1 {
        let counter: i32 = value_counter.get(&value).copied().unwrap_or(0);
        similarity_score += value * counter;
    }

    println!("Similarity score: {similarity_score}");
}
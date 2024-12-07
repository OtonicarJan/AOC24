use std::collections::HashMap;
use std::env;
use std::fs;

fn bubble_sort<'a>(arr: &'a mut Vec<i32>, rules: &'a HashMap<i32, Vec<i32>>) -> &'a mut Vec<i32> {
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 0..arr.len() - 1 {
            if let Some(arrs) = rules.get(&arr[i]) {
                if arrs.contains(&arr[i + 1]) {
                    arr.swap(i + 1, i);
                    swapped = true;
                }
            }
        }
    }

    arr.reverse();
    arr
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut empty_line: bool = false;
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut counter_correct: i32 = 0;
    let mut counter_incorrect: i32 = 0;
    for line in contents.lines() {
        if !empty_line && !line.is_empty() {
            let line_vec: Vec<&str> = line.split("|").collect();
            rules
                .entry(line_vec[0].parse::<i32>().unwrap())
                .or_insert(Vec::new())
                .push(line_vec[1].parse::<i32>().unwrap());
        } else if line.is_empty() {
            empty_line = true;
        } else {
            let line_vec: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

            let mut sorted_vector: Vec<i32> = line_vec.clone();
            let sorted_vector: Vec<i32> = bubble_sort(&mut sorted_vector, &rules).clone();

            if line_vec == sorted_vector {
                counter_correct += line_vec[line_vec.len() / 2];
            } else {
                counter_incorrect += sorted_vector[sorted_vector.len() / 2];
            }
        }
    }
    println!("Part 1: {counter_correct}");
    println!("Part 2: {counter_incorrect}");
}

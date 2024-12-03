use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut counter: i32 = 0;

    for line in contents.lines() {
        let line_vec: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        let mut state: &str = "start";
        for i in 1..line_vec.len() {
            let diff: i32 = line_vec[i] - line_vec[i-1];

            if diff.abs() > 3 {
                // println!("Breaking");
                break
            }

            if diff == 0 {
                break
            } else if diff < 0 {
                if state == "increasing" {
                    break
                }
                state = "decreasing";
            } else if diff > 0 {
                if state == "decreasing" {
                    break
                }
                state = "increasing";
            }

            if i == line_vec.len() - 1 {
                counter += 1;
            }
        }
    }
    println!("{counter}");
    
}
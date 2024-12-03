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


        let mut possible: i32 = 0;
        for j in 0..line_vec.len() {
            if possible > 0 {
                break
            }
            let mut line_clone: Vec<i32> = line_vec.clone();
            line_clone.remove(j as usize);

            let mut state: &str = "start";
            for i in 1..line_clone.len() {

                let diff: i32 = line_clone[i] - line_clone[i-1];

                if diff.abs() > 3 {
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

                if i == line_clone.len() - 1 {
                    possible += 1;
                }
            }
        }

        if possible > 0 {
            counter += 1;
        }  
    }
    println!("{counter}");
    
}
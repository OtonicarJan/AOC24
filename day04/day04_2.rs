use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut mtx: Vec<Vec<char>> = Vec::new();

    for line in contents.split("\n") {
        mtx.push(line.chars().collect());
    }

    let mut counter: i32 = 0;
    for i in 0..mtx.len() - 2 {
        for j in 0..mtx[0].len() - 2 {
            let down: String = (0..3)
                .map(|k| (i + k, j + k))
                .filter_map(|(c, d)| mtx.get(c as usize).and_then(|row| row.get(d as usize)))
                .collect();

            let up: String = (0..3)
                .map(|k| (i + 2 - k, j + k))
                .filter_map(|(c, d)| mtx.get(c as usize).and_then(|row| row.get(d as usize)))
                .collect();

            if (down == "SAM" || down == "MAS") && (up == "SAM" || up == "MAS") {
                counter += 1;
            }
        }
    }
    println!("{counter}");
}
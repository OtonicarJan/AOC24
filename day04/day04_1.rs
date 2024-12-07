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
    for i in 0..mtx.len() {
        for j in 0..mtx[0].len() {
            if mtx[i][j] == 'X' || mtx[i][j] == 'S' {
                // horizontal
                if let Some(slice) = mtx[i].get(j..j + 4) {
                    let result: String = slice.iter().collect();
                    if result == "XMAS" || result == "SAMX" {
                        counter += 1;
                    }
                }
                // vertical
                if let Some(_rows) = mtx.get(i..i + 4) {
                    let result: String = (i..i + 4).map(|k| mtx[k as usize][j as usize]).collect();
                    if result == "XMAS" || result == "SAMX" {
                        counter += 1;
                    }
                }
                // diagonal down
                if let Some(_rows) = mtx.get(i..i + 4) {
                    if let Some(_rows) = mtx.get(j..j + 4) {
                        let result: String = (0..4)
                            .map(|k| (i + k, j + k))
                            .filter_map(|(c, d)| {
                                mtx.get(c as usize).and_then(|row| row.get(d as usize))
                            })
                            .collect();
                        if result == "XMAS" || result == "SAMX" {
                            counter += 1;
                        }
                    }
                }
                // diagonal up
                if i >= 3 {
                    if let Some(_rows) = mtx.get(i - 3..=i) {
                        if let Some(_rows) = mtx.get(j..j + 4) {
                            let result: String = (0..4)
                                .map(|k| (i - k, j + k))
                                .filter_map(|(c, d)| {
                                    mtx.get(c as usize).and_then(|row| row.get(d as usize))
                                })
                                .collect();
                            if result == "XMAS" || result == "SAMX" {
                                counter += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{counter}");
}

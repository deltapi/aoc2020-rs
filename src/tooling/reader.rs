use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub fn read_file_to_vec(path: &str) -> Vec<usize> {
    let mut entries = vec!();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ip) = line {
                entries.push(ip.parse::<usize>().unwrap());
            }
        }
    }
    entries
}

pub fn read_file_to_vec_of_string(path: &str) -> Vec<String> {
    let mut entries = vec!();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ip) = line {
                entries.push(ip);
            }
        }
    }
    entries
}

pub fn read_block_file_to_vec_of_string(path: &str) -> Vec<String> {
    let mut entries = vec!();
    if let Ok(lines) = read_lines(path) {
        let mut entry = "".to_string();
        for line in lines {
            if let Ok(ip) = line {
                match ip.as_str() {
                    "" => {
                        entries.push(entry);
                        entry = "".to_string();
                    },
                    _ => {
                        entry.push_str(" ");
                        entry.push_str(&ip);
                    },
                }
            }
        }
        entries.push(entry);
    }
    entries
}

pub fn read_block_file_to_vec_vec_of_string(path: &str) -> Vec<Vec<String>> {
    let mut entries: Vec<Vec<String>> = vec!();
    if let Ok(lines) = read_lines(path) {
        let mut entry = vec![];
        for line in lines {
            if let Ok(ip) = line {
                match ip.as_str() {
                    "" => {
                        entries.push(entry);
                        entry = vec![];
                    },
                    _ => {
                        entry.push(ip);
                    },
                }
            }
        }
        entries.push(entry);
    }
    entries
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
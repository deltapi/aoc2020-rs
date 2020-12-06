use crate::tooling::vector::split_each_at;

fn _day_4_1(input: Vec<String>) -> usize {
    let mut valid: usize = 0;
    for line in input {
        let re = regex::Regex::new(r" |\\n").unwrap();
        let split = re.split(&line).collect();
        let splitt: Vec<Vec<String>> = split_each_at(split, ":");
        if splitt.iter().any(|i| i[0] == "byr")
            && splitt.iter().any(|i| i[0].trim() == "iyr")
            && splitt.iter().any(|i| i[0].trim() == "eyr")
            && splitt.iter().any(|i| i[0].trim() == "hgt")
            && splitt.iter().any(|i| i[0].trim() == "hcl")
            && splitt.iter().any(|i| i[0].trim() == "ecl")
            && splitt.iter().any(|i| i[0].trim() == "pid") {
            valid += 1;
        }
    }
    valid
}


fn _day_4_2(input: Vec<String>) -> usize {
    let mut valid: usize = 0;
    for line in input {
        let re = regex::Regex::new(r" |\\n").unwrap();
        let split = re.split(&line).collect();
        let splitt: Vec<Vec<String>> = split_each_at(split, ":");
        //println!("{:?}", splitt);
        if splitt.iter().find(|i| i[0] == "byr")
            .map_or(false, |x|
                x[1].parse::<usize>().unwrap() <= 2002usize
                    && x[1].parse::<usize>().unwrap() >= 1920usize)
            && splitt.iter().find(|i| i[0] == "iyr")
            .map_or(false, |x|
                x[1].parse::<usize>().unwrap() <= 2020usize
                    && x[1].parse::<usize>().unwrap() >= 2010usize)
            && splitt.iter().find(|i| i[0] == "eyr")
            .map_or(false, |x|
                x[1].parse::<usize>().unwrap() <= 2030usize
                    && x[1].parse::<usize>().unwrap() >= 2020usize)
            && splitt.iter().find(|i| i[0] == "hgt")
            .map_or(false, |x| {
                let reg = regex::Regex::new(r"^(\d{2,3})(in|cm)$").unwrap();
                return reg.is_match(&x[1])
                    && match reg.captures(&x[1]).unwrap().get(2).unwrap().as_str() {
                    "in" => {
                        return reg.captures(&x[1]).map_or("0", |m| m.get(1).unwrap().as_str()).parse::<usize>().unwrap() <= 76
                            && reg.captures(&x[1]).map_or("0", |m| m.get(1).unwrap().as_str()).parse::<usize>().unwrap() >= 59;
                    }
                    "cm" => {
                        return reg.captures(&x[1]).map_or("0", |m| m.get(1).unwrap().as_str()).parse::<usize>().unwrap() <= 193
                            && reg.captures(&x[1]).map_or("0", |m| m.get(1).unwrap().as_str()).parse::<usize>().unwrap() >= 150;
                    }
                    _ => {
                        return false;
                    }
                };
            },
            )
            && splitt.iter().find(|i| i[0] == "hcl")
            .map_or(false, |x| {
                let reg = regex::Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                return reg.is_match(&x[1]);
            })
            && splitt.iter().find(|i| i[0] == "ecl")
            .map_or(false, |x| {
                match x[1].as_str() {
                    "amb" | "blu" | "brn"
                    | "gry" | "grn" | "hzl"
                    | "oth" => return true,
                    _ => return false,
                }
            })
            && splitt.iter().find(|i| i[0] == "pid")
            .map_or(false, |x| {
                let reg = regex::Regex::new(r"^\d{9}$").unwrap();
                return reg.is_match(&x[1]);
            })
        {
            valid += 1;
        }
    }
    valid
}

#[cfg(test)]
mod tests {
    use crate::tooling::reader::*;
    use crate::day_4::{_day_4_1, _day_4_2};

    #[test]
    fn solve_day_4_1() {
        let entries = read_block_file_to_vec_of_string("resources/day_4");
        println!("Result: {}", _day_4_1(entries));
    }

    #[test]
    fn solve_day_4_2() {
        let entries = read_block_file_to_vec_of_string("resources/day_4");
        println!("Result: {}", _day_4_2(entries));
    }

    #[test]
    fn regex() {
        let reg = regex::Regex::new(r"^\d{2,3}(in|cm)$").unwrap();
        assert!(reg.is_match("222cm"));
    }
}

fn _day_2_1(input: Vec<String>) -> usize {
    let mut valid: usize = 0;

    for case in input {
        let split: Vec<&str> = case.split(" ").collect::<Vec<&str>>();
        let index_low = split[0].split("-").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
        let index_high = split[0].split("-").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
        let mut letter = split[1].to_string();
        letter.pop();
        let pwd = split[2];

        let count = pwd.chars()
            .filter(|a| letter == a.to_string())
            .count();

        // TODO make the conditional a function pointer
        if count <= index_high && count >= index_low {
            valid += 1;
        }
    }
    valid
}

fn _day_2_2(input: Vec<String>) -> usize {
    let mut valid: usize = 0;

    for case in input {
        let split: Vec<&str> = case.split(" ").collect::<Vec<&str>>();
        let index_low = split[0].split("-").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
        let index_high = split[0].split("-").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
        let mut letter = split[1].to_string();
        letter.pop();
        let pwd = split[2];

        if (pwd.get(index_low - 1..index_low).unwrap() == letter)
            != (pwd.get(index_high - 1..index_high).unwrap() == letter) {
            valid += 1;
        }
    }
    valid
}

#[cfg(test)]
mod tests {
    use crate::tooling::reader::*;
    use crate::day_2::{_day_2_1, _day_2_2};

    #[test]
    fn solve_day_2_1() {
        let entries = read_file_to_vec_of_string("resources/day_2");
        let result = _day_2_1(entries);
        println!("Result: {}", result);
        assert_eq!(result, 418)
    }

    #[test]
    fn solve_day_2_2() {
        let entries = read_file_to_vec_of_string("resources/day_2");
        let result = _day_2_2(entries);
        println!("Result: {}", result);
        assert_eq!(result, 616)
    }
}

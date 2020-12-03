use itertools::iproduct;

fn day_1_1(input: Vec<usize>) -> usize {
    iproduct!(&input, &input)
        .find(|(a, b)| **a + **b == 2020)
        .map(|(a, b)| a * b)
        .unwrap()
}

fn day_1_2(input: Vec<usize>) -> usize {
    iproduct!(&input, &input, &input)
        .find(|(a, b, c)| **a + **b + **c == 2020)
        .map(|(a, b, c)| a * b * c)
        .unwrap()
}

fn day_2_1(input: Vec<String>) -> usize {
    let mut valid: usize = 0;

    for case in input {

    let split: Vec<&str> = case.split(" ").collect::<Vec<&str>>();
    let range_low = split[0].split("-").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
    let range_high = split[0].split("-").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
    let mut letter = split[1].to_string();
    letter.pop();
    let pwd = split[2];

    let count = pwd.chars()
        .filter(|a| letter == a.to_string() )
        .count();

    if count <= range_high && count >= range_low {
        valid += 1;
    }
    }

    valid
}

fn day_2_2(input: Vec<String>) -> usize {
    let mut valid: usize = 0;

    for case in input {

        let split: Vec<&str> = case.split(" ").collect::<Vec<&str>>();
        let index_low = split[0].split("-").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
        let index_high = split[0].split("-").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
        let mut letter = split[1].to_string();
        letter.pop();
        let pwd = split[2];

        if (pwd.get(index_low-1..index_low).unwrap() == letter)
            != (pwd.get(index_high-1..index_high).unwrap() == letter) {
            valid += 1;
        }

    }

    valid
}

#[cfg(test)]
mod tests {
    use crate::tooling::{read_file_to_vec, read_file_to_vec_of_string};
    use crate::day_1::{day_1_1, day_1_2, day_2_1, day_2_2};

    #[test]
    fn solve_day_1_1() {
        let entries = read_file_to_vec("resources/day_1");
        let result = day_1_1(entries);
        println!("Result: {}", result);
    }

    #[test]
    fn solve_day_1_2() {
        let entries = read_file_to_vec("resources/day_1");
        let result = day_1_2(entries);
        println!("Result: {}", result);
    }

    #[test]
    fn solve_day_2_1() {
        let entries = read_file_to_vec_of_string("resources/day_2");

        println!("Result: {}", day_2_1(entries));
    }

    #[test]
    fn solve_day_2_2() {
        let entries = read_file_to_vec_of_string("resources/day_2");


        println!("Result: {}", day_2_2(entries));
    }

}

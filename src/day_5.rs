fn _day_5_1(input: Vec<String>) -> usize {
    _to_usize(&input)
        .into_iter()
        .max()
        .unwrap()
}

fn _day_5_2(input: Vec<String>) -> usize {
    let offset = _to_usize(&input)
        .into_iter()
        .min()
        .unwrap();

    let mut sorted = _to_usize(&input);
    sorted.sort();

    let first_jump = sorted
        .into_iter()
        .enumerate()
        .find(|(i, x)| *x != i + offset)
        .unwrap().1;

    first_jump - 1
}

fn _to_usize(input: &Vec<String>) -> Vec<usize> {
    input.iter()
        .map(|x| x.replace("F", "0"))
        .map(|x| x.replace("B", "1"))
        .map(|x| x.replace("L", "0"))
        .map(|x| x.replace("R", "1"))
        .map(|x| u32::from_str_radix(&x, 2).unwrap() as usize)
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use crate::tooling::reader::*;
    use crate::day_5::{_day_5_1, _day_5_2};

    #[test]
    fn solve_day_5_1() {
        let entries = read_file_to_vec_of_string("resources/day_5");
        println!("{}", _day_5_1(entries));
    }

    #[test]
    fn solve_day_5_2() {
        let entries = read_file_to_vec_of_string("resources/day_5");
        println!("{}", _day_5_2(entries));
    }
}

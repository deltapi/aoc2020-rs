use itertools::iproduct;

fn day_5_1(input: Vec<String>) -> usize {
    to_usize(&input)
        .into_iter()
        .max()
        .unwrap()
}

fn day_5_2(input: Vec<String>) -> usize {
    let offset = to_usize(&input)
        .into_iter()
        .min()
        .unwrap();

    let mut sorted = to_usize(&input);
    sorted.sort();

    let first_jump = sorted
        .into_iter()
        .enumerate()
        .find(|(i, x)| *x != i + 80)
        .unwrap().1;

    first_jump - 1
}

fn to_usize(input: &Vec<String>) -> Vec<usize> {
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
    use crate::tooling::*;
    use crate::day_5::{day_5_1, day_5_2};

    #[test]
    fn solve_day_5_1() {
        let entries = read_file_to_vec_of_string("resources/day_5");
        println!("{}", day_5_1(entries));
    }

    #[test]
    fn solve_day_5_2() {
        let entries = read_file_to_vec_of_string("resources/day_5");
        println!("{}", day_5_2(entries));
    }
}

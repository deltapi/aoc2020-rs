use itertools::iproduct;

fn day_7_1(input: Vec<String>) -> usize {
    0
}

fn day_7_2(input: Vec<Vec<String>>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::tooling::*;
    use crate::day_7::{day_7_1, day_7_2};

    #[test]
    fn solve_day_7_1() {
        let entries = read_block_file_to_vec_of_string("resources/day_7");
        println!("{}", day_7_1(entries));
    }

    #[test]
    fn solve_day_7_2() {
        let entries = read_block_file_to_vec_vec_of_string("resources/day_7");
        println!("{}", day_7_2(entries));
    }
}

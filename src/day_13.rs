use itertools::iproduct;

fn _day_13_1(input: Vec<usize>) -> usize {
    0
}

fn _day_13_2(input: Vec<usize>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::day_13::{_day_13_1, _day_13_2};
    use crate::tooling::reader::*;

    #[test]
    fn solve_day_13_1() {
        let entries = read_file_to_vec("resources/day_13");
        let result = _day_13_1(entries);
        println!("Result: {}", result);
        assert_eq!(result, 539851);
    }

    #[test]
    fn solve_day_13_2() {
        let entries = read_file_to_vec("resources/day_13");
        let result = _day_13_2(entries);
        println!("Result: {}", result);
        assert_eq!(result, 212481360);
    }
}

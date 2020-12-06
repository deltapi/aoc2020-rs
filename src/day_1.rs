use itertools::iproduct;

fn _day_1_1(input: Vec<usize>) -> usize {
    iproduct!(&input, &input)
        .find(|(a, b)| **a + **b == 2020)
        .map(|(a, b)| a * b)
        .unwrap_or(0)
}

fn _day_1_2(input: Vec<usize>) -> usize {
    iproduct!(&input, &input, &input)
        .find(|(a, b, c)| **a + **b + **c == 2020)
        .map(|(a, b, c)| a * b * c)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::tooling::reader::*;
    use crate::day_1::{_day_1_1, _day_1_2};

    #[test]
    fn solve_day_1_1() {
        let entries = read_file_to_vec("resources/day_1");
        let result = _day_1_1(entries);
        println!("Result: {}", result);
        assert_eq!(result, 539851);
    }

    #[test]
    fn solve_day_1_2() {
        let entries = read_file_to_vec("resources/day_1");
        let result = _day_1_2(entries);
        println!("Result: {}", result);
        assert_eq!(result, 212481360);
    }
}

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

#[cfg(test)]
mod tests {
    use crate::tooling::read_file_to_vec;
    use crate::day_1::{day_1_1, day_1_2};

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
}

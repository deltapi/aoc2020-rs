fn _day_7_1(_input: Vec<String>) -> usize {
    0
}

fn _day_7_2(_input: Vec<Vec<String>>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::tooling::reader::*;
    use crate::day_7::{_day_7_1, _day_7_2};

    #[test]
    fn solve_day_7_1() {
        let entries = read_block_file_to_vec_of_string("resources/day_7");
        println!("{}", _day_7_1(entries));
    }

    #[test]
    fn solve_day_7_2() {
        let entries = read_block_file_to_vec_vec_of_string("resources/day_7");
        println!("{}", _day_7_2(entries));
    }
}

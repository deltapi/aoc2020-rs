use itertools::iproduct;

fn day_3_1(input: Vec<String>) -> usize {
    for_slope(input, (3, 1))
}

fn day_3_2(input: Vec<String>) -> usize {
    let slopes = vec!((1, 1), (3, 1), (5, 1), (7, 1), (1, 2));

    slopes.iter().map(|(x, y)| for_slope(input.clone(), (*x, *y))).product()
}


fn for_slope(input: Vec<String>, slope: (usize, usize)) -> usize {
    let length = input.len();
    let width = input[0].len();
    let mut pos: (usize, usize) = (0, 0);

    let mut encounters: usize = 0;

    while pos.1 < length - slope.1 {
        pos.0 += &slope.0;
        pos.1 += &slope.1;
        let corrected_to_right = &pos.0 % width;
        if (input[pos.1].get(corrected_to_right..corrected_to_right + 1) == Some("#")) {
            encounters += 1;
        }
    }

    encounters
}

#[cfg(test)]
mod tests {
    use crate::tooling::*;
    use crate::day_3::{day_3_1, day_3_2};

    #[test]
    fn solve_day_3_1() {
        let entries = read_file_to_vec_of_string("resources/day_3");


        println!("Result: {}", day_3_1(entries));
    }

    #[test]
    fn solve_day_3_2() {
        let entries = read_file_to_vec_of_string("resources/day_3");


        println!("Result: {}", day_3_2(entries));
    }
}

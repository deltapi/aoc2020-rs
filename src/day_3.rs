fn _day_3_1(map: Vec<String>) -> usize {
    _encounters_for_slope(&map, (3, 1))
}

fn _day_3_2(map: Vec<String>) -> usize {
    vec!((1, 1), (3, 1), (5, 1), (7, 1), (1, 2))
        .iter()
        .map(|(x, y)| _encounters_for_slope(&map, (*x, *y)))
        .product()
}

fn _encounters_for_slope(map: &[String], slope: (usize, usize)) -> usize {
    let length = map.len();
    let width = map[0].len();

    let mut pos: (usize, usize) = (0, 0);
    let mut encounters: usize = 0;

    while pos.1 < &length - &slope.1 {
        pos.0 += &slope.0;
        pos.1 += &slope.1;
        let corrected_to_right = &pos.0 % width;
        if map[pos.1.clone()].get(corrected_to_right..corrected_to_right + 1) == Some("#") {
            encounters += 1;
        }
    }
    encounters
}

#[cfg(test)]
mod tests {
    use crate::tooling::reader::*;
    use crate::day_3::{_day_3_1, _day_3_2};

    #[test]
    fn solve_day_3_1() {
        let entries = read_file_to_vec_of_string("resources/day_3");
        let result = _day_3_1(entries);
        println!("Result: {}", result);
        assert_eq!(result, 294);
    }

    #[test]
    fn solve_day_3_2() {
        let entries = read_file_to_vec_of_string("resources/day_3");
        let result = _day_3_2(entries);
        println!("Result: {}", result);
        assert_eq!(result, 5774564250);
    }
}

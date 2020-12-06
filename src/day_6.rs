fn _day_6_1(input: Vec<String>) -> usize {
    input.iter()
        .map(|x| x.replace(" ", ""))
        .map(|x| x.chars().collect::<Vec<char>>())
        .map(|mut x| {
            x.sort();
            x.dedup();
            x.len()
        })
        .sum()
}

fn _day_6_2(input: Vec<Vec<String>>) -> usize {
    input.iter()
        .map(|x| {
            let size = x.len();
            let occurrences = (b'a'..= b'z')
                .map(char::from)
                .map(|c| x.iter()
                    .filter(|y| y.contains(c))
                    .count()
                )
                .filter(|c| *c == size)
                .count();
            occurrences
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::tooling::reader::*;
    use crate::day_6::{_day_6_1, _day_6_2};

    #[test]
    fn solve_day_6_1() {
        let entries = read_block_file_to_vec_of_string("resources/day_6");
        println!("{}", _day_6_1(entries));
    }

    #[test]
    fn solve_day_6_2() {
        let entries = read_block_file_to_vec_vec_of_string("resources/day_6");
        println!("{}", _day_6_2(entries));
    }
}

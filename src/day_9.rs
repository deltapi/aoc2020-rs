use itertools::iproduct;

fn _day_9_1(input: Vec<u64>) -> u64 {
    let windows = input.windows(26).collect::<Vec<&[u64]>>();
    for (i, window) in windows.iter().enumerate() {
        let to_check = window[window.len() - 1];
        let sums = iproduct!(window.to_vec(), window.to_vec())
            .map(|(x, y)| x + y)
            .collect::<Vec<u64>>();
        if !sums.contains(&to_check) {
            return input[i + 25];
        }
    }
    0
}

fn _day_9_2(input: Vec<u64>) -> u64 {
    let number = _day_9_1(input.clone());
    for i in 0..=input.len() {
        let mut sum = 0;
        let mut j = i.clone();
        while sum < number {
            sum += input[j];
            j += 1;
        }
        if sum == number {
            let x = input.iter().skip(i).take(j - i).min().unwrap();
            let y = input.iter().skip(i).take(j - i).max().unwrap();
            println!("{} {}: {} {}: {}, {}", i, j, x, y, number, sum);
            return x + y;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::day_9::{_day_9_1, _day_9_2};
    use crate::tooling::reader::*;

    #[test]
    fn solve_day_9_1() {
        let entries = read_file_to_vec_u64("resources/day_9");
        let result = _day_9_1(entries);
        println!("Result: {}", result);
        assert_eq!(result, 15353384);
    }

    #[test]
    fn solve_day_9_2() {
        let entries = read_file_to_vec_u64("resources/day_9");
        let result = _day_9_2(entries);
        println!("Result: {}", result);
        assert_eq!(result, 2466556);
    }
}

use itertools::iproduct;
use rayon::prelude::*;
use std::collections::HashMap;
use std::iter::FromIterator;

fn _day_10_1(input: Vec<usize>) -> usize {
    let mut inp: Vec<usize> = vec![0];
    inp.extend(input.iter().copied());
    inp.push(inp.iter().max().unwrap() + 3);
    let low = inp.iter().min().unwrap();
    inp.sort();
    let diff_1 = inp
        .windows(2)
        .map(|a: &[usize]| a[1] - a[0])
        .filter(|a| *a == 1)
        .count();
    let diff_3 = inp
        .windows(2)
        .map(|a: &[usize]| a[1] - a[0])
        .filter(|a| *a == 3)
        .count();

    diff_1 * diff_3
}

fn _day_10_2(input: Vec<usize>) -> usize {
    let mut input2 = input.clone();
    input2.sort();
    let maximal = input.iter().max().unwrap() + 3;
    input2.push(maximal);
    construct(&input2)
}

fn construct(input: &Vec<usize>) -> usize {
    let mut input2 = vec![1]; // one possibility
    if input.contains(&1) {
        input2.push(1);
    } else {
        input2.push(0);
    }
    if input.contains(&2) {
        input2.push(1 + input2[1]);
    } else {
        input2.push(0);
    }

    for i in 3..=input[input.len() - 1] {
        if input.contains(&i) {
            input2.push(input2[i - 3] + input2[i - 2] + input2[i - 1]);
        } else {
            input2.push(0);
        }
    }

    input2[input2.len() - 1]
}

#[cfg(test)]
mod tests {
    use crate::day_10::{_day_10_1, _day_10_2};
    use crate::tooling::reader::*;

    #[test]
    fn solve_day_10_1() {
        let entries = read_file_to_vec("resources/day_10");
        let result = _day_10_1(entries);
        println!("Result: {}", result);
        assert_eq!(result, 2176);
    }

    #[test]
    fn solve_day_10_2() {
        let entries = read_file_to_vec("resources/day_10");
        let result = _day_10_2(entries);
        println!("Result: {}", result);
        assert_eq!(result, 18512297918464);
    }
}

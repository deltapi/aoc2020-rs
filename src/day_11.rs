use crate::day_11::State::{Empty, Floor, Occupied};
use itertools::iproduct;

#[derive(Debug, PartialEq, Clone)]
enum State {
    Floor,
    Empty,
    Occupied,
}

fn _day_11_1(input: Vec<String>) -> usize {
    let state = input
        .iter()
        .map(|a| {
            a.chars()
                .map(|c| match c {
                    '.' => Floor,
                    'L' => Empty,
                    '#' => Occupied,
                    _ => Floor,
                })
                .collect::<Vec<State>>()
        })
        .collect::<Vec<Vec<State>>>();

    let mut rounds: usize = 0;
    let mut result = progress(&state);
    rounds += 1;
    let mut new_result = vec![];
    loop {
        new_result = result.clone();
        result = progress(&result);
        rounds += 1;
        if result == new_result {
            break;
        }
    }
    result
        .iter()
        .map(|row| row.iter().filter(|state| **state == Occupied).count())
        .sum()
}

fn _day_11_2(input: Vec<String>) -> usize {
    let state = input
        .iter()
        .map(|a| {
            a.chars()
                .map(|c| match c {
                    '.' => Floor,
                    'L' => Empty,
                    '#' => Occupied,
                    _ => Floor,
                })
                .collect::<Vec<State>>()
        })
        .collect::<Vec<Vec<State>>>();

    let mut rounds: usize = 0;
    let mut result = progress2(&state);
    rounds += 1;
    let mut new_result = vec![];
    loop {
        new_result = result.clone();
        result = progress2(&result);
        rounds += 1;
        if result == new_result {
            break;
        }
    }
    result
        .iter()
        .map(|row| row.iter().filter(|state| **state == Occupied).count())
        .sum()
}

fn progress(input: &Vec<Vec<State>>) -> Vec<Vec<State>> {
    let mut result = vec![];
    for (i, row) in input.iter().enumerate() {
        let mut result_row = vec![];
        for (j, entry) in row.iter().enumerate() {
            let x = match entry {
                Floor => Floor,
                Empty => {
                    let empty = vec![];
                    let surrounding = get_immediate_surrounding(input, i, j, &empty);
                    if surrounding
                        .iter()
                        .filter(|state| ***state == Occupied)
                        .count()
                        == 0
                    {
                        Occupied
                    } else {
                        Empty
                    }
                }
                Occupied => {
                    let empty = vec![];
                    let surrounding = get_immediate_surrounding(input, i, j, &empty);
                    if surrounding
                        .iter()
                        .filter(|state| ***state == Occupied)
                        .count()
                        >= 4
                    {
                        Empty
                    } else {
                        Occupied
                    }
                }
            };
            result_row.push(x);
        }
        result.push(result_row)
    }
    result
}

fn progress2(input: &Vec<Vec<State>>) -> Vec<Vec<State>> {
    let mut result = vec![];
    for (i, row) in input.iter().enumerate() {
        let mut result_row = vec![];
        for (j, entry) in row.iter().enumerate() {
            let x = match entry {
                Floor => Floor,
                Empty => {
                    let empty = vec![];
                    let surrounding = get_surrounding_in_sight(input, i, j, &empty);
                    if surrounding
                        .iter()
                        .filter(|state| ***state == Occupied)
                        .count()
                        == 0
                    {
                        Occupied
                    } else {
                        Empty
                    }
                }
                Occupied => {
                    let empty = vec![];
                    let surrounding = get_surrounding_in_sight(input, i, j, &empty);
                    if surrounding
                        .iter()
                        .filter(|state| ***state == Occupied)
                        .count()
                        >= 5
                    {
                        Empty
                    } else {
                        Occupied
                    }
                }
            };
            result_row.push(x);
        }
        result.push(result_row)
    }
    result
}

fn get_immediate_surrounding<'a>(
    input: &'a Vec<Vec<State>>,
    i: usize,
    j: usize,
    empty: &'a Vec<State>,
) -> Vec<&'a State> {
    let maximum_i = input.len();
    let maximum_j = input[0].len();
    let mut result = vec![];
    if i > 0 {
        if j > 0 {
            result.push(
                input
                    .get(i - 1)
                    .unwrap_or(&empty)
                    .get(j - 1)
                    .unwrap_or(&Floor),
            );
        } else {
            result.push(&Floor)
        }
        result.push(input.get(i - 1).unwrap_or(&empty).get(j).unwrap_or(&Floor));
        if j < maximum_j {
            result.push(
                input
                    .get(i - 1)
                    .unwrap_or(&empty)
                    .get(j + 1)
                    .unwrap_or(&Floor),
            );
        } else {
            result.push(&Floor)
        }
    } else {
        result.push(&Floor);
        result.push(&Floor);
        result.push(&Floor);
    }
    if j > 0 {
        result.push(input.get(i).unwrap_or(&empty).get(j - 1).unwrap_or(&Floor));
    } else {
        result.push(&Floor);
    }
    if j < maximum_j {
        result.push(input.get(i).unwrap_or(&empty).get(j + 1).unwrap_or(&Floor));
    } else {
        result.push(&Floor);
    }
    if i < maximum_i {
        if j > 0 {
            result.push(
                input
                    .get(i + 1)
                    .unwrap_or(&empty)
                    .get(j - 1)
                    .unwrap_or(&Floor),
            );
        } else {
            result.push(&Floor);
        }
        result.push(input.get(i + 1).unwrap_or(&empty).get(j).unwrap_or(&Floor));
        if j < maximum_j {
            result.push(
                input
                    .get(i + 1)
                    .unwrap_or(&empty)
                    .get(j + 1)
                    .unwrap_or(&Floor),
            );
        } else {
            result.push(&Floor);
        }
    } else {
        result.push(&Floor);
        result.push(&Floor);
        result.push(&Floor);
    }
    result
}

fn get_surrounding_in_sight<'a>(
    input: &'a Vec<Vec<State>>,
    i: usize,
    j: usize,
    empty: &'a Vec<State>,
) -> Vec<&'a State> {
    let maximum_i = input.len();
    let maximum_j = input[0].len();
    let mut result = vec![];
    if i > 0 {
        if j > 0 {
            result.push(get_nw(input, i, j, maximum_i, maximum_j));
        } else {
            result.push(&Floor)
        }
        result.push(get_n(input, i, j, maximum_i, maximum_j));
        if j < maximum_j {
            result.push(get_ne(input, i, j, maximum_i, maximum_j));
        } else {
            result.push(&Floor)
        }
    } else {
        result.push(&Floor);
        result.push(&Floor);
        result.push(&Floor);
    }
    if j > 0 {
        result.push(get_w(input, i, j, maximum_i, maximum_j));
    } else {
        result.push(&Floor);
    }
    if j < maximum_j {
        result.push(get_e(input, i, j, maximum_i, maximum_j));
    } else {
        result.push(&Floor);
    }
    if i < maximum_i {
        if j > 0 {
            result.push(get_sw(input, i, j, maximum_i, maximum_j));
        } else {
            result.push(&Floor);
        }
        result.push(get_s(input, i, j, maximum_i, maximum_j));
        if j < maximum_j {
            result.push(get_se(input, i, j, maximum_i, maximum_j));
        } else {
            result.push(&Floor);
        }
    } else {
        result.push(&Floor);
        result.push(&Floor);
        result.push(&Floor);
    }
    result
}

fn get_nw<'a>(
    input: &'a Vec<Vec<State>>,
    i: usize,
    j: usize,
    maximum_i: usize,
    maximum_j: usize,
) -> &'a State {
    let state = Floor;
    let mut local_i_index = i - 1;
    let mut local_j_index = j - 1;

    while local_i_index > 0 && local_j_index > 0 && input[local_i_index][local_j_index] == Floor {
        local_i_index -= 1;
        local_j_index -= 1;
    }
    if local_i_index >= 0 && local_j_index >= 0 {
        &input[local_i_index][local_j_index]
    } else {
        &Floor
    }
}

fn get_n<'a>(
    input: &'a Vec<Vec<State>>,
    i: usize,
    j: usize,
    maximum_i: usize,
    maximum_j: usize,
) -> &'a State {
    let state = Floor;
    let mut local_i_index = i - 1;
    let mut local_j_index = j;

    while local_i_index > 0 && input[local_i_index][local_j_index] == Floor {
        local_i_index -= 1;
    }
    if local_i_index >= 0 {
        &input[local_i_index][local_j_index]
    } else {
        &Floor
    }
}

fn get_ne<'a>(
    input: &'a Vec<Vec<State>>,
    i: usize,
    j: usize,
    maximum_i: usize,
    maximum_j: usize,
) -> &'a State {
    let state = Floor;
    let mut local_i_index = i - 1;
    let mut local_j_index = j + 1;

    while local_i_index > 0
        && local_j_index < maximum_j - 1
        && input[local_i_index][local_j_index] == Floor
    {
        local_i_index -= 1;
        local_j_index += 1;
    }
    if local_i_index >= 0 && local_j_index < maximum_j {
        &input[local_i_index][local_j_index]
    } else {
        &Floor
    }
}

fn get_w<'a>(
    input: &'a Vec<Vec<State>>,
    i: usize,
    j: usize,
    maximum_i: usize,
    maximum_j: usize,
) -> &'a State {
    let state = Floor;
    let mut local_i_index = i;
    let mut local_j_index = j - 1;

    while local_j_index > 0 && input[local_i_index][local_j_index] == Floor {
        local_j_index -= 1;
    }
    if local_j_index >= 0 {
        &input[local_i_index][local_j_index]
    } else {
        &Floor
    }
}

fn get_e<'a>(
    input: &'a Vec<Vec<State>>,
    i: usize,
    j: usize,
    maximum_i: usize,
    maximum_j: usize,
) -> &'a State {
    let state = Floor;
    let mut local_i_index = i;
    let mut local_j_index = j + 1;

    while local_j_index < maximum_j - 1 && input[local_i_index][local_j_index] == Floor {
        local_j_index += 1;
    }
    if local_j_index < maximum_j {
        &input[local_i_index][local_j_index]
    } else {
        &Floor
    }
}

fn get_sw<'a>(
    input: &'a Vec<Vec<State>>,
    i: usize,
    j: usize,
    maximum_i: usize,
    maximum_j: usize,
) -> &'a State {
    let state = Floor;
    let mut local_i_index = i + 1;
    let mut local_j_index = j - 1;

    while local_i_index < maximum_i - 1
        && local_j_index > 0
        && input[local_i_index][local_j_index] == Floor
    {
        local_i_index += 1;
        local_j_index -= 1;
    }
    if local_i_index < maximum_i && local_j_index >= 0 {
        &input[local_i_index][local_j_index]
    } else {
        &Floor
    }
}

fn get_s<'a>(
    input: &'a Vec<Vec<State>>,
    i: usize,
    j: usize,
    maximum_i: usize,
    maximum_j: usize,
) -> &'a State {
    let state = Floor;
    let mut local_i_index = i + 1;
    let mut local_j_index = j;

    while local_i_index < maximum_i - 1 && input[local_i_index][local_j_index] == Floor {
        local_i_index += 1;
    }
    if local_i_index < maximum_i {
        &input[local_i_index][local_j_index]
    } else {
        &Floor
    }
}

fn get_se<'a>(
    input: &'a Vec<Vec<State>>,
    i: usize,
    j: usize,
    maximum_i: usize,
    maximum_j: usize,
) -> &'a State {
    let state = Floor;
    let mut local_i_index = i + 1;
    let mut local_j_index = j + 1;

    while local_i_index < maximum_i - 1
        && local_j_index < maximum_j - 1
        && input[local_i_index][local_j_index] == Floor
    {
        local_i_index += 1;
        local_j_index += 1;
    }
    if local_i_index < maximum_i && local_j_index < maximum_j {
        &input[local_i_index][local_j_index]
    } else {
        &Floor
    }
}

#[cfg(test)]
mod tests {
    use crate::day_11::{_day_11_1, _day_11_2};
    use crate::tooling::reader::*;

    #[test]
    fn solve_day_11_1() {
        let entries = read_file_to_vec_of_string("resources/day_11");
        let result = _day_11_1(entries);
        println!("Result: {}", result);
        assert_eq!(result, 2324);
    }

    #[test]
    fn solve_day_11_2() {
        let entries = read_file_to_vec_of_string("resources/day_11");
        let result = _day_11_2(entries);
        println!("Result: {}", result);
        assert_eq!(result, 2068);
    }
}

use crate::day_8::Operation::{Acc, Jump, Noop};
use itertools::iproduct;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{digit0, space0};
use nom::combinator::map;
use nom::sequence::{pair, separated_pair, tuple};
use nom::IResult;

#[derive(Debug)]
enum Operation {
    Acc(isize),
    Jump(isize),
    Noop(isize),
}

#[derive(Debug)]
struct OperationState {
    operation: Operation,
    visited: bool,
}

#[derive(Debug)]
struct State {
    content: Vec<OperationState>,
    accumulator: isize,
}

impl State {
    fn reset(&mut self) {
        self.content.iter_mut().for_each(|mut o| o.visited = false);
        self.accumulator = 0;
    }

    fn mutate(&mut self, index: usize) {
        let (index_to_change, operation_to_change) = self
            .content
            .iter()
            .enumerate()
            .filter(|(i, c)| match c.operation {
                Acc(_) => false,
                _ => true,
            })
            .nth(index)
            .unwrap();

        self.content[index_to_change] = OperationState {
            operation: match operation_to_change.operation {
                Jump(a) => Noop(a),
                Acc(a) => Acc(a),
                Noop(a) => Jump(a),
            },
            visited: false,
        };

        self.reset();
    }

    fn process1(&mut self) -> isize {
        let mut index = 0;
        while index < self.content.len() as isize && !&self.content[index as usize].visited {
            index = *&self.apply(&mut index);
        }
        self.accumulator
    }

    fn process2(&mut self) -> isize {
        let mut index = 0;
        while index < self.content.len() as isize && !&self.content[index as usize].visited {
            index = *&self.apply(&mut index);
        }

        if index >= self.content.len() as isize {
            self.accumulator
        } else {
            -1
        }
    }

    fn apply(&mut self, index: &mut isize) -> isize {
        match self.content[*index as usize].operation {
            Acc(digit) => {
                self.accumulator += digit;
                self.content[*index as usize].visited = true;
                *index + 1
            }
            Jump(digit) => {
                self.content[*index as usize].visited = true;
                *index + digit
            }
            Noop(_) => {
                self.content[*index as usize].visited = true;
                *index + 1
            }
        }
    }
}

fn _day_8_1(input: Vec<String>) -> isize {
    let mut state = State {
        content: input
            .iter()
            .map(|x| OperationState {
                operation: parse_line(&x).unwrap().1,
                visited: false,
            })
            .collect(),
        accumulator: 0,
    };

    state.process1()
}

fn _day_8_2(input: Vec<String>) -> isize {
    let mut state = State {
        content: input
            .iter()
            .map(|x| OperationState {
                operation: parse_line(&x).unwrap().1,
                visited: false,
            })
            .collect(),
        accumulator: 0,
    };

    let mut index = 0;
    while state.process2() == -1 {
        if index != 0 {
            state.mutate(index - 1);
        }
        state.mutate(index);
        index += 1;
    }
    state.reset();
    state.process2()
}

fn parse_line(i: &str) -> IResult<&str, Operation> {
    alt((
        map(
            separated_pair(tag("acc"), space0, parse_signed_isize),
            |(_, digit): (_, isize)| Acc(digit),
        ),
        map(
            separated_pair(tag("jmp"), space0, parse_signed_isize),
            |(_, digit): (_, isize)| Jump(digit),
        ),
        map(
            separated_pair(tag("nop"), space0, parse_signed_isize),
            |(_, digit): (_, isize)| Noop(digit),
        ),
    ))(i)
}

fn parse_signed_isize(i: &str) -> IResult<&str, isize> {
    alt((
        map(pair(tag("+"), digit0), |(_, digit): (_, &str)| {
            digit.to_string().parse::<isize>().unwrap()
        }),
        map(pair(tag("-"), digit0), |(_, digit): (_, &str)| {
            digit.to_string().parse::<isize>().unwrap() * -1isize
        }),
    ))(i)
}

#[cfg(test)]
mod tests {
    use crate::day_8::{_day_8_1, _day_8_2};
    use crate::tooling::reader::*;

    #[test]
    fn solve_day_8_1() {
        let entries = read_file_to_vec_of_string("resources/day_8");
        let result = _day_8_1(entries);
        println!("Result: {}", result);
        assert_eq!(result, 1584);
    }

    #[test]
    fn solve_day_8_2() {
        let entries = read_file_to_vec_of_string("resources/day_8");
        let result = _day_8_2(entries);
        println!("Result: {}", result);
        assert_eq!(result, 920);
    }
}

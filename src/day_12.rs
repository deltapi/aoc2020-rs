use crate::day_12::Direction::{E, N, S, W};
use crate::day_12::Instruction::*;
use itertools::{iproduct, Format};
use nom::character::complete::{alpha0, digit0};
use nom::combinator::map;
use nom::sequence::{pair, tuple};
use nom::IResult;

#[derive(Debug)]
enum Instruction {
    Forward(isize),
    Left(isize),
    Right(isize),
    North(isize),
    West(isize),
    South(isize),
    East(isize),
}

#[derive(Debug)]
enum Direction {
    N,
    W,
    E,
    S,
}

#[derive(Debug)]
struct ShipState {
    facing: Direction,
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct WaypointState {
    x: isize,
    y: isize,
}

fn _day_12_1(input: Vec<String>) -> usize {
    let instructions = input
        .iter()
        .map(|s| parse_input(s.to_string()))
        .collect::<Vec<Instruction>>();
    let mut ship_state = ShipState {
        facing: Direction::E,
        x: 0,
        y: 0,
    };

    instructions
        .iter()
        .for_each(|i| apply_to_state(&mut ship_state, i));

    (ship_state.x.abs() + ship_state.y.abs()) as usize
}

fn _day_12_2(input: Vec<String>) -> usize {
    let instructions = input
        .iter()
        .map(|s| parse_input(s.to_string()))
        .collect::<Vec<Instruction>>();
    let mut ship_state = ShipState {
        facing: Direction::E,
        x: 0,
        y: 0,
    };
    let mut waypoint_state = WaypointState { x: 10, y: 1 };

    instructions
        .iter()
        .for_each(|i| apply_to_state2(&mut ship_state, &mut waypoint_state, i));

    (ship_state.x.abs() + ship_state.y.abs()) as usize
}

fn parse_input(input: String) -> Instruction {
    let result: IResult<&str, (String, isize)> =
        map(pair(alpha0, digit0), |(a, b): (&str, &str)| {
            (a.to_string(), b.to_string().parse::<isize>().unwrap())
        })(input.as_str());
    let (remaining, (direction, amount)) = result.unwrap();
    match direction.as_str() {
        "F" => Forward(amount),
        "R" => Right(amount),
        "L" => Left(amount),
        "N" => North(amount),
        "W" => West(amount),
        "S" => South(amount),
        "E" => East(amount),
        _ => panic!("Whaaaaa!"),
    }
}

fn apply_to_state(state: &mut ShipState, instruction: &Instruction) {
    let currently_facing = &state.facing;
    match instruction {
        North(a) => state.y += *a,
        South(a) => state.y -= *a,
        East(a) => state.x += *a,
        West(a) => state.x -= *a,
        Forward(a) => match currently_facing {
            N => state.y += *a,
            S => state.y -= *a,
            E => state.x += *a,
            W => state.x -= *a,
        },
        Left(a) => match currently_facing {
            N => match a {
                90 => state.facing = W,
                180 => state.facing = S,
                270 => state.facing = E,
                360 | 0 | _ => (),
            },
            S => match a {
                90 => state.facing = E,
                180 => state.facing = N,
                270 => state.facing = W,
                360 | 0 | _ => (),
            },
            E => match a {
                90 => state.facing = N,
                180 => state.facing = W,
                270 => state.facing = S,
                360 | 0 | _ => (),
            },
            W => match a {
                90 => state.facing = S,
                180 => state.facing = E,
                270 => state.facing = N,
                360 | 0 | _ => (),
            },
        },
        Right(a) => match currently_facing {
            N => match a {
                90 => state.facing = E,
                180 => state.facing = S,
                270 => state.facing = W,
                360 | 0 | _ => (),
            },
            S => match a {
                90 => state.facing = W,
                180 => state.facing = N,
                270 => state.facing = E,
                360 | 0 | _ => (),
            },
            E => match a {
                90 => state.facing = S,
                180 => state.facing = W,
                270 => state.facing = N,
                360 | 0 | _ => (),
            },
            W => match a {
                90 => state.facing = N,
                180 => state.facing = E,
                270 => state.facing = S,
                360 | 0 | _ => (),
            },
        },
    };
}

fn apply_to_state2(
    ship_state: &mut ShipState,
    waypoint_state: &mut WaypointState,
    instruction: &Instruction,
) {
    match instruction {
        North(a) => waypoint_state.y += *a,
        South(a) => waypoint_state.y -= *a,
        East(a) => waypoint_state.x += *a,
        West(a) => waypoint_state.x -= *a,
        Forward(a) => {
            let x_diff = *a * (waypoint_state.x - ship_state.x);
            let y_diff = *a * (waypoint_state.y - ship_state.y);
            ship_state.y += y_diff;
            ship_state.x += x_diff;
            waypoint_state.y += y_diff;
            waypoint_state.x += x_diff;
        }
        Left(a) => match a {
            90 => {
                let x = -(waypoint_state.y - ship_state.y) + ship_state.x;
                let y = (waypoint_state.x - ship_state.x) + ship_state.y;
                waypoint_state.x = x;
                waypoint_state.y = y;
            }
            180 => {
                let x = -(waypoint_state.x - ship_state.x) + ship_state.x;
                let y = -(waypoint_state.y - ship_state.y) + ship_state.y;
                waypoint_state.x = x;
                waypoint_state.y = y;
            }
            270 => {
                let x = (waypoint_state.y - ship_state.y) + ship_state.x;
                let y = -(waypoint_state.x - ship_state.x) + ship_state.y;
                waypoint_state.x = x;
                waypoint_state.y = y;
            }
            360 | 0 | _ => panic!("wiiii"),
        },
        Right(a) => match a {
            270 => {
                let x = -(waypoint_state.y - ship_state.y) + ship_state.x;
                let y = (waypoint_state.x - ship_state.x) + ship_state.y;
                waypoint_state.x = x;
                waypoint_state.y = y;
            }
            180 => {
                let x = -(waypoint_state.x - ship_state.x) + ship_state.x;
                let y = -(waypoint_state.y - ship_state.y) + ship_state.y;
                waypoint_state.x = x;
                waypoint_state.y = y;
            }
            90 => {
                let x = (waypoint_state.y - ship_state.y) + ship_state.x;
                let y = -(waypoint_state.x - ship_state.x) + ship_state.y;
                waypoint_state.x = x;
                waypoint_state.y = y;
            }
            360 | 0 | _ => panic!("wiiii"),
        },
    };
}

#[cfg(test)]
mod tests {
    use crate::day_12::{_day_12_1, _day_12_2};
    use crate::tooling::reader::*;

    #[test]
    fn solve_day_12_1() {
        let entries = read_file_to_vec_of_string("resources/day_12");
        let result = _day_12_1(entries);
        println!("Result: {}", result);
        assert_eq!(result, 1956);
    }

    #[test]
    fn solve_day_12_2() {
        let entries = read_file_to_vec_of_string("resources/day_12");
        let result = _day_12_2(entries);
        println!("Result: {}", result);
        assert_eq!(result, 126797);
    }
}

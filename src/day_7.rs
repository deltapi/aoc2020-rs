use itertools::fold;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha0, digit0, space0};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{separated_pair, terminated, tuple};
use nom::{Err, IResult};
use petgraph::algo::has_path_connecting;
use petgraph::csr::NodeIndex;
use petgraph::graphmap::DiGraphMap;
use petgraph::prelude::DiGraph;
use petgraph::stable_graph::IndexType;
use petgraph::Graph;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Hash)]
struct Bag {
    extension: String,
    base: String,
}

impl Bag {
    fn fetch_rule<'a>(&self, rules: &'a Vec<Rule>) -> Option<&'a Rule> {
        rules
            .iter()
            .filter(|r| {
                r.source.extension == self.extension.to_string()
                    && r.source.base == self.base.to_string()
            })
            .filter(|r| r.targets.len() != 0)
            .find(|_| true)
    }

    fn is_ancestor_of(&self, rules: &Vec<Rule>, extension: &str, base: &str) -> bool {
        match self.fetch_rule(rules) {
            None => false,
            Some(rule) => rule
                .targets
                .iter()
                .find(|t| {
                    t.bag.extension == extension.to_string() && t.bag.base == base.to_string()
                        || t.bag.is_ancestor_of(rules, extension, base)
                })
                .is_some(),
        }
    }

    fn number_of_transitive_children(&self, rules: &Vec<Rule>) -> usize {
        match self.fetch_rule(rules) {
            None => 0,
            Some(rule) => rule
                .targets
                .iter()
                .map(|t| t.count * (t.bag.number_of_transitive_children(rules) + 1))
                .sum(),
        }
    }
}

#[derive(Debug, Clone)]
struct Bags {
    bag: Bag,
    count: usize,
}

#[derive(Debug, Clone)]
struct Rule {
    source: Bag,
    targets: Vec<Bags>,
}

fn _day_7_1(_input: Vec<String>) -> usize {
    let rules = _input
        .iter()
        .map(|x| _parse_rule(&x).unwrap().1)
        .collect::<Vec<Rule>>();

    rules
        .iter()
        .filter(|r| r.source.is_ancestor_of(&rules, "shiny", "gold"))
        .count()
}

fn _day_7_2(_input: Vec<String>) -> usize {
    let rules = _input
        .iter()
        .map(|x| _parse_rule(&x).unwrap().1)
        .collect::<Vec<Rule>>();

    Bag {
        extension: "shiny".to_string(),
        base: "gold".to_string(),
    }
    .number_of_transitive_children(&rules)
}

fn _parse_rule(input: &str) -> nom::IResult<&str, Rule> {
    let x = match nom::combinator::all_consuming(nom::sequence::tuple((
        parse_color,
        space0,
        tag("bags contain"),
        space0,
        terminated(separated_list0(tag(", "), parse_child), tag(".")),
    )))(input)
    {
        Ok((remaining_input, ((extension, base), _, _, _, bags))) => Ok((
            remaining_input,
            Rule {
                source: Bag {
                    extension: extension.to_string(),
                    base: base.to_string(),
                },
                targets: bags
                    .iter()
                    .filter(|b| b.is_some())
                    .map(|b| b.as_ref().unwrap().clone())
                    .collect(),
            },
        )),
        Err(e) => Err(e),
    };
    println!("{:?}", x);
    x
}

fn parse_child(i: &str) -> IResult<&str, Option<Bags>> {
    alt((
        map(tag("no other bags"), |_| None),
        map(
            tuple((separated_pair(digit0, space0, parse_color), space0, alpha0)),
            |((count, (extension, base)), _, _)| {
                Some(Bags {
                    bag: Bag {
                        extension: extension.to_string(),
                        base: base.to_string(),
                    },
                    count: count.to_string().parse::<usize>().unwrap(),
                })
            },
        ),
    ))(i)
}

fn parse_color(i: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha0, space0, alpha0)(i)
}

#[cfg(test)]
mod tests {
    use crate::day_7::{_day_7_1, _day_7_2};
    use crate::tooling::reader::*;

    #[test]
    fn solve_day_7_1() {
        let entries = read_file_to_vec_of_string("resources/day_7");
        println!("{}", _day_7_1(entries));
    }

    #[test]
    fn solve_day_7_2() {
        let entries = read_file_to_vec_of_string("resources/day_7");
        println!("{}", _day_7_2(entries));
    }
}

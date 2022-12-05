use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{self, BufRead},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Item(char);

impl From<char> for Item {
    fn from(c: char) -> Self {
        Self(c)
    }
}

impl Item {
    pub fn priority(&self) -> u32 {
        let offset = if self.0.is_uppercase() { 38 } else { 96 };

        self.0 as u32 - offset
    }
}

struct Compartment {
    pub items: Vec<Item>,
}

impl Compartment {

    pub fn as_set(&self) -> HashSet<Item> {
        self.items.iter().copied().collect()
    }

    pub fn intersection(&self, o: &Self) -> HashSet<Item> {
        let a = self.as_set();
        let b = o.as_set();

        a.intersection(&b).copied().collect()
    }

    pub fn union(&self, o: &Self) -> HashSet<Item> {
        let a = self.as_set();
        let b = o.as_set();

        a.union(&b).copied().collect()
    }
}

struct Rucksack {
    first: Compartment,
    second: Compartment,
}

impl Rucksack {
    pub fn intersection(&self) -> HashSet<Item> {
        self.first.intersection(&self.second)
    }

    pub fn union(&self) -> HashSet<Item> {
        self.first.union(&self.second)
    }
}

#[derive(Debug)]
struct ParseError(pub String);

impl FromStr for Compartment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            items: s.chars().map(Into::into).collect(),
        })
    }
}

impl FromStr for Rucksack {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            first: s[..s.len() / 2].parse()?,
            second: s[s.len() / 2..].parse()?,
        })
    }
}

fn main() {
    let file = File::open("input.txt").expect("unable to open input.txt");
    let sacks: Vec<Rucksack> = io::BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok()?.parse().ok())
        .collect();

    let sum: u32 = sacks.iter()
        .filter_map(|s| Some(s.intersection().iter().next()?.priority()))
        .sum();

    let tripplet_sum: u32 = sacks.into_iter()
        .tuples()
        .map(|(a, b, c)| (a.union(), b.union(), c.union()))
        .filter_map(|(a, b, c)| a.intersection(&b).copied().collect::<HashSet<_>>().intersection(&c).copied().next())
        .map(|item| item.priority())
        .sum();
    println!("Sum {}, tripplet sum {}", sum, tripplet_sum);
}
